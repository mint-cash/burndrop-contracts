use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

use crate::types::output_token::OutputToken;

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

    #[error("Referrer not provided")]
    ReferrerNotProvided {},

    #[error("Referrer already first referrer")]
    ReferrerAlreadyFirstReferrer {},

    #[error("Referrer is self")]
    ReferrerIsSelf {},

    #[error("Overflow")]
    Overflow {},

    #[error("Swap: zero amount")]
    NotAllowZeroAmount {},

    #[error("Swap: not allow other denoms")]
    NotAllowOtherDenoms { denom: String },

    #[error("Swap: available cap exceeded")]
    AvailableCapExceeded { available: Uint128 },

    #[error("Swap: attempted division by zero")]
    DivisionByZeroError {},

    #[error("Swap: under min_amount_out")]
    UnderMinAmountOut {},

    #[error("Swap: invalid rounds")]
    InvalidRounds {},

    #[error("Swap: invalid round time")]
    InvalidRoundTime { start_time: u64, end_time: u64 },

    #[error("Swap: round not found")]
    RoundNotFound { round_id: u64 },

    #[error("Swap: round id already exists")]
    RoundIdAlreadyExists { round_id: u64 },

    #[error("Swap: no active executions round")]
    NoActiveSwapRound {},

    #[error("Swap: no liquidity for {token:?}")]
    NoLiquidity { token: OutputToken },

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<ContractError> for StdError {
    fn from(error: ContractError) -> Self {
        StdError::generic_err(error.to_string())
    }
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
