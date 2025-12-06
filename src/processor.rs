use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::VaultError;
use crate::instruction::VaultInstruction;
use crate::state::VaultState;

/// Seeds prefix for vault PDAs
const VAULT_SEED_PREFIX: &[u8] = b"vault";

pub struct Processor {}
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VaultInstruction::unpack(instruction_data)?;
        match instruction {
            VaultInstruction::Deposit { amount } => {
                msg!("Instruction: Deposit");
                Self::process_deposit(program_id, accounts, amount)
            }
            VaultInstruction::Withdraw { amount } => {
                msg!("Instruction: Withdraw");
                Self::process_withdraw(program_id, accounts, amount)
            }
        }
    }

    /// Accounts for Deposit:
    /// 0. [signer, writable] depositor (payer)
    /// 1. [writable] vault_pda_account (PDA)
    /// 2. [] system_program
    /// 3. [] rent_sysvar
    pub fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let depositor_info = next_account_info(account_info_iter)?; // signer, payer
        let vault_info = next_account_info(account_info_iter)?; // PDA
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;

        // Basic checks
        if !depositor_info.is_signer {
            msg!("Error: depositor must be signer");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !vault_info.is_writable {
            msg!("Error: vault account must be writable");
            return Err(ProgramError::InvalidAccountData);
        }

        // Derive expected PDA
        let (expected_pda, bump) = Pubkey::find_program_address(
            &[VAULT_SEED_PREFIX, depositor_info.key.as_ref()],
            program_id,
        );
        if expected_pda != *vault_info.key {
            msg!("PDA mismatch (deposit)");
            return Err(ProgramError::InvalidArgument);
        }

        // If uninitialized (owner not program_id), create account and initialize
        if vault_info.owner != program_id {
            msg!("Creating PDA account (not owned by program)");

            let rent = &Rent::from_account_info(rent_info)?;
            let required_lamports = rent.minimum_balance(VaultState::size());
            let lamports_to_fund = required_lamports
                .checked_add(amount)
                .ok_or(ProgramError::InsufficientFunds)?;

            let create_ix = system_instruction::create_account(
                depositor_info.key,
                vault_info.key,
                lamports_to_fund,
                VaultState::size() as u64,
                program_id,
            );

            let seeds: &[&[u8]] = &[VAULT_SEED_PREFIX, depositor_info.key.as_ref(), &[bump]];
            invoke_signed(
                &create_ix,
                &[
                    depositor_info.clone(),
                    vault_info.clone(),
                    system_program_info.clone(),
                ],
                &[seeds],
            )?;

            // Initialize vault state struct
            let vault_state = VaultState {
                depositor: *depositor_info.key,
                bump,
                initialized: true,
            };

            {
                let mut data = vault_info.try_borrow_mut_data()?;
                vault_state.pack(&mut data)?;
            }

            msg!("Vault created and initialized");
            return Ok(());
        } else {
            // Already exists: transfer amount from depositor to vault
            if amount == 0 {
                msg!("Deposit amount must be > 0 for existing vault");
                return Err(ProgramError::InvalidArgument);
            }

            let transfer_ix =
                system_instruction::transfer(depositor_info.key, vault_info.key, amount);
            invoke(
                &transfer_ix,
                &[
                    depositor_info.clone(),
                    vault_info.clone(),
                    system_program_info.clone(),
                ],
            )?;

            msg!("Deposit transferred");
            return Ok(());
        }
    }

    /// Accounts for Withdraw:
    /// 0. [signer, writable] depositor (requester)
    /// 1. [writable] vault_pda_account (PDA) - must be owned by this program
    /// 2. [] system_program
    /// 3. [] rent_sysvar
    pub fn process_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        requested_amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let depositor_info = next_account_info(account_info_iter)?;
        let vault_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;

        // signer required
        if !depositor_info.is_signer {
            msg!("depositor must be signer");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !vault_info.is_writable {
            msg!("vault must be writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if vault_info.owner != program_id {
            msg!("vault owner mismatch");
            return Err(ProgramError::IllegalOwner);
        }

        // Read state and verify depositor
        let state = {
            let data = vault_info.try_borrow_data()?;
            VaultState::unpack(&data)?
        };

        if !state.initialized {
            msg!("Vault not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if state.depositor != *depositor_info.key {
            msg!("Unauthorized: only original depositor can withdraw");
            return Err(VaultError::Unauthorized.into());
        }

        let rent = Rent::from_account_info(rent_info)?;
        let rent_exempt_min = rent.minimum_balance(VaultState::size());

        let vault_balance = **vault_info.lamports.borrow();

        // amount available = vault_balance - rent_exempt_min
        let available = vault_balance
            .checked_sub(rent_exempt_min)
            .ok_or(ProgramError::InsufficientFunds)?;

        let amount_to_withdraw = if requested_amount == 0 {
            available
        } else {
            if requested_amount > available {
                msg!("Requested amount exceeds available");
                return Err(ProgramError::InsufficientFunds);
            }
            requested_amount
        };

        if amount_to_withdraw == 0 {
            msg!("Nothing available to withdraw (preserve rent)");
            return Err(ProgramError::InsufficientFunds);
        }

        let (expected_pda, bump) = Pubkey::find_program_address(
            &[VAULT_SEED_PREFIX, depositor_info.key.as_ref()],
            program_id,
        );
        if expected_pda != *vault_info.key {
            msg!("PDA mismatch on withdraw");
            return Err(ProgramError::InvalidArgument);
        }

        let transfer_ix =
            system_instruction::transfer(vault_info.key, depositor_info.key, amount_to_withdraw);

        let seeds: &[&[u8]] = &[VAULT_SEED_PREFIX, depositor_info.key.as_ref(), &[bump]];

        invoke_signed(
            &transfer_ix,
            &[
                vault_info.clone(),
                depositor_info.clone(),
                system_program_info.clone(),
            ],
            &[seeds],
        )?;

        msg!("Withdrawn to depositor");
        Ok(())
    }
}
