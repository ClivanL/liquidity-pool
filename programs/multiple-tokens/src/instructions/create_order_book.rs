use anchor_lang::prelude::*;
use crate::context::*;
use crate::enums::*;
use core::str::FromStr;
use crate::errors::CustomError;

pub fn handler(ctx: Context<CreateOrderBook>, token_pair:String, direction:String, order_book_subseed:String) -> Result<()> {
    let order_book_directory = &mut ctx.accounts.order_book_directory;
    if format!("OB{}",order_book_directory.last_index)!=order_book_subseed{
        return Err(CustomError::SubSeedInUse.into());
    }
    let order_book = &mut ctx.accounts.order_book;
    order_book.last_index = 0;
    order_book.token_pair = TokenPair::from_str(&token_pair)?;
    order_book.direction = Direction::from_str(&direction)?;
    order_book.orders = Vec::new();
    order_book_directory.last_index+=1;
    order_book_directory.orderbook_subseeds.push(order_book_subseed);
    Ok(())
}