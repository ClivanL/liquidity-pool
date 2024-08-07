use anchor_lang::prelude::*;
use crate::context::*;

pub fn handler(ctx: Context<InitStakeRecords>) -> Result<()> {
    let stake_records = &mut ctx.accounts.stake_records;

    stake_records.token_a_stake=0;
    stake_records.token_b_stake=0;
    stake_records.token_c_stake=0;
    stake_records.token_d_stake=0;
    stake_records.token_e_stake=0;

    Ok(())
}