use anchor_lang::prelude::*;


#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub estate: Pubkey,
    pub bump: u8,
    pub active: bool,
    pub description: String,
    pub amount: u64,
    pub agree_votes: u64,
    pub disagree_votes: u64
}

impl Space for Poll {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 1 + (4 + 40) + 8 + 8 + 8;
}

