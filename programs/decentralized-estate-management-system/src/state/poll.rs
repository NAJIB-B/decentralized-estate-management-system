use anchor_lang::prelude::*;


#[account]
pub struct PollState {
    pub creator: Pubkey,
    pub estate: Pubkey,
    pub bump: u8,
    pub active: bool,
    pub amount: u64,
    pub agree_votes: u64,
    pub disagree_votes: u64,
    pub description: String,
}

impl Space for PollState {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 1 + 8 + 8 + 8 + (4 + 40);
}

