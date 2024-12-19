use anchor_lang::prelude::*;

#[account]
pub struct UserInfo {
    pub user: Pubkey,
    pub balance: u64,
    pub banana_balance: u64,
    pub current_apr: u64,
    pub pending_reward: u64,
    pub last_updated_timestamp: u64,
}

#[account]
pub struct TotalStatus {
    pub total_locked_balance: u64,
    pub total_supply: u64,
}