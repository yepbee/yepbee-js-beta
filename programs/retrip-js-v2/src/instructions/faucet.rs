use crate::common::*;
use crate::states::{MainState, UserId, UserState};

#[derive(Accounts)]
pub struct Faucet<'info> {
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, spl::token::Token>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub main_state: Account<'info, MainState>,

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
        mut,
        seeds = [ b"user_state-pubkey", payer.key().as_ref() ],
        bump = pubkey_to_user_id.bump,
    )]
    pub pubkey_to_user_id: Account<'info, UserId>,

    #[account(
        mut,
        seeds = [ b"user_state-id", pubkey_to_user_id.id.to_string().as_bytes()],
        bump = user_id_to_user_state.bump.id_to_state_bump,
    )]
    pub user_id_to_user_state: Account<'info, UserState>,

    #[account(
        mut,
        seeds = [ main_state.as_program_mint().key().as_ref(), payer.key().as_ref() ],
        bump = user_id_to_user_state.token_account_info.to_bump(),
        token::mint = program_mint,
        token::authority = program_token_account,
    )]
    pub payer_token_account: Box<Account<'info, spl::token::TokenAccount>>,
}

impl<'info> Faucet<'info> {
    #[inline]
    pub fn run(&mut self) -> Result<()> {
        self.main_state.validate_signer(&self.payer)?; // whitelist only

        let seeds = self.user_id_to_user_state.token_account_info.as_seeds();
        let signer = [&seeds[..]];

        let cpi_transfer = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            spl::token::Transfer {
                from: self.program_token_account.to_account_info(),
                authority: self.program_token_account.to_account_info(),
                to: self.payer_token_account.to_account_info(),
            },
            &signer,
        );

        let current_balance = spl::token::accessor::amount(&self.token_program.to_account_info())?;

        spl::token::transfer(cpi_transfer, current_balance / 1000) // airdrop
    }
}
