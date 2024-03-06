use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::overridden_round::OverriddenRound;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OverriddenRounds {
    rounds: Vec<OverriddenRound>,
}

impl OverriddenRounds {
    // pub fn current_round(&self, now: u64) -> Option<(&OverriddenRound, u64)> {
    //     self.rounds
    //         .iter()
    //         .enumerate()
    //         .find(|(_, r)| r.start_time <= now && now <= r.end_time)
    //         .map(|(i, r)| (r, i as u64))
    // }

    pub fn recent_active_round(&self, now: u64) -> Option<(&OverriddenRound, u64)> {
        self.rounds
            .iter()
            .enumerate()
            // .rfind(|r: &&OverriddenRound| r.start_time <= now)
            .find(|(_, r)| r.start_time <= now)
            .map(|(i, r)| (r, i as u64))
    }

    pub fn is_active(&self, round: Option<&OverriddenRound>, now: u64) -> bool {
        match (round, now) {
            (Some(round), now) => round.start_time <= now && now <= round.end_time,
            (None, _) => false,
        }
    }
}

pub const OVERRIDDEN_ROUNDS: Item<OverriddenRounds> = Item::new("overridden-rounds");
pub const OVERRIDDEN_BURNED_UUSD: Map<(u64, Addr), Uint128> = Map::new("overridden-burned-uusd");
