use crate::{
    common::*,
    states::{MainState, NftId, NftState},
};

#[derive(Accounts)]
pub struct CreateNft<'info> {
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, spl::token::Token>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub main_state: Account<'info, MainState>,

    /// CHECK: for argument
    #[account(mut)]
    pub user_pubkey: AccountInfo<'info>,

    // #[account(
    //     mut,
    //     seeds = [ b"user_state", user_pubkey.key().as_ref() ],
    //     bump = user_state.bump,
    // )]
    // pub user_state: Account<'info, UserState>,
    #[account(
        init,
        payer = payer,
        space = 8 + NftId::LEN,
        seeds = [ b"nft_state-pubkey", nft_mint.key().as_ref() ],
        bump,
    )]
    pub pubkey_to_nft_id: Account<'info, NftId>,

    #[account(
        init,
        payer = payer,
        space = 8 + NftState::LEN,
        seeds = [ b"nft_state-id", (main_state.to_total_nft_supply() + 1).to_string().as_bytes()],
        bump,
    )]
    pub nft_id_to_nft_state: Account<'info, NftState>,

    // -------------------------
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub nft_token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    // -------------------------
}

impl<'info> CreateNft<'info> {
    #[inline]
    fn mint_nft(&mut self, mint_args: MintArgs) -> Result<()> {
        let cpi_mint_to = CpiContext::new(
            self.token_program.to_account_info(),
            spl::token::MintTo {
                mint: self.nft_mint.to_account_info(),
                authority: self.payer.to_account_info(),
                to: self.nft_token_account.to_account_info(),
            },
        );

        spl::token::mint_to(cpi_mint_to, 1)?;

        let MintArgs {
            creator_key,
            metadata_uri,
            name,
            symbol,
            royalty,
        } = mint_args;

        let account_info = vec![
            self.metadata.to_account_info(),
            self.nft_mint.to_account_info(),
            self.payer.to_account_info(),
            self.payer.to_account_info(),
            self.token_metadata_program.to_account_info(),
            self.token_program.to_account_info(),
            self.system_program.to_account_info(),
            self.rent.to_account_info(),
        ];

        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: self.payer.key(),
                verified: true,
                share: 0,
            },
        ];

        program::invoke(
            &create_metadata_accounts_v2(
                self.token_metadata_program.key(),
                self.metadata.key(),
                self.nft_mint.key(),
                self.payer.key(),
                self.payer.key(),
                self.payer.key(),
                name,
                symbol,
                metadata_uri,
                Some(creator),
                royalty * 100, // * 1%
                true,
                true,
                None,
                // Some(Collection {
                //     key: self.collection.key(),
                //     verified: true,
                // }),
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created!");
        let master_edition_infos = vec![
            self.master_edition.to_account_info(),
            self.nft_mint.to_account_info(),
            self.payer.to_account_info(),
            self.payer.to_account_info(),
            self.metadata.to_account_info(),
            self.token_metadata_program.to_account_info(),
            self.token_program.to_account_info(),
            self.system_program.to_account_info(),
            self.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        program::invoke(
            &create_master_edition_v3(
                self.token_metadata_program.key(),
                self.master_edition.key(),
                self.nft_mint.key(),
                self.payer.key(),
                self.payer.key(),
                self.metadata.key(),
                self.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted! {}", self.nft_mint.key());

        self.main_state.increment_nft_supply();

        Ok(())
    }

    #[inline]
    pub fn run(&mut self, mint_args: MintArgs) -> Result<()> {
        self.main_state.validate_signer(&self.payer)?; // whitelist only

        let initial_owner = mint_args.creator_key.clone();

        self.mint_nft(mint_args)?; // mint

        let nft_pubkey = self.nft_mint.key();
        let nft_index = self.main_state.to_total_nft_supply();

        let nft_state = self
            .main_state
            .as_program_token_account_info()
            .into_nft_state(nft_index, nft_pubkey, initial_owner);

        self.nft_id_to_nft_state.set_inner(nft_state);

        self.pubkey_to_nft_id.id = nft_index;
        self.pubkey_to_nft_id.bump = self.nft_id_to_nft_state.bump.pubkey_to_id_bump;

        Ok(())
    }
}
