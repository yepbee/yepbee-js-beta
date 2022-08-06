use crate::common::*;

#[inline]
pub fn find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}
#[inline]
pub fn find_bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    find_program_address(seeds, program_id).1
}
#[inline]
pub fn find_token_account(
    mint_address: &Pubkey,
    user_address: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    find_program_address(&[mint_address.as_ref(), user_address.as_ref()], program_id)
}

#[inline]
pub fn get_current_unix_timestamp_as_hour() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp / 3600)
}
