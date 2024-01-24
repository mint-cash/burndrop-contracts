use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::output_token::OutputTokenMap;
use crate::types::swap_round::SwapRound;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub total_swapped: OutputTokenMap,
    pub total_claimed: OutputTokenMap,

    pub rounds: Vec<SwapRound>,
}

impl State {
    pub fn recent_active_round(&self, now: u64) -> Option<&SwapRound> {
        self.rounds.iter().rfind(|r| r.start_time <= now)
    }
}

pub const STATE: Item<State> = Item::new("state");
