pub use super::*;
use ink_prelude::vec::Vec;

pub struct timeLockData{
    token: AccountId,
    beneficiary: AccountId,
    release_time: Timestamp,
} 