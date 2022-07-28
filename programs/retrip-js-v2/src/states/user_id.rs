use crate::common::*;

#[account]
pub struct UserId {
    pub id: u128,
    pub bump: u8,
}

impl UserId {
    pub const LEN: usize = 16 + 1;
}
