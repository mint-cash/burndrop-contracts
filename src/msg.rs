use crate::states::config::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_slot_size: Uint128,
    pub sale_amount: Uint128,

    pub x_liquidity: Uint128,
    pub y_liquidity: Uint128,
}

#[cw_serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BurnTokens { amount: Uint128, referrer: String },
    Register2ndReferrer { referrer: String },
    UpdateSlotSize { slot_size: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},

    #[returns(UserInfoResponse)]
    UserInfo { address: String },

    #[returns(PriceResponse)]
    CurrentPrice {},

    #[returns(SimulateBurnResponse)]
    SimulateBurn { amount: Uint128 },
}

#[cw_serde]
pub struct UserInfoResponse {
    pub burned: Uint128,
    pub burnable: Uint128,
    pub cap: Uint128,
    pub slots: Uint128,
    pub slot_size: Uint128,
    pub swapped_out: Uint128,
}

#[cw_serde]
pub struct PriceResponse {
    pub price: Decimal,
}

#[cw_serde]
pub struct SimulateBurnResponse {
    pub swapped_out: Uint128,
    pub virtual_slippage: Uint128,
    pub final_amount: Uint128,
}
