use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub total_swapped: Uint128,
    pub total_claimed: Uint128,

    pub x_liquidity: Uint128,
    pub y_liquidity: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
