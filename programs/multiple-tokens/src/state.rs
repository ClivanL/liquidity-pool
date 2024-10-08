use anchor_lang::prelude::*;
use crate::constants::*;
use crate::enums::*;

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

#[account]
pub struct PubkeyHolder {
    pub pubkey: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct LimitOrder {
    pub user: Pubkey,
    pub amount_to_trade: f64,
    pub exchange_rate: f64,
    pub created_at: i64,
    pub token_pair: TokenPair,
    pub closed: bool,
    pub direction: Direction,
    #[max_len(MAX_SUBSEED_CHAR)]
    pub sub_seed: String
}

// impl LimitOrder{
//     pub const INIT_SPACE:usize = PUBKEY_SIZE + F64_SIZE + F64_SIZE + I64_SIZE + ENUM_SIZE + BOOL_SIZE + ENUM_SIZE;
// }

#[account]
#[derive(InitSpace)]
pub struct OrderBookDirectory {
    pub last_index:u8,
    pub token_pair:TokenPair,
    pub direction:Direction,
    #[max_len(MAX_ORDERBOOK,MAX_STRING_LENGTH)]
    pub orderbook_subseeds: Vec<String>
}

#[account]
#[derive(InitSpace)]
pub struct OrderBook {
    pub last_index:u8,
    pub token_pair:TokenPair,
    pub direction:Direction,
    #[max_len(MAX_ORDERS)]
    pub orders: Vec<LimitOrder>
}

// impl OrderBook {
//     pub const INIT_SPACE:usize = U8_SIZE + ENUM_SIZE + ENUM_SIZE + LIMIT_ORDER_VECTOR_SIZE;
// }

#[account]
#[derive(InitSpace)]
pub struct PendingTransfersRecord {
    pub last_index:u8,
    #[max_len(MAX_PENDING_TRANSFER,MAX_STRING_LENGTH)]
    pub pending_transfer_subseeds: Vec<String>
}

#[account]
#[derive(InitSpace)]
pub struct PendingTransfers {
    #[max_len(MAX_TRANSFERS)]
    pub transfers: Vec<Transfer>
}

#[account]
#[derive(InitSpace)]
pub struct Transfer {
    pub account_from: Pubkey,
    pub account_to: Pubkey,
    pub amount: f64,
    #[max_len(TOKEN_NAME_SIZE)]
    pub token_name: Vec<u8>
}