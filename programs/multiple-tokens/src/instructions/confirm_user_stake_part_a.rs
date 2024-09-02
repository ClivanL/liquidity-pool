use anchor_lang::prelude::*;
use crate::context::*;
use solana_program::pubkey::Pubkey;
use crate::state::*;
use crate::errors::*;
use anchor_spl::token;
use anchor_spl::token::{MintTo};
use solana_program::system_instruction::create_account;
use solana_program::program::invoke_signed;
use crate::instructions::*;
use solana_program::stake_history::Epoch;


pub fn handler<'info>(ctx: Context<'info,'info,'_,'info,ConfirmUserStakePartA<'info>>) -> Result<()> {
    let sub_seeds = &mut ctx.accounts.pending_stake_seed_records.sub_seeds;
    for sub_seed in sub_seeds.iter() {
        let seed_byte = sub_seed.as_bytes();
        let seeds = &[b"pending_stake", seed_byte];
        let (stake_token_transaction_hold_pda, bump) = Pubkey::find_program_address(seeds, &ctx.program_id);
        let stake_token_transaction_pda = &mut ctx.accounts.stake_token_transaction_pda;
        stake_token_transaction_pda.pubkey = stake_token_transaction_hold_pda;
        // Convert PDA to AccountInfo
        let stake_token_transaction_info = AccountInfo::new(
            &stake_token_transaction_pda.pubkey,
            false, // is_signer
            true,  // is_writable
            &mut 0, // lamports
            &mut [], // data
            &ctx.program_id, // owner
            false, // executable
            Epoch::default() // rent_epoch
        );
        let stake_token_transaction_account:Account<'info, StakeTokenTransaction> = Account::try_from(&stake_token_transaction_info)?;

        let bumps = ConfirmUserStakePartBBumps {
            stake_token_transaction: bump, 
            lp_mint: ctx.bumps.lp_mint,
            // token_lp_vault: *ctx.bumps.token_lp_vault.unwrap(),
            // stake_records: *ctx.bumps.stake_records.unwrap(),
            // liquidity_pool: *ctx.bumps.liquidity_pool.unwrap(),
            // initializer: *ctx.bumps.initializer.unwrap(),
        };

        let mut cpi_accounts = ConfirmUserStakePartB {
            system_program: ctx.accounts.system_program.clone(),
            stake_token_transaction: stake_token_transaction_account.clone(),
            lp_mint: ctx.accounts.lp_mint.clone(),
            token_lp_vault: ctx.accounts.token_lp_vault.clone(),
            stake_records: ctx.accounts.stake_records.clone(),
            liquidity_pool: ctx.accounts.liquidity_pool.clone(),
            initializer: ctx.accounts.initializer.clone(),
        };

        let cpi_ctx = Context::new(
            ctx.program_id,
            &mut cpi_accounts,
            &[],
            bumps,   
        );

        confirm_user_stake_part_b::handler(cpi_ctx,sub_seed.to_string());
        //confirm_user_stake_part_b::handler(ctx.accounts.system_program,stake_token_transaction_account, ctx.accounts.lp_mint,ctx.accounts.token_lp_vault,ctx.accounts.stake_records,ctx.accounts.liquidity_pool,ctx.accounts.initializer,sub_seed.to_string());
    }  
    Ok(())
}