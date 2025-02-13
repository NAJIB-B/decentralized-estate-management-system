use anchor_lang::prelude::*;

use crate::state::{ResidentState, EstateState};


#[derive(Accounts)]
pub struct Resident<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init, payer = user, seeds = [b"resident", estate.key().as_ref(), user.key().as_ref()], bump, space = ResidentState::INIT_SPACE
    )]
    pub resident: Account<'info, ResidentState>,
    pub system_program: Program<'info, System>
}

impl<'info> Resident<'info> {
    pub fn join_estate(&mut self, bump: &ResidentBumps) -> Result<()> {
        self.resident.set_inner(ResidentState{
            user: self.user.key(),
            estate: self.estate.key(),
            bump: bump.resident,
            total_contributed: 0
        });

        //update number of estate residents
        self.estate.no_of_residents += 1;

        Ok(())
    }
}
