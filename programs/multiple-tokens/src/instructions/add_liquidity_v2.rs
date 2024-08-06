use anchor_lang::prelude::*;
use crate::utils::*;
use crate::context::*;
use anchor_spl::token::{MintTo};
use anchor_spl::token;

pub fn handler(ctx: Context<AddLiquidityV2>, amount:u64) -> Result<()> {
    
    // Transfer tokens from user to vault
    token::transfer(ctx.accounts.into_transfer_to_vault_context(), amount)?;
    
    let user_token_account = &mut ctx.accounts.user_token_account;
    user_token_account.balance += amount;

    Ok(())
}