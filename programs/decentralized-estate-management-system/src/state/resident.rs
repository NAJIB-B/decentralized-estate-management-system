
use anchor_lang::prelude::*;


#[account]
pub struct Resident {
    pub user: Pubkey,
    pub estate: Pubkey,
    pub bump: u8,
    pub total_contributed: u64
}

impl Space for Resident {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 8;
}

