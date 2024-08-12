use anchor_lang::prelude::*;
use crate::context::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use crate::errors::*;
use crate::utils::*;
use switchboard_solana::{AggregatorAccountData, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID, JobAccountData};

pub fn handler(ctx: Context<StakeTokens>, stake_amount:u64) -> Result<()> {
    let user_token_account = &mut ctx.accounts.user_token_account;

    // check if balance is sufficient for deduction of stated amount
    if user_token_account.balance<stake_amount{
        return Err(CustomError::InsufficientBalance.into());
    }

    // deduct from token account
    user_token_account.balance-=stake_amount;

    let token_name_str = String::from_utf8(user_token_account.token_name.clone()).map_err(|_| CustomError::InvalidUtf8)?;;
    let token_name:&str = &token_name_str;

    // Iterate over job public keys and their corresponding accounts
        // for (i, job_pubkey) in feed.job_pubkeys_data.iter().enumerate() {
        //     if job_pubkey != &Pubkey::default() {
    
        //         // Load the job data
        //         let job_data: JobAccountData = JobAccountData::try_from_slice(job_pubkey)?;
    
        //         // Extract the job result or other relevant information
        //         msg!("Job {} result: {:?}", i + 1, job_data.data);
        //     }
        // }
    
    //let feed_account = ctx.accounts.feed.data.borrow();
    //let feed = PullFeedAccountData::parse(feed_account).unwrap();

    //msg!("price: {:?}", feed_data[0]);

    let exchange_rate = check_exchange_rate(&ctx,token_name)?;
    let lp_token_to_receive = stake_amount.checked_mul(exchange_rate).ok_or(CustomError::Overflow)?;
    
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
    ), lp_token_to_receive)?; 

    // update total minted lp token supply in liquidity pool
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    liquidity_pool.total_lp_supply+=lp_token_to_receive;

    // update staked records 
    let stake_records = &mut ctx.accounts.stake_records;
    match token_name {
        "token_a" => {
            stake_records.token_a_stake = stake_records.token_a_stake.checked_add(stake_amount).ok_or(CustomError::AdditionOverflow)?;
        },
        "token_b" => {
            stake_records.token_b_stake = stake_records.token_b_stake.checked_add(stake_amount).ok_or(CustomError::AdditionOverflow)?;
        },
        "token_c" => {
            stake_records.token_c_stake = stake_records.token_c_stake.checked_add(stake_amount).ok_or(CustomError::AdditionOverflow)?;
        },
        "token_d" => {
            stake_records.token_d_stake = stake_records.token_d_stake.checked_add(stake_amount).ok_or(CustomError::AdditionOverflow)?;
        },
        "token_e" => {
            stake_records.token_e_stake = stake_records.token_e_stake.checked_add(stake_amount).ok_or(CustomError::AdditionOverflow)?;
        },
        _=>{
            return Err(CustomError::InvalidTokenName.into());
        }
    }

    // update user_lp_token_account
    let user_lp_token_account = &mut ctx.accounts.user_lp_token_account;
    user_lp_token_account.balance.checked_add(lp_token_to_receive).ok_or(CustomError::AdditionOverflow)?;

    Ok(())
}