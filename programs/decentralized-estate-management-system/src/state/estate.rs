use anchor_lang::prelude::*;


#[account]
pub struct Estate {
    pub name: String,
    pub leader: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub no_of_residents: u32,
    pub vault_balance: u64
}

impl Space for Estate {
    const INIT_SPACE: usize = 8 + (4 + 32) + 32  + 1 + 1 + 4 + 8;
}

