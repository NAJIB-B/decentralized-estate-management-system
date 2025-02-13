use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};

use crate::error::DemsError;
use crate::state::{EstateState, ResidentState};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump,
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        mut,
        seeds = [b"vault", estate.key().as_ref()],
        bump = estate.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
       has_one = user,
       seeds = [b"resident", estate.key().as_ref(), user.key().as_ref()], bump=resident.bump,
    )]
    pub resident: Account<'info, ResidentState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        //update estate vault balance
        self.estate.vault_balance = self.vault.to_account_info().get_lamports();

        //update resident contributions
        self.resident.total_contributed =  self.vault.to_account_info().get_lamports();


        Ok(())
    }
}
