use anchor_lang::prelude::*;


#[account]
pub struct TransactionState {
    pub estate: Pubkey,
    pub bump: u8,
    pub timestamp: i64,
    pub is_deposit: bool,
    pub amount: u64,
    pub from: Pubkey,
    pub to: Pubkey,
}

impl Space for TransactionState {
    const INIT_SPACE: usize = 8 + 32 + 1 + 8 + 1 + 8 + 32 + 32;
}

