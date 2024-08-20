use anchor_lang::prelude::*;
use crate::context::*;
use solana_program::pubkey::Pubkey;

pub fn handler(ctx: Context<ConfirmUserStake>) -> Result<()> {

    let pendingStakeSeedRecords = &mut ctx.accounts.pending_stake_seed_records;
    let sub_seeds = &pendingStakeSeedRecords.sub_seeds;
    let program_id = ctx.program_id;

    for sub_seed in sub_seeds.iter(){
        let seeds = &[b"pending_stake".as_ref(), sub_seed.copy().as_ref()];
        let (pda,bump) = Pubkey::find_program_address(seeds,program_id)?;
        msg!("PDA: {:?}", pda);

        let stake_token_transaction:StakeTokenTransaction = program.account(pda)?;

        let user_seeds = &[b"lp_token".as_ref(), stake_token_transaction.user_pubkey.copy().as_ref()];
        let (user_pda,user_bump)  = Pubkey::find_program_address(seeds, program_id);

        // check if the lp_token account exists for the user
        let user_lp_token_account:UserAccount = program.account(user_pda)?;

        // create a new pda address if account does not exist
        if user_lp_token_account.data_len()!=8+UserAccount::INIT_SPACE{
            let (user_pda,user_bump)  = Pubkey::create_program_address(seeds, program_id);
            let space = 8+PendingStakeSeedRecords::INIT_SPACE;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let create_account_ix = system_program::create_account(
                &ctx.accounts.initializer.to_account_info(),
                &user_pda.to_account_info(),
                lamports,
                space as u64,
                &ctx.accounts.system_program.to_account_info(),
            );
            
            // Execute the create account instruction
            invoke_signed(
                &create_account_ix,
                &[ctx.accounts.initializer.to_account_info(), user_pda.to_account_info()],
                &[&user_seeds[..], &[user_bump]]
            )?;
        }

        let user_lp_token_account:UserAccount = program.account(user_pda)?;
        
        let user_token_seeds = &[stake_token_transaction.token_name.copy().as_ref(), stake_token_transaction.user_pubkey.copy().as_ref()]; //do we use copy? when to use use_ref?
        let (user_token_pda,user_token_bump) = Pubkey::find_program_address(user_token_seeds,program_id)?;
        let user_token_account:UserAccount = program.account(user_token_pda)?;

        if user_token_account.token_name!=stake_token_transaction.token_name{  //comparing of vectors- to review again
            return Err(CustomError::WrongAccountRetrieval.into());
        }
        if user_token_account.user!=stake_token_transaction.user{  //comparing of pubkey, dont need to dereference?
            return Err(CustomError::WrongAccountRetrieval.into());
        }

        // mint to vault for lp_token
        token::mint_to(
            CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.token_lp_vault.to_account_info(),
                mint: ctx.accounts.lp_mint.to_account_info()
            },
            &[&[
                "lp_mint".as_bytes(),
                &[ctx.bumps.lp_mint]
            ]]
        ), user_token_account.pending_stake as u64)?; 

        // update total minted lp token supply in liquidity pool
        let liquidity_pool = &mut ctx.accounts.liquidity_pool;
        liquidity_pool.total_lp_supply+=user_token_account.pending_stake;

        user_lp_token_account.token_name = "lp_token".into_bytes();
        user_lp_token_account.balance += user_token_account.pending_stake;
        user_lp_token_account.user = user_token_account.user.copy();

        user_token_account.pending_stake = 0.0;

            // update staked records 
        let stake_records = &mut ctx.accounts.stake_records;
        match stake_token_transaction.token_name {
            "token_a" => {
                let new_balance = stake_records.token_a_stake+stake_balance;
                if new_balance.is_nan() || new_balance.is_infinite(){
                    return Err(CustomError::InvalidValue.into());
                }
                stake_records.token_a_stake = new_balance;
            },
            "token_b" => {
                let new_balance = stake_records.token_b_stake+stake_balance;
                if new_balance.is_nan() || new_balance.is_infinite(){
                    return Err(CustomError::InvalidValue.into());
                }
                stake_records.token_b_stake = new_balance;
            },
            "token_c" => {
                let new_balance = stake_records.token_c_stake+stake_balance;
                if new_balance.is_nan() || new_balance.is_infinite(){
                    return Err(CustomError::InvalidValue.into());
                }
                stake_records.token_c_stake = new_balance;
            },
            "token_d" => {
                let new_balance = stake_records.token_d_stake+stake_balance;
                if new_balance.is_nan() || new_balance.is_infinite(){
                    return Err(CustomError::InvalidValue.into());
                }
                stake_records.token_d_stake = new_balance;
            },
            "token_e" => {
                let new_balance = stake_records.token_e_stake+stake_balance;
                if new_balance.is_nan() || new_balance.is_infinite(){
                    return Err(CustomError::InvalidValue.into());
                }
                stake_records.token_e_stake = new_balance;
            },
            _=>{
                return Err(CustomError::InvalidTokenName.into());
            }
        }
    }
    Ok(())
}