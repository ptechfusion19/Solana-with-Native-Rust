#![no_std]

extern crate alloc;
use alloc::format;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

mod error;
mod instruction;
mod processor;
mod state;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("solana_vault: entrypoint");
    processor::Processor::process(program_id, accounts, instruction_data)
}
