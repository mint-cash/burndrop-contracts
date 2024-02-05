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
    pub first_referrer: Option<Addr>,
}

impl User {
    // slots = 511q + 2^(r+1) + b + c - 1, where a = 9q + r (0 <= r < 9)
    pub fn slots(&self) -> Uint128 {
        let q = (self.referral_a / 9) as u128;
        let r = self.referral_a % 9;
        let b = u128::from(self.referral_b);
        let c = u128::from(self.referral_c);
        let slots = 511 * q + 2_u128.pow(r + 1) + b + c - 1;
        Uint128::new(slots)
    }
}

pub const USER: Map<Addr, User> = Map::new("user");
