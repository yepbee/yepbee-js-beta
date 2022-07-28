use crate::common::*;

#[account]
pub struct NftId {
    pub id: u128,
    pub bump: u8,
}

impl NftId {
    pub const LEN: usize = 16 + 1;
}
