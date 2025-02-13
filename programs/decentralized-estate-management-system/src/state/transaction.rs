use anchor_lang::prelude::*;


#[account]
pub struct TransactionState {
    pub initiator: Pubkey,
    pub estate: Pubkey,
    pub bump: u8,
    pub timestamp: i64,
    pub is_deposit: bool,
    pub amount: u64,
}

impl Space for TransactionState {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 8 + 1 + 8;
}

