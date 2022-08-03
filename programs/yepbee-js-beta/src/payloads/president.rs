use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct President {
    pub pubkey: Pubkey,
    pub expiration_date: i64, // this is unix hour ex:) Date.now() / 3600000 | 0 (in js)
}
// }
impl President {
    pub const LEN: usize = 32 + 8;
}
