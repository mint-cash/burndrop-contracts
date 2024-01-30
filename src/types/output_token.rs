use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, SubAssign};

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

impl<T: AddAssign + SubAssign> OutputTokenMap<T> {
    pub fn get(&self, token: OutputToken) -> &T {
        match token {
            OutputToken::OppaMINT => &self.oppamint,
            OutputToken::Ancs => &self.ancs,
        }
    }

    pub fn set(&mut self, token: OutputToken, amount: T) {
        match token {
            OutputToken::OppaMINT => self.oppamint = amount,
            OutputToken::Ancs => self.ancs = amount,
        }
    }

    pub fn add(&mut self, token: OutputToken, amount: T) {
        match token {
            OutputToken::OppaMINT => self.oppamint += amount,
            OutputToken::Ancs => self.ancs += amount,
        }
    }

    pub fn sub(&mut self, token: OutputToken, amount: T) {
        match token {
            OutputToken::OppaMINT => self.oppamint -= amount,
            OutputToken::Ancs => self.ancs -= amount,
        }
    }
}
