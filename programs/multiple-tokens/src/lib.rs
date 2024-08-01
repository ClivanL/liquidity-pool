use anchor_lang::prelude::*;

declare_id!("EDBwJ2TUonePxXiA7C46VEucdw7LQE6GfDnytkauBJ6f");

#[program]
pub mod multiple_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
