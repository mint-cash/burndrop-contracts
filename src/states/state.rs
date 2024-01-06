use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub burned_uusd_by_user: HashMap<String, Uint128>,
    pub slots_by_user: HashMap<String, Uint128>,
}

pub const STATE: Item<State> = Item::new("state");
