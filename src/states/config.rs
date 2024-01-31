use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &str = "config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub slot_size: Uint128,
    pub max_query_limit: u32,
    pub default_query_limit: u32,
}

pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
