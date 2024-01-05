use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use crate::states::config::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_slot_size: Uint128,
}

#[cw_serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BurnTokens { amount: Uint128 },
    UpdateSlotSize { slot_size: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},

    #[returns(GetBurnInfoResponse)]
    GetBurnInfo { address: String },
}

#[cw_serde]
pub struct GetBurnInfoResponse {
    pub burned: Uint128,
    pub burnable: Uint128,
    pub cap: Uint128,
    pub slots: Uint128,
    pub slot_size: Uint128,
}
