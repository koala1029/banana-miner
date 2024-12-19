use anchor_lang::prelude::*;

declare_id!("91WzewhWPDC6xQJHEH2LDBNxU4h2tPSBJX4wnBgZCdgp");

pub mod contexts;
pub mod models;
pub mod constants;
pub mod errors;

pub use contexts::*;

#[program]
pub mod banana_miner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }

    pub fn buy_banana(ctx: Context<BuyBanana>, amount: u64) -> Result<()> {
        ctx.accounts.buy_banana(amount)
    }

    pub fn compound(ctx: Context<UserActions>) -> Result<()> {
        ctx.accounts.compound()
    }

    pub fn claim_rewards(ctx: Context<UserActions>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }
}