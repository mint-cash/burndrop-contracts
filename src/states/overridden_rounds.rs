use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::overridden_round::OverriddenRound;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OverriddenRounds {
    rounds: Vec<OverriddenRound>,
}

impl OverriddenRounds {
    pub fn current_round(&self, now: u64) -> Option<&OverriddenRound> {
        self.rounds
            .iter()
            .rfind(|r| r.start_time <= now && now <= r.end_time)
    }
}

pub const OVERRIDDEN_ROUNDS: Item<OverriddenRounds> = Item::new("overridden-rounds");
pub const OVERRIDDEN_BURNED_UUSD: Map<u64, Uint128> = Map::new("overridden-burned-uusd");
