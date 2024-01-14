use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub burned_uusd_by_user: HashMap<String, Uint128>,
    pub slots_by_user: HashMap<String, Uint128>,

    pub referral_count_by_user: HashMap<String, u32>,
    pub second_referrer_registered: HashMap<String, bool>,

    pub total_swapped: Uint128,
    pub total_claimed: Uint128,

    pub x_liquidity: Uint128,
    pub y_liquidity: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
