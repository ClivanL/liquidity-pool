use anchor_lang::prelude::*;
use crate::utils::*;
use crate::context::*;
use anchor_spl::token::{MintTo};
use anchor_spl::token;

pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> Result<()> {
    
    // Transfer tokens from user to vault
    token::transfer(ctx.accounts.into_transfer_to_vault_a_context(), amount_a)?;
    token::transfer(ctx.accounts.into_transfer_to_vault_b_context(), amount_b)?;
    token::transfer(ctx.accounts.into_transfer_to_vault_c_context(), amount_c)?;
    token::transfer(ctx.accounts.into_transfer_to_vault_d_context(), amount_d)?;
    token::transfer(ctx.accounts.into_transfer_to_vault_e_context(), amount_e)?;
    
    //msg!("up to here");
    let lp_amount = match calculate_lp_amount(amount_a, amount_b, amount_c, amount_d, amount_e) {
        Ok(sum) => {
            println!("The sum is: {}", sum);
            sum
        },
        Err(e) => {
            println!("Error: {}", e);
            0
        },
    };

    assert!(lp_amount!=0, "Value should not be zero");
    token::mint_to(
        CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            authority: ctx.accounts.lp_mint.to_account_info(),
            to: ctx.accounts.user_lp_account.to_account_info(),
            mint: ctx.accounts.lp_mint.to_account_info()
        },
        &[&[
            "lp_mint".as_bytes(),
            &[ctx.bumps.lp_mint]
        ]]
    ), lp_amount)?;
    let liquidity_pool = &mut *ctx.accounts.liquidity_pool;

    // Update liquidity pool state
    liquidity_pool.total_lp_supply += lp_amount as f64;

    Ok(())
}