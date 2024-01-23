use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::output_token::OutputToken;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct SwapRound {
    pub start_time: u64,
    pub end_time: u64,

    pub output_token: OutputToken,
}
