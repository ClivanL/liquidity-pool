use anchor_lang::prelude::*;
use crate::context::*;
use crate::enums::*;
use core::str::FromStr;

pub fn handler(ctx: Context<CreateOrderBookDirectory>, token_pair:String, direction:String) -> Result<()> {
    let order_book_directory = &mut ctx.accounts.order_book_directory;
    order_book_directory.last_index = 0;
    order_book_directory.token_pair = TokenPair::from_str(&token_pair)?;
    order_book_directory.direction = Direction::from_str(&direction)?;
    order_book_directory.orderbook_subseeds = Vec::new();
    Ok(())
}