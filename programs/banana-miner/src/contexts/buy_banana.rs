use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use crate::models::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct BuyBanana<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8,
        payer = user,
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

impl<'info> BuyBanana<'info> {
    pub fn buy_banana(&mut self, amount: u64) -> Result<()> {
        let user_info = &mut self.user_info;
        let total_status = &mut self.total_status;
        
        // Perform transfer from user to vault
        let ix = system_instruction::transfer(&self.user.key(), &self.vault.key(), amount);
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[self.user.to_account_info(), self.vault.to_account_info()],
        )?;

        // Get the current timestamp
        let now: u64 = Clock::get()?.unix_timestamp as u64;
        
        // Calculate pending rewards if the user has an updated timestamp
        if user_info.last_updated_timestamp > 0 {
            let elapsed_time = now - user_info.last_updated_timestamp;
            let daily_reward = DEFAULT_APR * user_info.banana_balance * elapsed_time / (3600 * 24 * 100);
            user_info.pending_reward = user_info.pending_reward + daily_reward;
        }

        // Update user's information
        let banana_increment = if total_status.total_supply == 0 {
            amount
        } else {
            amount * total_status.total_supply / (total_status.total_locked_balance + amount)
        };

        user_info.user = self.user.key();
        user_info.balance += amount;
        user_info.banana_balance += banana_increment;
        user_info.last_updated_timestamp = now;

        total_status.total_locked_balance += amount;
        total_status.total_supply += banana_increment;

        Ok(())
    }
}