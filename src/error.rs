use solana_program::program_error::ProgramError;

/// Custom error codes for the Vault program.
/// Convert to ProgramError::Custom(u32) via From.
#[repr(u32)]
pub enum VaultError {
    InvalidInstruction = 0,
    Unauthorized = 1,
    NotRentExempt = 2,
    AlreadyInitialized = 3,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> ProgramError {
        ProgramError::Custom(e as u32)
    }
}
