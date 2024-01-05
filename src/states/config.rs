use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &str = "config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub slot_size: Uint128,
}

pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

impl Config {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        CONFIG.save(storage, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Config> {
        CONFIG.load(storage)
    }

    pub fn update_slot_size(&mut self, storage: &mut dyn Storage, new_slot_size: Uint128) -> StdResult<()> {
        self.slot_size = new_slot_size;
        self.save(storage)
    }
}
