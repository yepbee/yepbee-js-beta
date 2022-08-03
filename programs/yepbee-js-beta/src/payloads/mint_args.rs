use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MintArgs {
    pub creator_key: Pubkey,
    pub metadata_uri: String,
    pub name: String,
    pub symbol: String,
    pub royalty: u16,
}
// }
