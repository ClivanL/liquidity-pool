use anchor_lang::prelude::*;
use crate::context::*;

pub fn handler(ctx: Context<CreateLiquidityPool>) -> Result<()> {
    let liquidity_pool = &mut *ctx.accounts.liquidity_pool;

    // Initialize token vaults pubkey in liquidity pool
    liquidity_pool.token_a_vault = Pubkey::default();
    liquidity_pool.token_b_vault = Pubkey::default();
    liquidity_pool.token_c_vault = Pubkey::default();
    liquidity_pool.token_d_vault = Pubkey::default();
    liquidity_pool.token_e_vault = Pubkey::default();
    liquidity_pool.lp_mint = (*ctx.accounts.lp_mint).key();
    liquidity_pool.total_lp_supply = 0.0; // Initialize the total supply

    Ok(())
}