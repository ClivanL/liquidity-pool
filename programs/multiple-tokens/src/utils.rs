use std::result::*;
use crate::errors::*;
use crate::context::*;
use anchor_lang::prelude::{Context,msg};

pub fn calculate_lp_amount(amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> Result<u64,CustomError> {
    let sum = amount_a
    .checked_add(amount_b).ok_or(CustomError::Overflow)?
    .checked_add(amount_c).ok_or(CustomError::Overflow)?
    .checked_add(amount_d).ok_or(CustomError::Overflow)?
    .checked_add(amount_e).ok_or(CustomError::Overflow)?;
    Ok(sum)

    // if sum.is_nan() || sum.is_infinite(){
    //     return Err(CustomError::InvalidValue);
    // }
    // else{
    //     Ok(sum)
    // }
    

}

pub fn check_valid_token_name(token_name: &str) -> Result<(),CustomError> {
    match token_name {
        "token_a"|"token_b" |"token_c" |"token_d" |"token_e" => Ok(()),
        _ => Err(CustomError::InvalidTokenName)
    }
}


pub fn check_exchange_rate(ctx: &Context<StakeTokens>, token_name:&str) ->  Result<f64,CustomError> {
    match token_name{
        "token_a" => {
            let feed_a = &mut ctx.accounts.feed_aggregator_a.load()?;
            let feed_a_data:f64 =  feed_a.get_result()?.try_into()?;
            msg!("price token a: {:?}", feed_a_data);
            Ok(feed_a_data)
        },
        "token_b" => {
            let feed_b = &mut ctx.accounts.feed_aggregator_b.load()?;
            let feed_b_data:f64 =  feed_b.get_result()?.try_into()?;
            msg!("price token b: {:?}", feed_b_data);
            Ok(feed_b_data)
        },
        "token_c" => { 
            let feed_c = &mut ctx.accounts.feed_aggregator_c.load()?;
            let feed_c_data:f64 =  feed_c.get_result()?.try_into()?;
            msg!("price token c: {:?}", feed_c_data);
            Ok(feed_c_data)
        },
        "token_d" => {
            let feed_d = &mut ctx.accounts.feed_aggregator_d.load()?;
            let feed_d_data:f64 =  feed_d.get_result()?.try_into()?;
            msg!("price token d: {:?}", feed_d_data);
            Ok(feed_d_data)
        },
        "token_e" => {
            let feed_e = &mut ctx.accounts.feed_aggregator_e.load()?;
            let feed_e_data:f64 =  feed_e.get_result()?.try_into()?;
            msg!("price token a: {:?}", feed_e_data);
            Ok(feed_e_data)
        },
        _ => Err(CustomError::InvalidTokenName)
    }
}