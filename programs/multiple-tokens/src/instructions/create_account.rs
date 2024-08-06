use anchor_lang::prelude::*;
use crate::context::*;
use crate::utils::*;

pub fn handler(ctx: Context<CreateAccount>,token_name:String) -> Result<()> {
    check_valid_token_name(&token_name)?;
    let user_token_account = &mut ctx.accounts.user_token_account;

    user_token_account.user_token_vault = ctx.accounts.user_token_vault.key();
    user_token_account.user = ctx.accounts.user.key();
    user_token_account.balance = 0;

    Ok(())
}