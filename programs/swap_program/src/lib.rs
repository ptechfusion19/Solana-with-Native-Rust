#![no_std]

extern crate alloc;
use alloc::format;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction as token_instruction;

// Simple custom error codes
#[repr(u32)]
pub enum SwapError {
    InvalidInstruction = 0,
    NotRentExempt = 1,
    OwnerMismatch = 2,
    Unauthorized = 3,
    MathOverflow = 4,
}

impl From<SwapError> for ProgramError {
    fn from(e: SwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Pool state stored in pool_state_account.data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolState {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub vault_x: Pubkey,
    pub vault_y: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub bump: u8,
}

impl PoolState {
    pub const LEN: usize = 1 + 32 * 5 + 1;
}

// Instruction tags:
// 0 = InitializePool
// 1 = Deposit { amount: u64 }
// 2 = AdminRebalance { action: u8, amount: u64 }
entrypoint!(process_instruction);
pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    if input.is_empty() {
        return Err(SwapError::InvalidInstruction.into());
    }

    match input[0] {
        0 => process_initialize(program_id, accounts),
        1 => {
            if input.len() < 1 + 8 {
                return Err(SwapError::InvalidInstruction.into());
            }
            let amount = u64::from_le_bytes(
                input[1..9]
                    .try_into()
                    .map_err(|_| SwapError::InvalidInstruction)?,
            );
            process_deposit(program_id, accounts, amount)
        }
        2 => {
            if input.len() < 1 + 1 + 8 {
                return Err(SwapError::InvalidInstruction.into());
            }
            let action = input[1];
            let amount = u64::from_le_bytes(
                input[2..10]
                    .try_into()
                    .map_err(|_| SwapError::InvalidInstruction)?,
            );
            process_admin_rebalance(program_id, accounts, action, amount)
        }
        _ => Err(SwapError::InvalidInstruction.into()),
    }
}

// InitializePool accounts (order expected by program):
// 0. [signer] payer (optional)
// 1. [writable] pool_state_account (must be owned by program and allocated with PoolState::LEN)
// 2. [] admin_account (admin pubkey)
// 3. [writable] vault_x_token_account (SPL token account for mint_x)
// 4. [writable] vault_y_token_account (SPL token account for mint_y)
// 5. [] mint_x
// 6. [] mint_y
// 7. [] token_program
fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let ai = &mut accounts.iter();

    let _payer = next_account_info(ai)?;
    let pool_state_ai = next_account_info(ai)?;
    let admin_ai = next_account_info(ai)?;
    let vault_x_ai = next_account_info(ai)?;
    let vault_y_ai = next_account_info(ai)?;
    let mint_x_ai = next_account_info(ai)?;
    let mint_y_ai = next_account_info(ai)?;
    let _token_program_ai = next_account_info(ai)?;

    // pool_state must be owned by the program (client should create it)
    if pool_state_ai.owner != program_id {
        msg!("pool_state account not owned by program");
        return Err(SwapError::OwnerMismatch.into());
    }

    // Validate token accounts are owned by token program
    if *vault_x_ai.owner != spl_token::id() || *vault_y_ai.owner != spl_token::id() {
        msg!("vault accounts not owned by token program");
        return Err(SwapError::OwnerMismatch.into());
    }

    // Check data length
    if pool_state_ai.data_len() < PoolState::LEN {
        msg!(
            "Pool state account too small: {}, want {}",
            pool_state_ai.data_len(),
            PoolState::LEN
        );
        return Err(SwapError::InvalidInstruction.into());
    }

    let (_pda, bump) =
        Pubkey::find_program_address(&[b"pool", pool_state_ai.key.as_ref()], program_id);

    let state = PoolState {
        is_initialized: true,
        admin: *admin_ai.key,
        vault_x: *vault_x_ai.key,
        vault_y: *vault_y_ai.key,
        mint_x: *mint_x_ai.key,
        mint_y: *mint_y_ai.key,
        bump,
    };

    state.serialize(&mut &mut pool_state_ai.data.borrow_mut()[..])?;

    msg!(
        "Pool initialized. pool_state: {} bump: {}",
        pool_state_ai.key,
        bump
    );

    Ok(())
}

// Deposit accounts order expected:
// 0. [signer] user_authority
// 1. [writable] user_token_x (source)
// 2. [writable] pool_vault_x (destination)
// 3. [writable] pool_vault_y (source for payout)
// 4. [writable] user_token_y (destination for payout)
// 5. [writable] pool_state_account
// 6. [] token_program
// 7. [] pool_authority (PDA, readonly; required so token program sees authority AccountInfo)
fn process_deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let ai = &mut accounts.iter();

    let user_authority_ai = next_account_info(ai)?;
    let user_token_x_ai = next_account_info(ai)?;
    let pool_vault_x_ai = next_account_info(ai)?;
    let pool_vault_y_ai = next_account_info(ai)?;
    let user_token_y_ai = next_account_info(ai)?;
    let pool_state_ai = next_account_info(ai)?;
    let token_program_ai = next_account_info(ai)?;
    let pool_authority_ai = next_account_info(ai)?;

    // validate pool_state owner
    if pool_state_ai.owner != program_id {
        msg!("pool_state incorrect owner");
        return Err(SwapError::OwnerMismatch.into());
    }

    // parse state
    let state = PoolState::try_from_slice(&pool_state_ai.data.borrow())?;
    if !state.is_initialized {
        msg!("pool not initialized");
        return Err(SwapError::InvalidInstruction.into());
    }

    // validate token account owners are token program
    if *user_token_x_ai.owner != spl_token::id() || *pool_vault_x_ai.owner != spl_token::id() {
        msg!("token account owner mismatch for X accounts");
        return Err(SwapError::OwnerMismatch.into());
    }
    if *pool_vault_y_ai.owner != spl_token::id() || *user_token_y_ai.owner != spl_token::id() {
        msg!("token account owner mismatch for Y accounts");
        return Err(SwapError::OwnerMismatch.into());
    }

    // 1) Transfer Token X: user_token_x -> pool_vault_x (user_authority must sign)
    let ix1 = token_instruction::transfer(
        &spl_token::id(),
        user_token_x_ai.key,
        pool_vault_x_ai.key,
        user_authority_ai.key,
        &[],
        amount,
    )?;
    invoke(
        &ix1,
        &[
            user_token_x_ai.clone(),
            pool_vault_x_ai.clone(),
            user_authority_ai.clone(),
            token_program_ai.clone(),
        ],
    )?;

    // 2) Transfer Token Y: pool_vault_y -> user_token_y (signed by PDA)
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[b"pool", pool_state_ai.key.as_ref()], program_id);

    if *pool_authority_ai.key != pda {
        msg!("pool authority account mismatch; expected PDA: {}", pda);
        return Err(SwapError::OwnerMismatch.into());
    }

    if *pool_vault_y_ai.key != state.vault_y || *pool_vault_x_ai.key != state.vault_x {
        msg!("vault accounts mismatch with state");
        return Err(SwapError::InvalidInstruction.into());
    }

    let ix2 = token_instruction::transfer(
        &spl_token::id(),
        pool_vault_y_ai.key,
        user_token_y_ai.key,
        pool_authority_ai.key,
        &[],
        amount, // 1:1 payout for simplicity
    )?;

    invoke_signed(
        &ix2,
        &[
            pool_vault_y_ai.clone(),
            user_token_y_ai.clone(),
            pool_authority_ai.clone(),
            token_program_ai.clone(),
        ],
        &[&[b"pool", pool_state_ai.key.as_ref(), &[bump_seed]]],
    )?;

    // Emit machine-parseable event log (no tx sig on-chain)
    msg!(
        "EVENT:Deposit|user={} |amount={}",
        user_authority_ai.key,
        amount
    );

    Ok(())
}

// AdminRebalance accounts:
// 0. [signer] admin_authority
// 1. [writable] admin_token_y
// 2. [writable] pool_vault_y
// 3. [writable] pool_state_account
// 4. [] token_program
// 5. [] pool_authority (only required for withdraw action)
//
// action: 0 = top-up (admin -> pool_vault_y)
// action: 1 = withdraw (pool_vault_y -> admin) (requires PDA authority account)
fn process_admin_rebalance(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    action: u8,
    amount: u64,
) -> ProgramResult {
    let ai = &mut accounts.iter();

    let admin_ai = next_account_info(ai)?;
    let admin_token_y_ai = next_account_info(ai)?;
    let pool_vault_y_ai = next_account_info(ai)?;
    let pool_state_ai = next_account_info(ai)?;
    let token_program_ai = next_account_info(ai)?;
    let pool_authority_ai_opt = ai.next(); // optional

    if !admin_ai.is_signer {
        msg!("admin must be signer");
        return Err(SwapError::Unauthorized.into());
    }

    if pool_state_ai.owner != program_id {
        msg!("pool_state incorrect owner");
        return Err(SwapError::OwnerMismatch.into());
    }

    let state = PoolState::try_from_slice(&pool_state_ai.data.borrow())?;
    if state.admin != *admin_ai.key {
        msg!("admin pubkey mismatch");
        return Err(SwapError::Unauthorized.into());
    }

    if action == 0u8 {
        // admin -> pool_vault_y
        let ix = token_instruction::transfer(
            &spl_token::id(),
            admin_token_y_ai.key,
            pool_vault_y_ai.key,
            admin_ai.key,
            &[],
            amount,
        )?;
        invoke(
            &ix,
            &[
                admin_token_y_ai.clone(),
                pool_vault_y_ai.clone(),
                admin_ai.clone(),
                token_program_ai.clone(),
            ],
        )?;
        msg!(
            "EVENT:AdminRebalance|action=topup|admin={} |amount={}",
            admin_ai.key,
            amount
        );
        return Ok(());
    }

    if action == 1u8 {
        let pool_authority_ai = match pool_authority_ai_opt {
            Some(ai) => ai,
            None => {
                msg!("pool authority account required for withdraw");
                return Err(SwapError::InvalidInstruction.into());
            }
        };

        let (pda, bump_seed) =
            Pubkey::find_program_address(&[b"pool", pool_state_ai.key.as_ref()], program_id);

        if *pool_authority_ai.key != pda {
            msg!("pool authority mismatch");
            return Err(SwapError::OwnerMismatch.into());
        }

        let ix = token_instruction::transfer(
            &spl_token::id(),
            pool_vault_y_ai.key,
            admin_token_y_ai.key,
            pool_authority_ai.key,
            &[],
            amount,
        )?;

        invoke_signed(
            &ix,
            &[
                pool_vault_y_ai.clone(),
                admin_token_y_ai.clone(),
                pool_authority_ai.clone(),
                token_program_ai.clone(),
            ],
            &[&[b"pool", pool_state_ai.key.as_ref(), &[bump_seed]]],
        )?;

        msg!(
            "EVENT:AdminRebalance|action=withdraw|admin={} |amount={}",
            admin_ai.key,
            amount
        );
        return Ok(());
    }

    Err(SwapError::InvalidInstruction.into())
}
