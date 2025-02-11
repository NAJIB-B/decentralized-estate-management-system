use anchor_lang::prelude::*;

pub mod state;
pub mod context;
pub mod error;

declare_id!("7aYAi7dRUXcBoaXA9SHD99akAmPKwVfCJSevF9Ch3L8b");

#[program]
pub mod decentralized_estate_management_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
