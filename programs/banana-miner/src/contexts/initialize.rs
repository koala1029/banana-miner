use anchor_lang::prelude::*;
use crate::models::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        space = 8 + 8 + 8,
        payer = user,
        seeds = [b"total_status"],
        bump
    )]
    pub total_status: Account<'info, TotalStatus>,
    #[account(
        init,
        payer = user,
        space = 8,
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK:
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
}