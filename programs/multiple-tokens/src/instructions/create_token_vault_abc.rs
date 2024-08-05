use anchor_lang::prelude::*;
use crate::context::*;
use crate::errors::*;

pub fn handler(ctx: Context<CreateTokenVaultABC>) -> Result<()> {
    let liquidity_pool = &mut *ctx.accounts.liquidity_pool;
    
    // Check if the liquidity pool is initialized
    if !liquidity_pool.is_initialized() {
        return Err(CustomError::LiquidityPoolNotInitialized.into());
    }
    // Update the vault public keys in the LiquidityPool struct
    liquidity_pool.token_a_vault = (*ctx.accounts.token_a_vault).key();
    liquidity_pool.token_b_vault = (*ctx.accounts.token_b_vault).key();
    liquidity_pool.token_c_vault = (*ctx.accounts.token_c_vault).key();

    Ok(())
}