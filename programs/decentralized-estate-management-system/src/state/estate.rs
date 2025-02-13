use anchor_lang::prelude::*;


#[account]
pub struct EstateState {
    pub leader: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub no_of_residents: u32,
    pub vault_balance: u64,
    pub name: String,
}

impl Space for EstateState {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1 + 4 + 8 + (4 + 32);
}

