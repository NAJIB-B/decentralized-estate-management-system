use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};

use crate::error::DemsError;
use crate::state::{EstateState, PollState, VoteState};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"estate", estate.name.as_str().as_bytes()],
        bump = estate.bump
    )]
    pub estate: Account<'info, EstateState>,
    #[account(
        init_if_needed , payer = user, seeds = [b"vote", estate.key().as_ref(), user.key().as_ref()], bump, space = VoteState::INIT_SPACE
    )]
    pub vote: Account<'info, VoteState>,
    #[account(
        seeds = [b"poll", estate.key().as_ref(), user.key().as_ref()],
        bump = poll.bump
    )]
    pub poll: Account<'info, PollState>,
    #[account(
        mut,
        seeds = [b"vault", estate.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub enum PollStatus {
    Finalized(bool),
    Pending,
}

impl<'info> Vote<'info> {
    pub fn vote_in_poll(&mut self, vote: bool, bump: &VoteBumps) -> Result<()> {
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

    pub fn compute_poll(&mut self) -> Result<()> {
        let total_voters = self.estate.no_of_residents;

        let agree_votes = self.poll.agree_votes;
        let disagree_votes = self.poll.disagree_votes;

        if agree_votes > (total_voters as f64 / 2.0).round() as u64 {
            self.release_funds()?;

            //update estate vault balance
            self.estate.vault_balance = self.vault.to_account_info().get_lamports();

            self.poll.active = false
        } else if disagree_votes > (total_voters as f64 / 2.0).round() as u64 {
            self.poll.active = false
        } else if agree_votes == disagree_votes
            && agree_votes + disagree_votes == total_voters as u64
        {
            self.poll.active = false
        }

        Ok(())
    }
    pub fn release_funds(&self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
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
}
