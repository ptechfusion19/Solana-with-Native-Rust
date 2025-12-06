use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// VaultState layout:
/// [0..32]   -> depositor Pubkey (32 bytes)
/// [32]      -> bump (1 byte)
/// [33]      -> initialized flag (1 byte) 0=false, 1=true
pub const VAULT_STATE_LEN: usize = 32 + 1 + 1;

pub struct VaultState {
    pub depositor: Pubkey,
    pub bump: u8,
    pub initialized: bool,
}

impl VaultState {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < VAULT_STATE_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let depositor = Pubkey::new_from_array(
            input[0..32]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        );
        let bump = input[32];
        let initialized = input[33] != 0;
        Ok(Self {
            depositor,
            bump,
            initialized,
        })
    }

    pub fn pack(&self, output: &mut [u8]) -> Result<(), ProgramError> {
        if output.len() < VAULT_STATE_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        output[0..32].copy_from_slice(self.depositor.as_ref());
        output[32] = self.bump;
        output[33] = if self.initialized { 1 } else { 0 };
        Ok(())
    }

    pub fn size() -> usize {
        VAULT_STATE_LEN
    }
}

