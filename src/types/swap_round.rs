use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::types::output_token::OutputToken;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct SwapRound {
    pub id: u64,
    pub start_time: u64,
    pub end_time: u64,

    pub output_token: OutputToken,

    pub x_liquidity: Uint128,
    pub y_liquidity: Uint128,
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
