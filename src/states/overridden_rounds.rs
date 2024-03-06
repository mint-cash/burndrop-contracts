use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::overridden_round::OverriddenRound;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OverriddenRounds {
    rounds: Vec<OverriddenRound>,
}

impl OverriddenRounds {
    // pub fn recent_active_round(&self, now: u64) -> Option<&OverriddenRound> {
    //     self.rounds.iter().rfind(|r| r.start_time <= now)
    // }
}

pub const OVERRIDDEN_ROUNDS: Item<OverriddenRounds> = Item::new("overridden-rounds");
