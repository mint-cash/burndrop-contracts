use cosmwasm_std::Uint128;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
    pub swapped_out_claimed: Uint128,
}

pub const USER: Map<&[u8], User> = Map::new("user");
