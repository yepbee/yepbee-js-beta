use crate::common::*;

#[account]
pub struct UserState {
    pub index: u128,    // 16
    pub pubkey: Pubkey, // 32
    pub token_account_info: TokenAccountInfo,
    pub bump: Bumps,
}
impl UserState {
    // pub const NAME_LEN: usize = 36;
    pub const LEN: usize = 16 + 32 + TokenAccountInfo::LEN + Bumps::LEN;
}
