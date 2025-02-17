use anchor_lang::prelude::*;

use crate::state::{EstateState, PollState};
use crate::error::DemsError;

#[derive(Accounts)]
#[instruction(description: String)]
pub struct Poll<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init, payer = user, seeds = [b"poll", estate.key().as_ref(), user.key().as_ref(), description.as_str().as_bytes()], bump, space = PollState::INIT_SPACE
    )]
    pub poll: Account<'info, PollState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Poll<'info> {
    pub fn create_poll(&mut self, description: String, amount: u64, bump: &PollBumps) -> Result<()> {

        require!(description.len() < (4 + 40), DemsError::DescriptionTooLong);
        require!(description.len() > 0, DemsError::InvalidDescription);
        require!(amount < self.estate.vault_balance, DemsError::ExceededBalance);


        self.poll.set_inner(PollState {
            active: true,
            agree_votes: 0,
            disagree_votes: 0,
            creator: self.user.key(),
            estate: self.estate.key(),
            bump: bump.poll,
            amount,
            description
        });

        Ok(())
    }
}
