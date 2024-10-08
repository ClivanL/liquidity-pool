pub mod context;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod constants;
pub mod config;
pub mod enums;

use anchor_lang::prelude::*;

use instructions::*;
use context::*;

declare_id!("GqwLRbiFFcAujKPm4NchYfLRhJ6ztTtaPyMADamd6e3");

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

    pub fn init_pending_stake_seed_records(ctx: Context<InitPendingStakeSeedRecords>) -> Result<()>{
        init_pending_stake_seed_records::handler(ctx)
    }

    pub fn stake_tokens_v2(ctx: Context<StakeTokensV2>, sub_seed:String, stake_amount:f64) -> Result<()>{
        stake_tokens_v2::handler(ctx,sub_seed,stake_amount)
    }

    pub fn confirm_user_stake(ctx:Context<ConfirmUserStake>) -> Result<()>{
        confirm_user_stake::handler(ctx)
    }

    // pub fn confirm_user_stake_part_a(ctx:Context<ConfirmUserStakePartA>) -> Result<()> {
    //     confirm_user_stake_part_a::handler(ctx)
    // }
    pub fn create_order_book(ctx:Context<CreateOrderBook>,token_pair:String, direction:String, order_book_subseed:String) -> Result<()> {
        create_order_book::handler(ctx, token_pair, direction, order_book_subseed)
    }

    pub fn create_limit_order(ctx: Context<CreateLimitOrder>,direction:String, sub_seed:String, token_pair:String, order_book_subseed:String, quantity:f64, exchange_rate:f64) -> Result<()> {
        create_limit_order::handler(ctx,direction,sub_seed,token_pair,order_book_subseed,quantity,exchange_rate)
    }

    pub fn create_order_book_directory(ctx:Context<CreateOrderBookDirectory>,token_pair:String, direction:String) -> Result<()> {
        create_order_book_directory::handler(ctx, token_pair, direction)
    }

    pub fn create_pending_transfers_record(ctx:Context<CreatePendingTransfersRecord>) -> Result<()> {
        create_pending_transfers_record::handler(ctx)
    }

    pub fn process_buy_limit_order(ctx:Context<ProcessBuyLimitOrder>, pending_transfers_subseed:String) -> Result<()> {
        process_buy_limit_order::handler(ctx,pending_transfers_subseed)
    }
}



