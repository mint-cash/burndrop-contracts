use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Cap exceeded")]
    CapExceeded {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Already registered")]
    AlreadyRegistered {},

    #[error("Referrer is not initialized")]
    ReferrerNotInitialized {},

    #[error("Swap: zero amount")]
    NotAllowZeroAmount {},

    #[error("Swap: not allow other denoms")]
    NotAllowOtherDenoms { denom: String },

    #[error("Swap: available cap exceeded")]
    AvailableCapExceeded { available: Uint128 },

    #[error("Swap: pool size exceeded")]
    PoolSizeExceeded { available: Uint128 },

    #[error("Swap: attempted division by zero")]
    DivisionByZeroError {},

    #[error("Swap: not started. (time: {start:?})")]
    SwapNotStarted { start: u64 },

    #[error("Swap: finished. (time: {end:?})")]
    SwapFinished { end: u64 },
}

impl From<ContractError> for StdError {
    fn from(error: ContractError) -> Self {
        StdError::generic_err(error.to_string())
    }
}
