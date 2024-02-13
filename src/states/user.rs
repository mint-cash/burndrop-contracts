use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::output_token::OutputTokenMap;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct User {
    pub address: Addr,

    // number of users who registered this user for the first referral code
    pub referral_a: u32,

    pub burned_uusd: Uint128, // swapped_in
    pub swapped_out: OutputTokenMap<Uint128>,
    pub first_referrer: Option<Addr>,

    pub guild_id: u64,
    pub guild_contributed_uusd: Uint128,
}

impl User {
    // slots = 511q + 2^(r+1) - 1, where a = 9q + r (0 <= r < 9)
    pub fn slots(&self) -> Uint128 {
        let q = (self.referral_a / 9) as u128;
        let r = self.referral_a % 9;
        let slots = 511 * q + 2_u128.pow(r + 1) - 1;
        Uint128::new(slots)
    }
}

pub const USER: Map<Addr, User> = Map::new("user");
