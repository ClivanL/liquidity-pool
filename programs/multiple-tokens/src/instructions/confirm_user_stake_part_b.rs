use anchor_lang::prelude::*;
use crate::context::*;
use solana_program::pubkey::Pubkey;
use crate::state::*;
use crate::errors::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use solana_program::system_instruction::create_account;
use solana_program::program::invoke_signed;


pub fn handler<'info>(ctx: Context<'_,'_,'_,'info,ConfirmUserStakePartB<'info>>,_sub_seed:String) -> Result<()> {

    let stake_token_transaction = &mut ctx.accounts.stake_token_transaction;
    //msg!(stake_token_transaction.token_name);
    msg!(&stake_token_transaction.user_pubkey.to_string());

    Ok(())

}