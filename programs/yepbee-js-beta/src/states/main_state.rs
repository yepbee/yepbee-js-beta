use crate::common::*;

#[account]
#[derive(Default)]
pub struct MainState {
    total_user_supply: u128,
    total_nft_supply: u128,
    white_list: Whitelist,
    program_token_account_info: TokenAccountInfo, // this has also mint key
}
impl MainState {
    // pub const ACCOUNT_LEN: usize = 32 + 1 + 1 + 4 + 4 + 32 + 1 + 8;
    pub const LEN: usize = 16 + 16 + Whitelist::LEN + TokenAccountInfo::LEN; //state::Mint::LEN + state::Account::LEN + 2 * Self::ACCOUNT_LEN;

    #[inline]
    pub fn new(program_token_account: &TokenAccountInfo, payer: &Signer) -> MainState {
        MainState::default().init(program_token_account, payer)
    }

    #[inline]
    pub fn init(
        mut self,
        program_token_account_info: &TokenAccountInfo,
        payer: &Signer,
    ) -> MainState {
        let initial_owner = payer.key();
        msg!("initial whitelist: {initial_owner}");
        self.white_list.push_keys(&[initial_owner]);
        self.program_token_account_info = program_token_account_info.clone();
        self.total_user_supply = 0;
        self.total_nft_supply = 0;
        self
    }

    #[inline]
    pub fn validate_signer(&mut self, signer: &Signer) -> Result<()> {
        match self.white_list.contains_key(signer.key()) {
            true => Ok(()),
            false => Err(Errors::NotInWhiteList.into()),
        }
    }
    #[inline]
    pub fn validate_whitelist_pubkey(&mut self, pubkey: &Pubkey) -> Result<()> {
        match self.white_list.contains_key(pubkey) {
            true => Ok(()),
            false => Err(Errors::TargetIsNotInWhiteList.into()),
        }
    }

    #[inline]
    pub fn as_whitelist(&self) -> &Whitelist {
        &self.white_list
    }
    #[inline]
    pub fn as_mut_whitelist(&mut self) -> &mut Whitelist {
        &mut self.white_list
    }
    #[inline]
    pub fn as_program_token_account_info(&self) -> &TokenAccountInfo {
        &self.program_token_account_info
    }
    #[inline]
    pub fn as_mut_program_token_account_info(&mut self) -> &mut TokenAccountInfo {
        &mut self.program_token_account_info
    }

    #[inline]
    pub fn as_creator(&self) -> &Pubkey {
        &self.program_token_account_info.creator
    }
    #[inline]
    pub fn as_program_mint(&self) -> &Pubkey {
        &self.program_token_account_info.mint_address
    }
    #[inline]
    pub fn as_program_token_account(&self) -> &Pubkey {
        &self.program_token_account_info.token_account
    }
    #[inline]
    pub fn as_program_token_account_bump(&self) -> u8 {
        self.program_token_account_info.to_bump()
    }

    #[inline]
    pub fn to_total_user_supply(&self) -> u128 {
        self.total_user_supply
    }
    #[inline]
    pub fn to_total_nft_supply(&self) -> u128 {
        self.total_nft_supply
    }
    #[inline]
    pub fn increment_user_supply(&mut self) -> u128 {
        match self.total_user_supply.checked_add(1) {
            None => Err(Errors::OverflowOccurs).unwrap(),
            Some(n) => self.total_user_supply = n,
        }
        self.total_user_supply
    }
    #[inline]
    pub fn decrement_user_supply(&mut self) -> u128 {
        match self.total_user_supply.checked_sub(1) {
            None => Err(Errors::OverflowOccurs).unwrap(),
            Some(n) => self.total_user_supply = n,
        }
        self.total_user_supply
    }
    #[inline]
    pub fn increment_nft_supply(&mut self) -> u128 {
        match self.total_nft_supply.checked_add(1) {
            None => Err(Errors::OverflowOccurs).unwrap(),
            Some(n) => self.total_nft_supply = n,
        }
        self.total_nft_supply
    }
    #[inline]
    pub fn decrement_nft_supply(&mut self) -> u128 {
        match self.total_nft_supply.checked_sub(1) {
            None => Err(Errors::OverflowOccurs).unwrap(),
            Some(n) => self.total_nft_supply = n,
        }
        self.total_nft_supply
    }
}
