use anchor_lang::prelude::*;
use crate::context::*;

pub fn handler(ctx: Context<CreatePendingTransfersRecord>) -> Result<()> {
    let pending_transfers_record = &mut ctx.accounts.pending_transfers_record;
    pending_transfers_record.last_index = 0;
    pending_transfers_record.pending_transfer_subseeds = Vec::new();
    Ok(())
}