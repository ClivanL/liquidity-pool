use anchor_lang::prelude::*;
use crate::context::*;
use crate::enums::*;
use core::str::FromStr;

pub fn handler(ctx: Context<CreateOrderBook>, token_pair:String, direction:String) -> Result<()> {
    let order_book = &mut ctx.accounts.order_book;
    order_book.last_index = 0;
    order_book.token_pair = TokenPair::from_str(&token_pair)?;
    order_book.direction = Direction::from_str(&direction)?;
    Ok(())
}