use anchor_lang::prelude::*;
use crate::context::*;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let system_program = &ctx.accounts.system_program;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let executor = &ctx.accounts.executor;
    let thread = &ctx.accounts.thread;
    let thread_authority = &ctx.accounts.thread_authority;
    let pending_stake_seed_records = &ctx.accounts.pending_stake_seed_records;


    Ok(())
}