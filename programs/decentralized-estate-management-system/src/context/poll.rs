use anchor_lang::prelude::*;

use crate::state::{EstateState, PollState};
use crate::error::DemsError;

#[derive(Accounts)]
pub struct Poll<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init, payer = user, seeds = [b"poll", estate.key().as_ref(), user.key().as_ref()], bump, space = PollState::INIT_SPACE
    )]
    pub poll: Account<'info, PollState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Poll<'info> {
    pub fn create_poll(&mut self, amount: u64, description: String,  bump: &PollBumps) -> Result<()> {

        require!(description.len() > 0 && description.len() < (4 + 40), DemsError::NameTooLong);


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
