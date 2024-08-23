use anchor_lang::prelude::*;
use crate::context::*;
use solana_program::pubkey::Pubkey;
use crate::state::*;
use crate::errors::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use solana_program::system_instruction::create_account;
use solana_program::program::invoke_signed;


pub fn handler(ctx: Context<ConfirmUserStake>) -> Result<()> {

    let stake_token_transaction = &mut ctx.accounts.stake_token_transaction;
    let user_lp_token_account = &mut ctx.accounts.user_lp_token_account;
    let user_token_account = &mut ctx.accounts.user_token_account;

    if user_token_account.token_name!=stake_token_transaction.token_name{  
        return Err(CustomError::WrongAccountRetrieval.into());
    }
    if user_token_account.user!=stake_token_transaction.user_pubkey{  
        return Err(CustomError::WrongAccountRetrieval.into());
    }

            // mint to vault for lp_token
        token::mint_to(
            CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.token_lp_vault.to_account_info(),
                mint: ctx.accounts.lp_mint.to_account_info()
            },
            &[&[
                "lp_mint".as_bytes(),
                &[ctx.bumps.lp_mint]
            ]]
        ), stake_token_transaction.tokens_to_mint)?; 

    // update total minted lp token supply in liquidity pool
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    liquidity_pool.total_lp_supply+=stake_token_transaction.tokens_to_mint as f64;

    user_lp_token_account.token_name = "lp_token".into();
    user_lp_token_account.balance += stake_token_transaction.tokens_to_mint as f64;
    user_lp_token_account.user = stake_token_transaction.user_pubkey;

    let staked_tokens = user_token_account.pending_stake;
    user_token_account.pending_stake = 0.0;

    // update staked records 
    let stake_records = &mut ctx.accounts.stake_records;
    let token_name = std::str::from_utf8(&stake_token_transaction.token_name)
    .map_err(|_| CustomError::InvalidUtf8)?;
    match token_name {
        "token_a" => {
            let new_balance = stake_records.token_a_stake+staked_tokens;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
                }
            stake_records.token_a_stake = new_balance;
            },
        "token_b" => {
            let new_balance = stake_records.token_b_stake+staked_tokens;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
                }
            stake_records.token_b_stake = new_balance;
            },
        "token_c" => {
            let new_balance = stake_records.token_c_stake+staked_tokens;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
                }
            stake_records.token_c_stake = new_balance;
            },
        "token_d" => {
            let new_balance = stake_records.token_d_stake+staked_tokens;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
                }
            stake_records.token_d_stake = new_balance;
            },
        "token_e" => {
            let new_balance = stake_records.token_e_stake+staked_tokens;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
                }
            stake_records.token_e_stake = new_balance;
            },
        _=>{
            return Err(CustomError::InvalidTokenName.into());
            }   
        }
    Ok(())

}