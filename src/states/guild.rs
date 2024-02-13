use crate::states::user::User;
use cosmwasm_std::Uint128;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Guild {
    pub slug: String,
    pub name: String,
    pub users: Vec<User>,
    pub burned_uusd: Uint128,
}

pub const GUILD: Map<u64, Guild> = Map::new("guild");
