use anchor_lang::prelude::*;
use crate::context::*;
use crate::errors::*;

pub fn handler(ctx: Context<CreateLpTokenVault>) -> Result<()> {
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    
    // Check if the liquidity pool is initialized
    if !liquidity_pool.is_initialized() {
        return Err(CustomError::LiquidityPoolNotInitialized.into());
    }
    Ok(())
}