use crate::{
    common::*,
    states::{MainState, UserId, UserState},
};

#[derive(Accounts)]
pub struct CreateUser<'info> {
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

    #[account(
        init,
        payer = payer,
        space = 8 + UserId::LEN,
        seeds = [ b"user_state-pubkey", user_pubkey.key().as_ref() ],
        bump,
    )]
    pub pubkey_to_user_id: Account<'info, UserId>,

    #[account(
        init,
        payer = payer,
        space = 8 + UserState::LEN,
        seeds = [ b"user_state-id", (main_state.to_total_user_supply() + 1).to_string().as_bytes()],
        bump,
    )]
    pub user_id_to_user_state: Account<'info, UserState>,

    #[account(mut, address = main_state.as_program_mint().key())]
    pub program_mint: Account<'info, spl::token::Mint>,

    #[account(
        mut,
        seeds = [ main_state.as_program_mint().as_ref() ],
        bump = main_state.as_program_token_account_bump(),
        token::mint = program_mint,
        token::authority = program_token_account,
    )]
    pub program_token_account: Account<'info, spl::token::TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [ main_state.as_program_mint().key().as_ref(), user_pubkey.key().as_ref() ],
        bump,
        token::mint = program_mint,
        token::authority = program_token_account,
    )]
    pub user_token_account: Box<Account<'info, spl::token::TokenAccount>>,
}

impl<'info> CreateUser<'info> {
    #[inline]
    pub fn run(&mut self) -> Result<()> {
        self.main_state.validate_signer(&self.payer)?; // whitelist only

        let user_pubkey = self.user_pubkey.key();
        let user_index = self.main_state.increment_user_supply();

        let user_state = self
            .main_state
            .as_program_token_account_info()
            .into_user_state(user_index, &user_pubkey);

        self.user_id_to_user_state.set_inner(user_state);

        self.pubkey_to_user_id.id = user_index;
        self.pubkey_to_user_id.bump = self.user_id_to_user_state.bump.pubkey_to_id_bump;

        Ok(())
    }
}
