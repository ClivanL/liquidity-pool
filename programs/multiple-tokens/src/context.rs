use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, TokenAccount, Mint, Token};
use crate::state::*;
use anchor_spl::associated_token::AssociatedToken;
use crate::config::*;
use switchboard_solana::{AggregatorAccountData};
use std::str::FromStr;


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
pub struct CreateLpTokenVault<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(init_if_needed, payer = initializer, associated_token::mint = lp_mint, associated_token::authority = liquidity_pool)]
    pub token_lp_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    pub user_token_vault: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = user, seeds = [&token_name.as_bytes(), user.key().as_ref()], bump, space = 8+UserAccount::INIT_SPACE)]
    pub user_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//method requires user to have all 5 token accounts transfer all 5 tokens at once
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

#[derive(Accounts)]
pub struct AddLiquidityV2<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> AddLiquidityV2<'info> {
    pub fn into_transfer_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token.to_account_info(),
                to: self.token_vault.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}

#[derive(Accounts)]
pub struct InitStakeRecords<'info> {
    #[account(init, payer=initializer, seeds=[b"stake_records"],bump, space = 8+StakeRecords::INIT_SPACE)]
    pub stake_records: Account<'info, StakeRecords>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub stake_records: Account<'info, StakeRecords>,
    #[account(mut)]
    pub token_lp_vault: Account<'info, TokenAccount>,
    #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
    pub lp_mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = user, seeds=["lp_token".as_bytes(),user.key().as_ref()],bump, space = 8+UserAccount::INIT_SPACE)]
    pub user_lp_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: This field is safe because this takes in public key for switchboard pull_feed
    // #[account(mut)]
    // pub feed: AccountInfo<'info>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_XRPBEARUSDT).unwrap()
    )]
    pub feed_aggregator_a: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_BTCUSD).unwrap()
    )]
    pub feed_aggregator_b: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_ETHUSDT).unwrap()
    )]
    pub feed_aggregator_c: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_BULLBUSD).unwrap()
    )]
    pub feed_aggregator_d: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_EOSBEARBUSD).unwrap()
    )]
    pub feed_aggregator_e: AccountLoader<'info, AggregatorAccountData>,
}

#[derive(Accounts)]
pub struct InitPendingStakeSeedRecords<'info>{
    #[account(init, payer=initializer, seeds=["master_seed".as_bytes()], bump, space=8+PendingStakeSeedRecords::INIT_SPACE)]
    pub pending_stake_seed_records: Account<'info,PendingStakeSeedRecords>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sub_seed:String)]
pub struct StakeTokensV2<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub pending_stake_seed_records: Account<'info,PendingStakeSeedRecords>,
    #[account(init, payer=user, seeds=["pending_stake".as_bytes(), sub_seed.to_string().as_ref()],bump, space=8+StakeTokenTransaction::INIT_SPACE)]
    pub stake_token_transaction: Account<'info,StakeTokenTransaction>,
    #[account(init_if_needed, payer = user, seeds=["lp_token".as_bytes(),user.key().as_ref()],bump, space = 8+UserAccount::INIT_SPACE)]
    pub user_lp_token_account: Account<'info, UserAccount>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_XRPBEARUSDT).unwrap()
    )]
    pub feed_aggregator_a: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_BTCUSD).unwrap()
    )]
    pub feed_aggregator_b: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_ETHUSDT).unwrap()
    )]
    pub feed_aggregator_c: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_BULLBUSD).unwrap()
    )]
    pub feed_aggregator_d: AccountLoader<'info, AggregatorAccountData>,
    #[account(
        address = Pubkey::from_str(FEED_ADDRESS_EOSBEARBUSD).unwrap()
    )]
    pub feed_aggregator_e: AccountLoader<'info, AggregatorAccountData>,
}

#[derive(Accounts)]
pub struct ConfirmUserStake<'info> {
    #[account(mut, close=user)]
    pub stake_token_transaction: Account<'info,StakeTokenTransaction>,
    #[account(mut)]
    pub user_token_account: Account<'info, UserAccount>,
    #[account(mut)]
    /// CHECK: pubkey will be counterchecked with stake_token_transaction before refunding rent to user
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_lp_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_records: Account<'info, StakeRecords>,
    #[account(mut)] 
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user_lp_token_account: Account<'info, UserAccount>,
    // #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    // pub thread: Account<'info, Thread>,
    // #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    // pub thread_authority: SystemAccount<'info>,
}


// #[derive(Accounts)]
// #[instruction(thread_id: Vec <u8>)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     pub pending_stake_seed_records: Account<'info,PendingStakeSeedRecords>,
//     #[account(mut)]
//     pub executor: Signer<'info>,
//     #[account(address = clockwork_sdk::ID)]
//     pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
//     pub system_program: Program<'info, System>,
//     // #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
//     // pub thread: SystemAccount<'info>,
//     // #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
//     // pub thread_authority: SystemAccount<'info>,
// }

// #[derive(Accounts)]
// pub struct ConfirmUserStakePartA<'info> {
//     #[account(mut)]
//     pub pending_stake_seed_records: Account<'info,PendingStakeSeedRecords>,
//     pub system_program: Program<'info, System>,
//     #[account(mut)]
//     pub stake_token_transaction: Account<'info, StakeTokenTransaction>,
//     #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
//     pub lp_mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub token_lp_vault: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub stake_records: Account<'info, StakeRecords>,
//     #[account(mut)] 
//     pub liquidity_pool: Account<'info, LiquidityPool>,
//     #[account(mut)]
//     pub initializer: Signer<'info>,
//     #[account(mut)]
//     pub stake_token_transaction_pda:Account<'info,PubkeyHolder>
// }

// #[derive(Accounts)]
// #[instruction(sub_seed:String)]
// pub struct ConfirmUserStakePartB<'info> {
//     pub system_program: Program<'info, System>,
//     #[account(init_if_needed, payer=initializer, seeds = [b"pending_stake", sub_seed.as_bytes()], bump, space=8+StakeTokenTransaction::INIT_SPACE)]
//     pub stake_token_transaction: Account<'info,StakeTokenTransaction>,
//     #[account(mut)]
//     pub initializer: Signer<'info>,

//     #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
//     pub lp_mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub token_lp_vault: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub stake_records: Account<'info, StakeRecords>,
//     #[account(mut)] 
//     pub liquidity_pool: Account<'info, LiquidityPool>,
// }

// #[derive(Accounts)]
// pub struct ConfirmUserStakePartC<'info> {
//     pub system_program: Program<'info, System>,
//     #[account(mut, close=user)]
//     pub stake_token_transaction: Account<'info,StakeTokenTransaction>,
//     /// CHECK: pubkey will be counterchecked with stake_token_transaction before refunding rent to user
//     pub user: AccountInfo<'info>,
//     #[account(mut)]
//     pub user_lp_token_account: Account<'info, UserAccount>,

//     pub token_program: Program<'info, Token>,
//     #[account(mut,seeds = ["lp_mint".as_bytes()], bump)]
//     pub lp_mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub token_lp_vault: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub stake_records: Account<'info, StakeRecords>,
//     #[account(mut)] 
//     pub liquidity_pool: Account<'info, LiquidityPool>,
// }