use anchor_lang::prelude::*;

#[derive(Debug)]
pub enum ProgramError {
    InvalidArgument,
    OverflowError,
}

#[error_code]
pub enum CustomError {
    #[msg("The liquidity pool is not initialized.")]
    LiquidityPoolNotInitialized,
    #[msg("The token name does not exist.")]
    InvalidTokenName,
    #[msg("Insufficient Balance in account. Please top up.")]
    InsufficientBalance,
    #[msg("Overflow.")]
    Overflow,
    #[msg("Addition overflowed.")]
    AdditionOverflow,
    #[msg("Invalid UTF-8 sequence.")]
    InvalidUtf8,
    #[msg("Cannot retrieve value from feed.")]
    FeedError,
    #[msg("An error occurred with the Switchboard Oracle.")]
    SwitchboardError,
    #[msg("Mathematical operation resulted in an invalid value")]
    InvalidValue,
    #[msg("Transaction initiator is not the owner.")]
    InvalidOwner
}


impl From<switchboard_solana::Error> for CustomError {
    fn from(_error: switchboard_solana::Error) -> Self {
        CustomError::SwitchboardError
    }
}