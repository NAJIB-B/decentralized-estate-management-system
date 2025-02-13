use anchor_lang::prelude::*;

use crate::state::{EstateState, ResidentState};
use crate::error::DemsError;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Estate<'info> {
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(
        init,
        payer = leader,
        seeds = [b"estate",  name.as_str().as_bytes()],
        bump,
        space = EstateState::INIT_SPACE
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init, payer = leader, seeds = [b"resident", estate.key().as_ref(), leader.key().as_ref()], bump, space = ResidentState::INIT_SPACE
    )]
    pub resident: Account<'info, ResidentState>,
    #[account(
        mut,
        seeds = [b"vault", estate.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}


impl<'info> Estate<'info> {
    pub fn create_estate(&mut self, name: String, bumps: &EstateBumps) -> Result<()> {

        require!(name.len() > 0 && name.len() < 4 + 32, DemsError::NameTooLong);


        self.estate.set_inner(EstateState {
            name,
            leader: self.leader.key(),
            bump: bumps.estate,
            vault_bump: bumps.vault,
            vault_balance: 0,
            no_of_residents: 0
        });
        Ok(())
    }

    pub fn add_leader_as_resident(&mut self, bumps: &EstateBumps) -> Result<()> {

        self.resident.set_inner(ResidentState{
            user: self.leader.key(),
            estate: self.estate.key(),
            bump: bumps.resident,
            total_contributed: 0
        });

        //update number of residents
        self.estate.no_of_residents = 1;
        Ok(())
    }
} 
