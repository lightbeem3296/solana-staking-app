use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Invalid signer
    #[error("Invalid Signer")]
    InvalidSigner,
    /// Invalid owner
    #[error("Invalid Owner")]
    InvalidOwner,
    /// Account already initialized
    #[error("Account already initialized")]
    AlreadyInitialized,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
