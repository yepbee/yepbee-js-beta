use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Transaction {
    // Target program to execute against.
    pub program_id: Pubkey,
    // Accounts requried for the transaction.
    pub accounts: Vec<TransactionAccount>, // 10 amount
    // Instruction data for the transaction.
    pub data: Vec<u8>,
}
// }

impl Transaction {
    pub const ACCOUNTS_COUNT: usize = 3;
    pub const LEN: usize = 32
        + (4 + TransactionAccount::LEN * Transaction::ACCOUNTS_COUNT)
        + (4 + 1 * Transaction::ACCOUNTS_COUNT);
}

impl From<&Transaction> for Instruction {
    #[inline]
    fn from(tx: &Transaction) -> Instruction {
        Instruction {
            program_id: tx.program_id,
            accounts: tx.accounts.iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}
