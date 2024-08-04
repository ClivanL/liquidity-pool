use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, MintTo, TokenAccount, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;
mod constants;
use constants::*;

declare_id!("EDBwJ2TUonePxXiA7C46VEucdw7LQE6GfDnytkauBJ6f");

#[program]
pub mod multiple_tokens {
    use super::*;

    pub fn create_liquidity_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        let liquidity_pool = &mut *ctx.accounts.liquidity_pool;
    
        // Initialize token vaults pubkey in liquidity pool
        liquidity_pool.token_a_vault = Pubkey::default();
        liquidity_pool.token_b_vault = Pubkey::default();
        liquidity_pool.token_c_vault = Pubkey::default();
        liquidity_pool.token_d_vault = Pubkey::default();
        liquidity_pool.token_e_vault = Pubkey::default();
        liquidity_pool.lp_mint = (*ctx.accounts.lp_mint).key();
        liquidity_pool.total_lp_supply = 0; // Initialize the total supply
    
        Ok(())
    }

    pub fn create_token_vault_abc(ctx: Context<CreateTokenVaultABC>) -> Result<()> {
        let liquidity_pool = &mut *ctx.accounts.liquidity_pool;
        
        // Check if the liquidity pool is initialized
        if !liquidity_pool.is_initialized() {
            return Err(CustomError::LiquidityPoolNotInitialized.into());
        }
        // Update the vault public keys in the LiquidityPool struct
        liquidity_pool.token_a_vault = (*ctx.accounts.token_a_vault).key();
        liquidity_pool.token_b_vault = (*ctx.accounts.token_b_vault).key();
        liquidity_pool.token_c_vault = (*ctx.accounts.token_c_vault).key();
    
        Ok(())
    }

    pub fn create_token_vault_de(ctx: Context<CreateTokenVaultDE>) -> Result<()> {
        let liquidity_pool = &mut *ctx.accounts.liquidity_pool;

        // Check if the liquidity pool is initialized
        if !liquidity_pool.is_initialized() {
            return Err(CustomError::LiquidityPoolNotInitialized.into());
        }

        // Update the vault public keys in the LiquidityPool struct
        liquidity_pool.token_d_vault = (*ctx.accounts.token_d_vault).key();
        liquidity_pool.token_e_vault = (*ctx.accounts.token_e_vault).key();
    
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> Result<()> {
    
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
        liquidity_pool.total_lp_supply += lp_amount;
    
        Ok(())
    }
}


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

#[account]
pub struct LiquidityPool {
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,    
    pub token_c_vault: Pubkey,
    pub token_d_vault: Pubkey,
    pub token_e_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub total_lp_supply: u64
}

impl LiquidityPool {
    const INIT_SPACE:usize = PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+U64_SIZE;

    pub fn is_initialized(&self) -> bool {
        if self.lp_mint != Pubkey::default(){
            true
        }
        else{
            false
        }
    }
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
    fn into_transfer_to_vault_a_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_a.to_account_info(),
                to: self.token_a_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_to_vault_b_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_b.to_account_info(),
                to: self.token_b_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_to_vault_c_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_c.to_account_info(),
                to: self.token_c_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_to_vault_d_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_d.to_account_info(),
                to: self.token_d_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_to_vault_e_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
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

fn calculate_lp_amount(amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> std::result::Result<u64,String> {
    let sum = amount_a
        .checked_add(amount_b)
        .ok_or("Overflow occurred during addition of amount_a and amount_b")?
        .checked_add(amount_c)
        .ok_or("Overflow occurred during addition of sum and amount_c")?
        .checked_add(amount_d)
        .ok_or("Overflow occurred during addition of sum and amount_d")?
        .checked_add(amount_e)
        .ok_or("Overflow occurred during addition of sum and amount_e")?;

    Ok(sum)

}


#[derive(Debug)]
pub enum ProgramError {
    InvalidArgument,
    OverflowError,
}

#[error_code]
pub enum CustomError {
    #[msg("The liquidity pool is not initialized.")]
    LiquidityPoolNotInitialized,
}