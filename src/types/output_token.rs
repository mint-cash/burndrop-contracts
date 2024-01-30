use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Copy, Eq)]
#[cw_serde(rename_all = "snake_case")]
pub enum OutputToken {
    OppaMINT,
    ANCS,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OutputTokenMap {
    pub oppamint: Uint128,
    pub ancs: Uint128,
}
impl OutputTokenMap {
    pub fn get(&self, token: OutputToken) -> Uint128 {
        match token {
            OutputToken::OppaMINT => self.oppamint,
            OutputToken::ANCS => self.ancs,
        }
    }

    pub fn set(&mut self, token: OutputToken, amount: Uint128) {
        match token {
            OutputToken::OppaMINT => self.oppamint = amount,
            OutputToken::ANCS => self.ancs = amount,
        }
    }

    pub fn add(&mut self, token: OutputToken, amount: Uint128) {
        match token {
            OutputToken::OppaMINT => self.oppamint += amount,
            OutputToken::ANCS => self.ancs += amount,
        }
    }

    pub fn sub(&mut self, token: OutputToken, amount: Uint128) {
        match token {
            OutputToken::OppaMINT => self.oppamint -= amount,
            OutputToken::ANCS => self.ancs -= amount,
        }
    }
}
