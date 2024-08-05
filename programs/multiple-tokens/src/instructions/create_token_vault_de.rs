use anchor_lang::prelude::*;
use crate::context::*;
use crate::errors::*;

pub fn handler(ctx: Context<CreateTokenVaultDE>) -> Result<()> {
    let liquidity_pool = &mut *ctx.accounts.liquidity_pool;

    // Check if the liquidity pool is initialized
    if !liquidity_pool.is_initialized() {
        return Err(CustomError::LiquidityPoolNotInitialized.into());
    }

    // Update the vault public keys in the LiquidityPool struct
    liquidity_pool.token_d_vault = (*ctx.accounts.token_d_vault).key();
    liquidity_pool.token_e_vault = (*ctx.accounts.token_e_vault).key();

    Ok(())
}