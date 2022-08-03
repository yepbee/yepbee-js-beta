use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Lister {
    pub pubkey: Pubkey,
    pub suggestion: Suggestion,
}
// }

impl Lister {
    pub const LEN: usize = 32 + Suggestion::LEN;
    #[inline]
    pub fn new(pubkey: Pubkey) -> Lister {
        Lister {
            pubkey,
            suggestion: Suggestion::None,
        }
    }
    #[inline]
    pub fn suggest(&mut self, suggestion: Suggestion) {
        self.suggestion = suggestion;
    }
    #[inline]
    pub fn eq_suggestion(&self, suggestion: &Suggestion) -> bool {
        &self.suggestion == suggestion
    }
}
