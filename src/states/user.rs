use cosmwasm_std::Uint128;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct User {
    pub slots: Uint128,
    pub referral_count: Uint128,
    pub second_referrer_registered: bool,

    pub burned_uusd: Uint128, // swapped_in
    pub swapped_out: Uint128,
}

pub const USER: Map<&[u8], User> = Map::new("user");
