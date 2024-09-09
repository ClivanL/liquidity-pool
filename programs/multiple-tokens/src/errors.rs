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
    InvalidOwner,
    #[msg("Wrong account retrieved.")]
    WrongAccountRetrieval,
    #[msg("The token pair does not exist.")]
    InvalidTokenPair,
    #[msg("The direction does not exist.")]
    WrongDirectionInput,
    #[msg("Different direction between order and order book.")]
    MismatchDirection,
    #[msg("Different token pair between order and order book.")]
    MismatchTokenPair,
    #[msg("SubSeed is in use.")]
    SubSeedInUse,
    #[msg("Order book is at max capacity.")]
    OrderBookCapacityReached,
    #[msg("Mismatch in token name.")]
    MismatchTokenName
}


impl From<switchboard_solana::Error> for CustomError {
    fn from(_error: switchboard_solana::Error) -> Self {
        CustomError::SwitchboardError
    }
}