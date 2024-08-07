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
}