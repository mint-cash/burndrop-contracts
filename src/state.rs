use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {

    pub owner: Addr,
    pub user_caps: HashMap<String, (Uint128, Uint128)>,
}

pub const STATE: Item<State> = Item::new("state");
