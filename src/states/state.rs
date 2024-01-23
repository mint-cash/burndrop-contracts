use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::output_token::OutputTokenMap;
use crate::types::swap_round::SwapRound;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_swapped: OutputTokenMap,
    pub total_claimed: OutputTokenMap,

    pub x_liquidity: Uint128,
    pub y_liquidity: OutputTokenMap,

    pub rounds: Vec<SwapRound>,
}

pub const STATE: Item<State> = Item::new("state");
