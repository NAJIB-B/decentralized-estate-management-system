use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};

use crate::error::DemsError;
use crate::state::{EstateState, PollState, TransactionState, VoteState};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub poll_creator: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init_if_needed , payer = user, seeds = [b"vote", estate.key().as_ref(), user.key().as_ref(), poll.key().as_ref()], bump, space = VoteState::INIT_SPACE
    )]
    pub vote: Account<'info, VoteState>,
    #[account(
        mut,
        seeds = [b"poll", estate.key().as_ref(), poll.creator.key().as_ref(), poll.description.as_str().as_bytes()],
        bump = poll.bump
    )]
    pub poll: Account<'info, PollState>,
    #[account(
        init, payer = user, seeds = [b"transaction", estate.key().as_ref(), user.key().as_ref(), seed.to_le_bytes().as_ref()], bump, space = TransactionState::INIT_SPACE
    )]
    pub transaction: Account<'info, TransactionState>,
    #[account(
        mut,
        seeds = [b"vault", estate.key().as_ref()],
        bump = estate.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Vote<'info> {
    pub fn vote_in_poll(&mut self, _seed: u64, vote: bool, bump: &VoteBumps) -> Result<()> {
        require!(self.vote.is_initialized == false, DemsError::AlreadyVoted);
        require!(self.poll.active == true, DemsError::PollClose);

        self.vote.set_inner(VoteState {
            is_initialized: true,
            bump: bump.vote,
            voter: self.user.key(),
            vote,
            poll: self.poll.key(),
        });

        let user_agree = match vote {
            true => true,
            false => false,
        };

        if user_agree {
            self.poll.agree_votes += 1;
        } else {
            self.poll.disagree_votes += 1;
        }

        Ok(())
    }

    pub fn compute_poll(&mut self, bump: &VoteBumps) -> Result<()> {
        let total_voters = self.estate.no_of_residents;

        let agree_votes = self.poll.agree_votes;
        let disagree_votes = self.poll.disagree_votes;

        if agree_votes > (total_voters as u64 + 1) / 2 {
            self.release_funds()?;

            //update estate vault balance
            self.estate.vault_balance = self.vault.to_account_info().get_lamports();

            //create transaction
            self.record_transaction(self.poll.amount, bump.transaction)?;

            self.poll.active = false
        } else if disagree_votes > (total_voters as u64 + 1) / 2 {
            self.poll.active = false;

            //close transaction account
            self.close_transaction_account()?;
        } else if agree_votes == disagree_votes
            && agree_votes + disagree_votes == total_voters as u64
        {
            self.poll.active = false;

            //close transaction account
            self.close_transaction_account()?;
        }

        Ok(())
    }
    pub fn release_funds(&self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.poll_creator.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"vault",
            self.estate.to_account_info().key.as_ref(),
            &[self.estate.vault_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer(cpi_ctx, self.poll.amount)?;

        Ok(())
    }
    pub fn record_transaction(&mut self, amount: u64, bump: u8) -> Result<()> {
        let clock = Clock::get()?.unix_timestamp;
        let is_deposit = false;

        self.transaction.set_inner(TransactionState {
            bump,
            estate: self.estate.key(),
            is_deposit,
            amount,
            timestamp: clock,
            from: self.vault.to_account_info().key(),
            to: self.poll.creator,
        });

        Ok(())
    }
    pub fn close_transaction_account(&mut self) -> Result<()> {

        //clear account data
        self.transaction.to_account_info().data.borrow_mut().fill(0);

        let lamports = self.transaction.to_account_info().lamports();

        // Transfer lamports from the account to the user
        **self.transaction.to_account_info().try_borrow_mut_lamports()? -= lamports;
        **self.user.to_account_info().try_borrow_mut_lamports()? += lamports;
        Ok(())
    }
}
