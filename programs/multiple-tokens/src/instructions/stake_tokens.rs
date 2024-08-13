use anchor_lang::prelude::*;
use crate::context::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use crate::errors::*;
use crate::utils::*;

pub fn handler(ctx: Context<StakeTokens>, stake_amount:f64) -> Result<()> {

    //check exchange rate
    let user_token_account_copy = ctx.accounts.user_token_account.clone();
    let token_name_str = String::from_utf8(user_token_account_copy.token_name.clone()).map_err(|_| CustomError::InvalidUtf8)?;
    let token_name:&str = &token_name_str;
    let exchange_rate = check_exchange_rate(&ctx,token_name)?;

    let user_token_account = &mut ctx.accounts.user_token_account;

    // check if balance is sufficient for deduction of stated amount
    if user_token_account.balance<stake_amount{
        return Err(CustomError::InsufficientBalance.into());
    }

    // deduct from token account
    user_token_account.balance-=stake_amount;

    let lp_token_to_receive = stake_amount*exchange_rate;
    if lp_token_to_receive.is_nan() || lp_token_to_receive.is_infinite(){
        return Err(CustomError::InvalidValue.into());
    }
    
    //calculate the balance to return to token account due to inability to complete minting of whole coin
    let lp_token_to_receive_rounded_down = lp_token_to_receive.floor();
    let refund = (lp_token_to_receive-lp_token_to_receive_rounded_down)/exchange_rate;
    user_token_account.balance+=refund;

    //update resulting stake_balance
    let stake_balance = stake_amount-refund;
    
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
    ), lp_token_to_receive_rounded_down as u64)?; 

    // update total minted lp token supply in liquidity pool
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    liquidity_pool.total_lp_supply+=lp_token_to_receive_rounded_down;

    // update staked records 
    let stake_records = &mut ctx.accounts.stake_records;
    match token_name {
        "token_a" => {
            let new_balance = stake_records.token_a_stake+stake_balance;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
            }
            stake_records.token_a_stake = new_balance;
        },
        "token_b" => {
            let new_balance = stake_records.token_b_stake+stake_balance;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
            }
            stake_records.token_b_stake = new_balance;
        },
        "token_c" => {
            let new_balance = stake_records.token_c_stake+stake_balance;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
            }
            stake_records.token_c_stake = new_balance;
        },
        "token_d" => {
            let new_balance = stake_records.token_d_stake+stake_balance;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
            }
            stake_records.token_d_stake = new_balance;
        },
        "token_e" => {
            let new_balance = stake_records.token_e_stake+stake_balance;
            if new_balance.is_nan() || new_balance.is_infinite(){
                return Err(CustomError::InvalidValue.into());
            }
            stake_records.token_e_stake = new_balance;
        },
        _=>{
            return Err(CustomError::InvalidTokenName.into());
        }
    }

    // update user_lp_token_account
    let user_lp_token_account = &mut ctx.accounts.user_lp_token_account;
    let new_balance = user_lp_token_account.balance+lp_token_to_receive_rounded_down;
    if new_balance.is_nan() || new_balance.is_infinite(){
        return Err(CustomError::InvalidValue.into());
    }
    user_lp_token_account.balance = new_balance;

    Ok(())
}