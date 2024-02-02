use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::output_token::OutputTokenMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct User {
    // number of users who registered this user for the first referral code
    pub referral_a: u32,
    // did someone register this user for the second referral code
    pub referral_b: bool,
    // did this user register someone for the second referral code
    pub referral_c: bool,

    pub burned_uusd: Uint128, // swapped_in
    pub swapped_out: OutputTokenMap<Uint128>,
}

impl User {
    // slots = 2^a + b + c
    pub fn slots(&self) -> Uint128 {
        let mut slots = 2u128.pow(self.referral_a);
        if self.referral_b {
            slots += 1;
        }
        if self.referral_c {
            slots += 1;
        }
        Uint128::new(slots)
    }
}

pub const USER: Map<Addr, User> = Map::new("user");
