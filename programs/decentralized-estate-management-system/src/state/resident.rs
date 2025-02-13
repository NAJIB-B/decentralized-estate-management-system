
use anchor_lang::prelude::*;


#[account]
pub struct ResidentState {
    pub user: Pubkey,
    pub estate: Pubkey,
    pub bump: u8,
    pub total_contributed: u64
}

impl Space for ResidentState {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 8;
}

