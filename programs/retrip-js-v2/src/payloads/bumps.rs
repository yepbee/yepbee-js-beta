use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct Bumps {
    pub pubkey_to_id_bump: u8,
    pub id_to_state_bump: u8, // this is unix hour ex:) Date.now() / 3600000 | 0 (in js)
}
// }
impl Bumps {
    pub const LEN: usize = 2;
    #[inline]
    pub fn new(pubkey_to_id_bump: u8, id_to_state_bump: u8) -> Bumps {
        Bumps {
            pubkey_to_id_bump,
            id_to_state_bump,
        }
    }
}
