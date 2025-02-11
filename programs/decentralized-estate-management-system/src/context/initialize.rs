use anchor_lang::prelude::*;

use crate::state::Estate;
use crate::error::DemsError;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(
        init,
        payer = leader,
        seeds = [b"estate",  name.as_str().as_bytes()],
        bump,
        space = Estate::INIT_SPACE
    )]
    pub estate: Account<'info, Estate>,
    #[account(
        mut,
        seeds = [b"vault", estate.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}


impl<'info> Initialize<'info> {
    pub fn create_estate(&mut self, name: String, bumps: &InitializeBumps) -> Result<()> {

        require!(name.len() > 0 && name.len() < 4 + 32, DemsError::NameTooLong);


        self.estate.set_inner(Estate {
            name,
            leader: self.leader.key(),
            bump: bumps.estate,
            vault_bump: bumps.vault,
            vault_balance: 0,
            no_of_residents: 0
        });
        Ok(())
    }
} 
