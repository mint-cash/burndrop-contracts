use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapRound {
    pub start_time: u64,
    pub end_time: u64,

    pub input_token: String,
}
