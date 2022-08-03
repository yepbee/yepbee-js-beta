use crate::common::*;

#[error_code]
pub enum Errors {
    // #[msg("Invalid H3 Position")]
    // InvalidPosition,
    #[msg("Each Pubkeys Must Be Unique")]
    DuplicatedPubkeys,
    #[msg("The Target Is Not In The Whitelist")]
    TargetIsNotInWhiteList,
    #[msg("Not In The Whitelist")]
    NotInWhiteList,
    #[msg("Not Enough Majority Number")]
    NotEnoughMajorityNumber,
    #[msg("At Least One Must Exist")]
    AtLeaseOneMustExist,
    #[msg("Overflow Occurs")]
    OverflowOccurs,
    #[msg("Out Of Bounds")]
    OutOfBounds,
    #[msg("Couldn't Find")]
    CouldntFind,
    #[msg("Unexpect Kind")]
    UnexpectKind,
    #[msg("Parsing Error")]
    Parsing,
    #[msg("Couldn't Get Time")]
    Time,
    #[msg("Target Value Is Too Small")]
    TooSmall,
    #[msg("The President Expiration Date Still Not Over")]
    PresidentDateNotOver,
    #[msg("The Nft Is Already Liked")]
    IsAlreadyLiked,
}
