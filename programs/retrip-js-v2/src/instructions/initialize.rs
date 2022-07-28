use crate::{common::*, states::MainState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + MainState::LEN
    )]
    pub main_state: Account<'info, MainState>,
    #[account(mut)]
    pub program_mint: Account<'info, token::Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [ program_mint.key().as_ref() ],
        bump,
        token::mint = program_mint,
        token::authority = program_token_account,
    )]
    pub program_token_account: Account<'info, token::TokenAccount>,
}

impl<'info> Initialize<'info> {
    #[inline]
    fn new_main_state(&self) -> MainState {
        let program_mint = self.program_mint.key();
        let program_token_account = self.program_token_account.key();
        let creator = self.payer.key();
        let program_token_account_bump = find_bump(&[program_mint.as_ref()]);

        MainState::new(
            &TokenAccountInfo::new(
                program_mint,
                program_token_account,
                program_token_account_bump,
                creator,
            ),
            &self.payer,
        )
    }
    #[inline]
    fn mint_initial_tokens(&mut self, amount: u64) -> Result<()> {
        let seeds = self.main_state.as_program_token_account_info().as_seeds();
        let signer = [&seeds[..]];
        let cpi_mint_to = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            spl::token::MintTo {
                mint: self.program_mint.to_account_info(),
                authority: self.program_token_account.to_account_info(),
                to: self.program_token_account.to_account_info(),
            },
            &signer,
        );

        spl::token::mint_to(cpi_mint_to, amount)
    }
    #[inline]
    pub fn run(&mut self, amount: u64) -> Result<()> {
        self.main_state.set_inner(self.new_main_state());
        msg!("create_mint_and_vault begin");
        self.mint_initial_tokens(amount)
    }
}
