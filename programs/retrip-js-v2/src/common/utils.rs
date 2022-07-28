use crate::common::*;

#[inline]
pub fn find_program_address(seeds: &[&[u8]]) -> (Pubkey, u8) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();
    Pubkey::find_program_address(seeds, &program_id)
}
#[inline]
pub fn find_bump(seeds: &[&[u8]]) -> u8 {
    find_program_address(seeds).1
}
#[inline]
pub fn find_token_account(mint_address: Pubkey, user_address: Pubkey) -> (Pubkey, u8) {
    find_program_address(&[mint_address.as_ref(), user_address.as_ref()])
}

#[inline]
pub fn get_current_unix_timestamp_as_hour() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp / 3600)
}
