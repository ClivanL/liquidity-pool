use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, TokenAccount, Mint, Token};
use crate::state::*;
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(init, payer = initializer,seeds = ["liquidity_pool".as_bytes()], bump, space = 8 + LiquidityPool::INIT_SPACE)]
    pub liquidity_pool: Box<Account<'info, LiquidityPool>>,
    #[account(init, payer = initializer,seeds = ["lp_mint".as_bytes()], bump, mint::decimals = 9, mint::authority = lp_mint)]
    pub lp_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct CreateTokenVaultABC<'info> {
    #[account(mut)]
    pub liquidity_pool: Box<Account<'info, LiquidityPool>>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = token_a_mint, associated_token::authority = liquidity_pool)]
    pub token_a_vault: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = token_b_mint, associated_token::authority = liquidity_pool)]
    pub token_b_vault: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = token_c_mint, associated_token::authority = liquidity_pool)]
    pub token_c_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_a_mint: Box<Account<'info, Mint>>,
    pub token_b_mint: Box<Account<'info, Mint>>,
    pub token_c_mint: Box<Account<'info, Mint>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct CreateTokenVaultDE<'info> {
    #[account(mut)]
    pub liquidity_pool: Box<Account<'info, LiquidityPool>>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = token_d_mint, associated_token::authority = liquidity_pool)]
    pub token_d_vault: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = token_e_mint, associated_token::authority = liquidity_pool)]
    pub token_e_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_d_mint: Box<Account<'info, Mint>>,
    pub token_e_mint: Box<Account<'info, Mint>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub liquidity_pool: Box<Account<'info, LiquidityPool>>,
    #[account(mut)]
    pub user_token_a: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_b: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_c: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_d: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_e: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_a_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_b_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_c_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_d_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_e_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
    pub lp_mint: Box<Account<'info, Mint>>,
    #[account(init_if_needed, payer = user, associated_token::mint = lp_mint, associated_token::authority = user)]
    pub user_lp_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> AddLiquidity<'info> {
    pub fn into_transfer_to_vault_a_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_a.to_account_info(),
                to: self.token_a_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn into_transfer_to_vault_b_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_b.to_account_info(),
                to: self.token_b_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn into_transfer_to_vault_c_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_c.to_account_info(),
                to: self.token_c_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn into_transfer_to_vault_d_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_d.to_account_info(),
                to: self.token_d_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn into_transfer_to_vault_e_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_e.to_account_info(),
                to: self.token_e_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    // fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
    //     let seeds: &[&[u8]] = &[
    //     b"lp_mint",
    //     ];
    //     CpiContext::new_with_signer(
    //         self.token_program.to_account_info(),
    //         MintTo {
    //             mint: self.lp_mint.to_account_info(),
    //             to: self.user_lp_account.to_account_info(),
    //             authority: self.lp_mint.to_account_info(),
    //         },
    //         &[&[
    //             b"lp_mint",
    //             ]]
    //     )
    // }
}