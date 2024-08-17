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
    pub total_lp_supply: f64
}

impl LiquidityPool {
    pub const INIT_SPACE:usize = PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+PUBKEY_SIZE+F64_SIZE;

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
    pub balance: f64,
    pub pending_stake: f64,
    pub token_name: Vec<u8>
    
}

impl UserAccount {
    pub const INIT_SPACE:usize = PUBKEY_SIZE+PUBKEY_SIZE+F64_SIZE+F64_SIZE+TOKEN_NAME_SIZE;
}

#[account]
pub struct StakeRecords{
    pub token_a_stake: f64,
    pub token_b_stake: f64,
    pub token_c_stake: f64,
    pub token_d_stake: f64,
    pub token_e_stake: f64,
}

impl StakeRecords{
    pub const INIT_SPACE:usize = F64_SIZE+F64_SIZE+F64_SIZE+F64_SIZE+F64_SIZE;
}

// #[account(zero_copy)]
// pub struct AggregatorAccountData {
//     pub rates: [u64; 5],
// }

#[account]
pub struct PendingStakeSeedRecords{
    pub last_index:u8,
    pub sub_seeds:Vec<String>,
    
}

impl PendingStakeSeedRecords{
    pub const INIT_SPACE:usize = U8_SIZE+SUB_SEEDS_VECTOR_SIZE;
}

#[account]
pub struct StakeTokenTransaction{
    pub stake_amount:f64,
    pub tokens_to_mint:u64,
    pub exchange_rate:f64,
    pub token_name: Vec<u8>,
    pub user_pubkey:Pubkey
}

impl StakeTokenTransaction{
    pub const INIT_SPACE:usize = F64_SIZE+U64_SIZE+F64_SIZE+TOKEN_NAME_SIZE+PUBKEY_SIZE;
}