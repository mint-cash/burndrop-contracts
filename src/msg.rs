use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

use crate::executions::round::UpdateRoundParams;
use crate::states::config::Config;
use crate::types::common::OrderBy;
use crate::types::output_token::OutputTokenMap;
use crate::types::swap_round::SwapRound;

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_slot_size: Uint128,

    pub rounds: Vec<SwapRound>,

    pub max_query_limit: u32,
    pub default_query_limit: u32,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BurnUusd {
        amount: Uint128,
        referrer: Option<String>,
        min_amount_out: Option<OutputTokenMap<Uint128>>,
    },
    RegisterStartingUser {
        user: String,
    },
    UpdateSlotSize {
        slot_size: Uint128,
    },
    CreateRound {
        round: SwapRound,
    },
    UpdateRound {
        params: UpdateRoundParams,
    },
    DeleteRound {
        id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},

    #[returns(UserInfoResponse)]
    UserInfo { address: String },

    #[returns(UsersInfoResponse)]
    UsersInfo {
        start: Option<String>,
        limit: Option<u32>,
        order: Option<OrderBy>,
    },

    #[returns(PriceResponse)]
    CurrentPrice {},

    #[returns(SimulateBurnResponse)]
    SimulateBurn { amount: Uint128 },

    #[returns(RoundsResponse)]
    Rounds {},
}

#[cw_serde]
pub struct UserInfoResponse {
    pub burned: Uint128,
    pub burnable: Uint128,
    pub cap: Uint128,
    pub slots: Uint128,
    pub slot_size: Uint128,
    pub swapped_out: OutputTokenMap<Uint128>,
}

#[cw_serde]
pub struct UsersInfoResponse {
    pub users: Vec<(String, UserInfoResponse)>,
}

#[cw_serde]
pub struct PriceResponse {
    pub price: OutputTokenMap<Decimal>,
}

#[cw_serde]
pub struct SimulateBurnResponse {
    pub swapped_out: OutputTokenMap<Uint128>,
    pub final_amount: Uint128,
}

#[cw_serde]
pub struct RoundsResponse {
    pub rounds: Vec<SwapRound>,
}
