use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::swap_round::SwapRound;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_swapped: Uint128,
    pub total_claimed: Uint128,

    pub x_liquidity: Uint128,
    pub y_liquidity: Uint128,

    pub rounds: Vec<SwapRound>,
}

pub const STATE: Item<State> = Item::new("state");
