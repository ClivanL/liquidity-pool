use anchor_lang::prelude::*;
use crate::context::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use crate::errors::*;
use crate::utils::*;

pub fn handler(ctx: Context<StakeTokensV2>, stake_amount:f64) -> Result<()> {

    //check exchange rate
    let user_token_account_copy = ctx.accounts.user_token_account.clone();
    let token_name_str = String::from_utf8(user_token_account_copy.token_name.clone()).map_err(|_| CustomError::InvalidUtf8)?;
    let token_name:&str = &token_name_str;
    let exchange_rate = check_exchange_rate_v2(&ctx,token_name)?;

    let user_token_account = &mut ctx.accounts.user_token_account;

    //Ensure transaction caller is the owner of the account
    if user_token_account.user!=*ctx.accounts.user.key{
        msg!("Transaction initiation not done by account owner.");
        return Err(CustomError::InvalidOwner.into());
    }

    // check if balance is sufficient for deduction of stated amount
    if user_token_account.balance<stake_amount{
        return Err(CustomError::InsufficientBalance.into());
    }

    // deduct from token account
    user_token_account.balance-=stake_amount;

    // increment token account pending stake balance
    user_token_account.pending_stake+=stake_amount;



    let lp_token_to_receive = stake_amount*exchange_rate;
    if lp_token_to_receive.is_nan() || lp_token_to_receive.is_infinite(){
        return Err(CustomError::InvalidValue.into());
    }
    
    //calculate the balance to return to token account due to inability to complete minting of whole coin
    let lp_token_to_receive_rounded_down = lp_token_to_receive.floor();
    let refund = (lp_token_to_receive-lp_token_to_receive_rounded_down)/exchange_rate;
    user_token_account.balance+=refund;

    //update user token account pending stake balance
    user_token_account.pending_stake-=refund;

    //update resulting stake_balance
    let stake_balance = stake_amount-refund;

    let stake_token_transaction = &mut ctx.accounts.stake_token_transaction;
    stake_token_transaction.stake_amount = stake_balance;
    stake_token_transaction.tokens_to_mint = lp_token_to_receive_rounded_down as u64;
    stake_token_transaction.exchange_rate = exchange_rate;
    stake_token_transaction.token_name = token_name.to_string().into();
    stake_token_transaction.user_pubkey = ctx.accounts.user.key();

    let pending_stake_seed_records = &mut ctx.accounts.pending_stake_seed_records;
    
    let last_index = pending_stake_seed_records.last_index;

    {
    let sub_seeds = &mut pending_stake_seed_records.sub_seeds;
    let new_seed = format!("s{}", last_index);
    sub_seeds.push(new_seed);
    pending_stake_seed_records.last_index+=1;
    }
    
    Ok(())
}