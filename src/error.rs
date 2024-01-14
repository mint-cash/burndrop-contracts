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
}
