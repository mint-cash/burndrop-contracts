use cosmwasm_std::{Addr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const KEY_CONFIG: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub slot_size: Uint128,
}

impl Config {
    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        ReadonlySingleton::new(storage, KEY_CONFIG).load()
    }

    pub fn save(storage: &mut dyn Storage, data: &Self) -> StdResult<()> {
        Singleton::new(storage, KEY_CONFIG).save(data)
    }

    pub fn update_slot_size(storage: &mut dyn Storage, new_slot_size: Uint128) -> StdResult<()> {
        let mut config: Config = Self::load(storage)?;
        config.slot_size = new_slot_size;
        Self::save(storage, &config)
    }
}
