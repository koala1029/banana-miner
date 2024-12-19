use anchor_lang::prelude::*;
use crate::models::*;
use crate::constants::*;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct UserActions<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_info", user.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(
        mut,
        seeds = [b"total_status"],
        bump
    )]
    pub total_status: Account<'info, TotalStatus>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK:
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UserActions<'info> {
    pub fn compound(&mut self) -> Result<()> {
        let user_info = &mut self.user_info;
        let total_status = &mut self.total_status;

        require!(total_status.total_supply != 0, ErrorCode::InvalidOperation);
        
        // Get the current timestamp
        let now: u64 = Clock::get()?.unix_timestamp as u64;
        
        // Calculate pending rewards if the user has an updated timestamp
        let elapsed_time = now - user_info.last_updated_timestamp;
        let daily_reward = DEFAULT_APR * user_info.banana_balance * elapsed_time / (3600 * 24 * 100);
        let pending_reward = user_info.pending_reward + daily_reward;

        let banana_price = total_status.total_locked_balance / total_status.total_supply;

        // Update user's information
        user_info.last_updated_timestamp = now;
        user_info.pending_reward = 0;
        user_info.balance += pending_reward * banana_price;
        user_info.banana_balance += pending_reward;

        total_status.total_supply += pending_reward;
        Ok(())
    }

    pub fn claim_rewards(&mut self) -> Result<()> {
        let user_info = &mut self.user_info;
        let total_status = &mut self.total_status;

        require!(total_status.total_supply != 0, ErrorCode::InvalidOperation);

        // Get the current timestamp
        let now: u64 = Clock::get()?.unix_timestamp as u64;
        
        // Calculate pending rewards if the user has an updated timestamp
        let elapsed_time = now - user_info.last_updated_timestamp;
        let daily_reward = DEFAULT_APR * user_info.banana_balance * elapsed_time / (3600 * 24 * 100);
        let pending_reward = user_info.pending_reward + daily_reward;

        let banana_price = total_status.total_locked_balance / total_status.total_supply;

        // Update user's information
        user_info.last_updated_timestamp = now;
        user_info.pending_reward = 0;

        let reward_amount = pending_reward * banana_price;

        // Perform transfer from vault to user
        **self.vault.to_account_info().try_borrow_mut_lamports()? -= reward_amount;
        **self.user.to_account_info().try_borrow_mut_lamports()? += reward_amount;

        total_status.total_locked_balance -= reward_amount;
        Ok(())
    }
}