use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OverriddenRound {
    pub start_time: u64,
    pub end_time: u64,

    pub slot_size: Uint128,
}

impl OverriddenRound {
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
