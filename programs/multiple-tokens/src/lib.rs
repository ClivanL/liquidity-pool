pub mod context;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod constants;
pub mod config;

use anchor_lang::prelude::*;

use instructions::*;
use context::*;

declare_id!("HpcdH7e76Dep273mkjbyJqGB5QE3cPAXCB8LmDnwj3Hd");

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
    pub fn add_liquidity_v2(ctx: Context<AddLiquidityV2>, amount: u64) -> Result<()> {
        add_liquidity_v2::handler(ctx, amount)
    }

    pub fn create_account(ctx: Context<CreateAccount>, token_name:String) -> Result<()>{
        create_account::handler(ctx,token_name)
    }

    pub fn init_stake_records(ctx: Context<InitStakeRecords>) -> Result<()>{
        init_stake_records::handler(ctx)
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>,stake_amount:f64) -> Result<()>{
        stake_tokens::handler(ctx,stake_amount)
    }

    pub fn create_lp_token_vault(ctx: Context<CreateLpTokenVault>) -> Result<()>{
        create_lp_token_vault::handler(ctx)
    }

}



