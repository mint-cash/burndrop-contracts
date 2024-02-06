use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{OverflowError, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Copy, Eq)]
#[cw_serde(rename_all = "snake_case")]
pub enum OutputToken {
    OppaMINT,
    Ancs,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OutputTokenMap<T> {
    pub oppamint: T,
    pub ancs: T,
}

impl OutputTokenMap<Uint128> {
    pub fn checked_sub(&self, rhs: Self) -> Result<Self, ContractError> {
        let oppamint = self.oppamint.checked_sub(rhs.oppamint)?;
        let ancs = self.ancs.checked_sub(rhs.ancs)?;
        Ok(OutputTokenMap { oppamint, ancs })
    }
}

impl From<OverflowError> for ContractError {
    fn from(_: OverflowError) -> Self {
        Self::Overflow {}
    }
}

impl ops::AddAssign for OutputTokenMap<Uint128> {
    fn add_assign(&mut self, rhs: Self) {
        self.oppamint += rhs.oppamint;
        self.ancs += rhs.ancs;
    }
}
