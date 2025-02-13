use anchor_lang::prelude::*;

use crate::state::{EstateState, TransactionState};
use crate::error::DemsError;

#[derive(Accounts)]
pub struct Transaction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init, payer = user, seeds = [b"transaction", estate.key().as_ref(), user.key().as_ref()], bump, space = TransactionState::INIT_SPACE
    )]
    pub transaction: Account<'info, TransactionState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Transaction<'info> {
    pub fn record_transaction(&mut self, amount: u64, is_deposit: bool, bump: &TransactionBumps) -> Result<()> {

        let clock = Clock::get()?.unix_timestamp;

        self.transaction.set_inner(TransactionState {
            initiator: self.user.key(),
            bump: bump.transaction,
            estate: self.estate.key(),
            amount,
            is_deposit,
            timestamp: clock
        });

        Ok(())
    }
}
