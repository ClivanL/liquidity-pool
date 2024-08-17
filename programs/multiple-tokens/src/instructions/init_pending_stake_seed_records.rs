use anchor_lang::prelude::*;
use crate::context::*;

pub fn handler(ctx: Context<InitPendingStakeSeedRecords>) -> Result<()> {
    let pending_stake_seed_records = &mut ctx.accounts.pending_stake_seed_records;
    pending_stake_seed_records.sub_seeds = Vec::new();
    pending_stake_seed_records.last_index = 0;
    Ok(())
}