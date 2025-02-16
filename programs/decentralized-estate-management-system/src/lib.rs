use anchor_lang::prelude::*;

pub mod state;
pub mod context;
pub mod error;

pub use context::*;


declare_id!("7aYAi7dRUXcBoaXA9SHD99akAmPKwVfCJSevF9Ch3L8b");

#[program]
pub mod decentralized_estate_management_system {
    use super::*;

    pub fn initialize(ctx: Context<Estate>, name: String) -> Result<()> {

        ctx.accounts.create_estate(name, &ctx.bumps)?;
        ctx.accounts.add_leader_as_resident(&ctx.bumps)?;
        Ok(())

    }

    pub fn add_resident(ctx: Context<Resident>) -> Result<()> {

        ctx.accounts.join_estate(&ctx.bumps)?;
        Ok(())

    }

    pub fn make_deposit(ctx: Context<Deposit>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.deposit(seed, amount, &ctx.bumps)?;
        Ok(())
    }
    pub fn create_poll(ctx: Context<Poll>, description: String, amount: u64) -> Result<()> {
        ctx.accounts.create_poll(description, amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, seed: u64, vote: bool) -> Result<()> {
        ctx.accounts.vote_in_poll(seed, vote, &ctx.bumps)?;
        ctx.accounts.compute_poll(&ctx.bumps)?;
        Ok(())
    }

}
