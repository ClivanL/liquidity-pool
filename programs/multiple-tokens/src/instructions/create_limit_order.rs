use anchor_lang::prelude::*;
use crate::context::*;
use crate::enums::*;
use core::str::FromStr;
use crate::errors::*;
use crate::constants::*;
use crate::state::*;
use chrono::Utc;

pub fn handler(ctx: Context<CreateLimitOrder>,direction:String, sub_seed:String, token_pair:String, quantity:f64, exchange_rate:f64) -> Result<()> {

    let order_book = &mut ctx.accounts.order_book;
    //CHECKS
    //check that direction and token_pair matches order book
    if order_book.direction!=Direction::from_str(&direction)?{
        return Err(CustomError::MismatchDirection.into());
    }
    if order_book.token_pair!=TokenPair::from_str(&token_pair)?{
        return Err(CustomError::MismatchTokenPair.into());
    }
    //check that sub_seed of limit order does not exist in any of the existing orders
    for order in &order_book.orders{
        if order.sub_seed == sub_seed{
            return Err(CustomError::SubSeedInUse.into());
        }
    }
    //check that existing length of order book does not exceed limit
    if order_book.orders.len() as i32==MAX_ORDERS{
        return Err(CustomError::OrderBookCapacityReached.into());
    }

    let user_token_account = &mut ctx.accounts.user_token_account;
    //check that user_token_account user matches user account
    if user_token_account.user!=*ctx.accounts.user.key{
        return Err(CustomError::InvalidOwner.into());
    }
    //check that user_token_account token type matches the from for token_pair
    if String::from_utf8(user_token_account.token_name.clone()).map_err(|_| CustomError::InvalidUtf8)?!=format!("token_{}", &token_pair[0..1]){
        return Err(CustomError::MismatchTokenName.into());
    }
    //check that user_token_account has sufficient balance
    if user_token_account.balance < quantity{
        return Err(CustomError::InsufficientBalance.into());
    }

    //deduct quantity from user_token_account
    user_token_account.balance-=quantity;
    let limit_order = &mut ctx.accounts.limit_order;
    limit_order.user = *ctx.accounts.user.key;
    limit_order.amount_to_trade = quantity;
    limit_order.exchange_rate = exchange_rate;
    let now = Utc::now();
    limit_order.created_at = now.timestamp_millis();
    limit_order.token_pair = TokenPair::from_str(&token_pair)?;
    limit_order.closed = false;
    limit_order.direction = Direction::from_str(&direction)?;
    limit_order.sub_seed = sub_seed.clone();
    //add limit order to order book, sort order book by closed, exchange rate, created_at
    let limit_order_new = LimitOrder{
        user:*ctx.accounts.user.key,
        amount_to_trade:quantity,
        exchange_rate:exchange_rate,
        created_at:now.timestamp_millis(),
        token_pair:TokenPair::from_str(&token_pair)?,
        closed:false,
        direction:Direction::from_str(&direction)?,
        sub_seed:sub_seed
    };
    order_book.orders.push(limit_order_new);

    Ok(())
}