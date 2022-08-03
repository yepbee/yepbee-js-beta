mod common;
use common::*;

mod states;
// use state::*;

mod payloads;
// use payload::*;

mod instructions;
use instructions::*;

pub const PROGRAM_ID: &str = "APDcLngZjQCDX5XyTMaw3H9ykwgyevmE9naXh5wXX3xg"; // #lib.rs
declare_id!("APDcLngZjQCDX5XyTMaw3H9ykwgyevmE9naXh5wXX3xg");

#[program]
pub mod yepbee_js_beta {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.run(amount)
    }
    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        ctx.accounts.run()
    }
    pub fn create_nft(ctx: Context<CreateNft>, mint_args: MintArgs) -> Result<()> {
        ctx.accounts.run(mint_args)
    }
    pub fn suggest(ctx: Context<Suggest>, suggestion: Suggestion) -> Result<()> {
        ctx.accounts.run(suggestion)
    }
    pub fn faucet(ctx: Context<Faucet>) -> Result<()> {
        ctx.accounts.run()
    }
}
