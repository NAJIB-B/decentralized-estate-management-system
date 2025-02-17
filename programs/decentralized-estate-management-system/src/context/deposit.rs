use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};

use crate::state::{EstateState, ResidentState, TransactionState};
use crate::error::DemsError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
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
        init, payer = user, seeds = [b"transaction", estate.key().as_ref(), user.key().as_ref(), seed.to_le_bytes().as_ref()], bump, space = TransactionState::INIT_SPACE
    )]
    pub transaction: Account<'info, TransactionState>,
    #[account(
        mut,
       has_one = user,
       seeds = [b"resident", estate.key().as_ref(), user.key().as_ref()], bump=resident.bump,
    )]
    pub resident: Account<'info, ResidentState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, _seed: u64, amount: u64, bump: &DepositBumps) -> Result<()> {
        require!(amount > 0, DemsError::InvalidAmount);
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        //update estate vault balance
        self.estate.vault_balance += self.vault.to_account_info().get_lamports();

        //update resident contributions
        self.resident.total_contributed = amount;

        //record transaction
        self.record_transaction(amount, bump.transaction)?;


        Ok(())
    }

    pub fn record_transaction(&mut self, amount: u64, bump: u8) -> Result<()> {


        let clock = Clock::get()?.unix_timestamp;
        let is_deposit = true;

        self.transaction.set_inner(TransactionState {
            bump,
            estate: self.estate.key(),
            is_deposit,
            amount,
            timestamp: clock,
            from: self.user.to_account_info().key(),
            to: self.vault.to_account_info().key()
        });

        Ok(())
    }
}
