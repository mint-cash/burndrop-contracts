use crate::contract::Response;
use crate::error::ContractError;
use crate::states::config::CONFIG;
use crate::states::overridden_rounds::OVERRIDDEN_ROUNDS;
use crate::types::overridden_round::OverriddenRound;
use classic_bindings::TerraQuery;
use cosmwasm_std::{DepsMut, MessageInfo, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn validate_rounds(rounds: &mut Vec<OverriddenRound>) -> Result<(), ContractError> {
    // Ensure the rounds are sorted by start time, and not overlapping.
    for i in 0..rounds.len() - 1 {
        if rounds[i].end_time > rounds[i + 1].start_time {
            return Err(ContractError::InvalidRounds {});
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UpdateOverriddenRoundParams {
    pub index: u64,

    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub slot_size: Uint128,
}
pub fn update_overridden_round(
    deps: DepsMut<TerraQuery>,
    info: MessageInfo,
    UpdateOverriddenRoundParams {
        index,
        start_time,
        end_time,
        slot_size,
    }: UpdateOverriddenRoundParams,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    let overridden_rounds = OVERRIDDEN_ROUNDS.load(deps.storage)?;

    let round = overridden_rounds.rounds[index as usize].clone();
    let updated_round = OverriddenRound {
        start_time: start_time.unwrap_or(round.start_time),
        end_time: end_time.unwrap_or(round.end_time),
        slot_size,
    };
    updated_round.validate()?;

    let mut rounds = overridden_rounds.rounds.clone();
    rounds[index as usize] = updated_round;
    validate_rounds(&mut rounds)?;

    OVERRIDDEN_ROUNDS.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.rounds = rounds;
        Ok(state)
    })?;

    Ok(Response::new())
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CreateOverriddenRoundParams {
    pub start_time: u64,
    pub end_time: u64,
    pub slot_size: Uint128,
}
pub fn create_overridden_round(
    deps: DepsMut<TerraQuery>,
    info: MessageInfo,
    CreateOverriddenRoundParams {
        start_time,
        end_time,
        slot_size,
    }: CreateOverriddenRoundParams,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    let overridden_rounds = OVERRIDDEN_ROUNDS.load(deps.storage)?;

    let round = OverriddenRound {
        start_time,
        end_time,
        slot_size,
    };
    round.validate()?;

    let mut rounds = overridden_rounds.rounds.clone();
    rounds.push(round);
    validate_rounds(&mut rounds)?;

    OVERRIDDEN_ROUNDS.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.rounds = rounds;
        Ok(state)
    })?;

    Ok(Response::new())
}
