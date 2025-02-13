
use anchor_lang::prelude::*;


#[account]
pub struct VoteState {
    pub vote: bool,
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 1 + 32 + 32 + 1 + 1;
}

