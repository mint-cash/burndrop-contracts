use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct LiquidityPair {
    pub x: Uint128,
    pub y: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct SwapRound {
    pub id: u64,
    pub start_time: u64,
    pub end_time: u64,

    pub oppamint_liquidity: LiquidityPair,
    pub ancs_liquidity: LiquidityPair,

    pub oppamint_weight: u32,
    pub ancs_weight: u32,
}

impl SwapRound {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.start_time >= self.end_time {
            return Err(ContractError::InvalidRoundTime {
                start_time: self.start_time,
                end_time: self.end_time,
            });
        }

        Ok(())
    }
}
