use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

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
    #[returns(GetBurnInfoResponse)]
    GetBurnInfo { address: String },
}

#[cw_serde]
pub struct GetBurnInfoResponse {
    pub burned: Uint128,
    pub remaining_cap: Uint128,
}
