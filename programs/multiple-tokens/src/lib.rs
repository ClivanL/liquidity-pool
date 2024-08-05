pub mod context;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod constants;

use anchor_lang::prelude::*;

use instructions::*;
use context::*;

declare_id!("EDBwJ2TUonePxXiA7C46VEucdw7LQE6GfDnytkauBJ6f");

#[program]
pub mod multiple_tokens {
    use super::*;

    pub fn create_liquidity_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        create_liquidity_pool::handler(ctx)
    }

    pub fn create_token_vault_abc(ctx: Context<CreateTokenVaultABC>) -> Result<()> {
        create_token_vault_abc::handler(ctx)
    }

    pub fn create_token_vault_de(ctx: Context<CreateTokenVaultDE>) -> Result<()> {
        create_token_vault_de::handler(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> Result<()> {
        add_liquidity::handler(ctx, amount_a, amount_b, amount_c, amount_d, amount_e)
    }
}



