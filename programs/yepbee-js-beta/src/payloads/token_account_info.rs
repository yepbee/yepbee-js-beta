use crate::{
    common::*,
    states::{NftState, UserState},
};

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct TokenAccountInfo {
    pub mint_address: Pubkey,
    pub token_account: Pubkey,
    pub bump: [u8; 1],
    pub creator: Pubkey,
}
// }
impl TokenAccountInfo {
    pub const LEN: usize = 32 + 32 + 32 + 1;
    #[inline]
    pub fn into_nft_state(
        &self,
        nft_index: u128,
        nft_address: &Pubkey,
        initial_owner: &Pubkey,
    ) -> NftState {
        let pubkey_to_nft_id_bump = find_bump(&[b"nft_state-pubkey", nft_address.as_ref()]);
        let nft_id_to_nft_state_bump =
            find_bump(&[b"nft_state-id", nft_index.to_string().as_bytes()]);

        NftState {
            index: nft_index,
            pubkey: nft_address.key(),
            likes: 0,
            stakes: 0,
            creator: initial_owner.key(),
            owner: initial_owner.key(),
            bump: Bumps::new(pubkey_to_nft_id_bump, nft_id_to_nft_state_bump),
        }
    }
    #[inline]
    pub fn into_user_state(&self, user_index: u128, user_address: &Pubkey) -> UserState {
        let (token_account, bump) = find_token_account(&self.mint_address, &user_address);

        let pubkey_to_user_id_bump = find_bump(&[b"user_state-pubkey", user_address.as_ref()]);
        let user_id_to_user_state_bump =
            find_bump(&[b"user_state-id", user_index.to_string().as_bytes()]);

        UserState {
            index: user_index,
            pubkey: user_address.key(),
            token_account_info: TokenAccountInfo {
                mint_address: self.mint_address,
                token_account,
                bump: [bump],
                creator: self.creator,
            },
            bump: Bumps::new(pubkey_to_user_id_bump, user_id_to_user_state_bump),
        }
    }
    #[inline]
    pub fn new(
        mint_address: Pubkey,
        token_account: Pubkey,
        bump: u8,
        creator: Pubkey,
    ) -> TokenAccountInfo {
        TokenAccountInfo {
            mint_address,
            token_account,
            creator,
            bump: [bump],
        }
    }
    #[inline]
    pub fn new_from_pubkey(
        mint_address: Pubkey,
        user_address: Pubkey,
        creator: Pubkey,
    ) -> TokenAccountInfo {
        let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();
        let (token_account, bump) = Pubkey::find_program_address(
            &[mint_address.as_ref(), user_address.as_ref()],
            &program_id,
        );

        TokenAccountInfo {
            mint_address,
            token_account,
            bump: [bump],
            creator,
        }
    }
    #[inline]
    pub fn to_bump(&self) -> u8 {
        self.bump[0]
    }
    #[inline]
    pub fn as_seeds(&self) -> [&[u8]; 2] {
        let seeds = [self.mint_address.as_ref(), &self.bump];
        seeds
    }
}
