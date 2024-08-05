use anchor_lang::prelude::*;
use crate::constants::*;

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
    pub const INIT_SPACE:usize = PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+U64_SIZE;

    pub fn is_initialized(&self) -> bool {
        if self.lp_mint != Pubkey::default(){
            true
        }
        else{
            false
        }
    }
}