use solana_program::program_error::ProgramError;

/// Instruction enum:
/// 0 = Deposit { amount: u64 }
/// 1 = Withdraw { amount: u64 }
pub enum VaultInstruction {
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

impl VaultInstruction {
    /// Unpack instruction bytes.
    /// Format: [tag: u8][amount: u64 LE]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < 9 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let tag = input[0];
        let amount_bytes: [u8; 8] = input[1..9]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let amount = u64::from_le_bytes(amount_bytes);
        match tag {
            0 => Ok(VaultInstruction::Deposit { amount }),
            1 => Ok(VaultInstruction::Withdraw { amount }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
