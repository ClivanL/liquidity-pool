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

#[account]
pub struct UserAccount {
    pub user_token_vault: Pubkey,
    pub user: Pubkey,
    pub balance: u64,
    pub token_name: Vec<u8>,
}

impl UserAccount {
    pub const INIT_SPACE:usize = PUBKEY_SIZE+PUBKEY_SIZE+U64_SIZE+TOKEN_NAME_SIZE;
}

#[account]
pub struct StakeRecords{
    pub token_a_stake: u64,
    pub token_b_stake: u64,
    pub token_c_stake: u64,
    pub token_d_stake: u64,
    pub token_e_stake: u64,
}

impl StakeRecords{
    pub const INIT_SPACE:usize = U64_SIZE+U64_SIZE+U64_SIZE+U64_SIZE+U64_SIZE;
}