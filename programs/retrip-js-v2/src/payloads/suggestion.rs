use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Suggestion {
    ExecTransactions { txs: Vec<Transaction> },
    AddWhiteList { pubkeys: Vec<Pubkey> },
    DelWhiteList { pubkeys: Vec<Pubkey> },
    VotePresident { vote: Option<President> },
    None,
}
// }
impl Suggestion {
    pub const LEN: usize = Transaction::LEN; // CHECK THE MAX SIZE
}

impl Default for Suggestion {
    fn default() -> Self {
        Self::None
    }
}
