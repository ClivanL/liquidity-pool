use std::result::*;
use crate::errors::*;
use crate::context::*;
use anchor_lang::prelude::*;
use switchboard_solana::{AggregatorAccountData, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID, JobAccountData};

pub fn calculate_lp_amount(amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> std::result::Result<u64,String> {
    let sum = amount_a
        .checked_add(amount_b)
        .ok_or("Overflow occurred during addition of amount_a and amount_b")?
        .checked_add(amount_c)
        .ok_or("Overflow occurred during addition of sum and amount_c")?
        .checked_add(amount_d)
        .ok_or("Overflow occurred during addition of sum and amount_d")?
        .checked_add(amount_e)
        .ok_or("Overflow occurred during addition of sum and amount_e")?;

    Ok(sum)

}

pub fn check_valid_token_name(token_name: &str) -> std::result::Result<(),CustomError> {
    match token_name {
        "token_a"|"token_b" |"token_c" |"token_d" |"token_e" => Ok(()),
        _ => Err(CustomError::InvalidTokenName)
    }
}


pub fn check_exchange_rate(ctx: &Context<StakeTokens>, token_name:&str) ->  std::result::Result<u64,CustomError> {

    let feed_a = &mut ctx.accounts.feed_aggregator_a.load()?;
    let feed_a_data:f64 =  feed_a.get_result()?.try_into().map_err(|_| CustomError::FeedError)?;
    msg!("price token a: {:?}", feed_a_data);

    let feed_b = &mut ctx.accounts.feed_aggregator_b.load()?;
    let feed_b_data:f64 =  feed_b.get_result()?.try_into().map_err(|_| CustomError::FeedError)?;
    msg!("price token b: {:?}", feed_b_data);

    let feed_c = &mut ctx.accounts.feed_aggregator_c.load()?;
    let feed_c_data:f64 =  feed_c.get_result()?.try_into().map_err(|_| CustomError::FeedError)?;
    msg!("price token c: {:?}", feed_c_data);

    let feed_d = &mut ctx.accounts.feed_aggregator_d.load()?;
    let feed_d_data:f64 =  feed_d.get_result()?.try_into().map_err(|_| CustomError::FeedError)?;
    msg!("price token d: {:?}", feed_d_data);

    let feed_e = &mut ctx.accounts.feed_aggregator_e.load()?;
    let feed_e_data:f64 =  feed_e.get_result()?.try_into().map_err(|_| CustomError::FeedError)?;
    msg!("price token a: {:?}", feed_e_data);

    match token_name{
        "token_a" => Ok(1),
        "token_b" => Ok(2),
        "token_c" => Ok(3),
        "token_d" => Ok(4),
        "token_e" => Ok(5),
        _ => Err(CustomError::InvalidTokenName)
    }
}