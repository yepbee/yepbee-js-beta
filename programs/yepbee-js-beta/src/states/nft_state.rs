use crate::common::*;

#[account]
pub struct NftState {
    pub index: u128,    // 16
    pub pubkey: Pubkey, // 32

    pub likes: u128,  // 16
    pub stakes: u128, // 16

    pub creator: Pubkey, // 32
    pub owner: Pubkey,   // 32

    pub bump: Bumps,
}
impl NftState {
    pub const LEN: usize = 16 + 32 + 16 + 16 + 32 + 32 + Bumps::LEN;
}
