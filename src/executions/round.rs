use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::states::config::CONFIG;
use crate::states::state::STATE;
use crate::types::output_token::OutputToken;
use crate::types::swap_round::SwapRound;

pub fn sort_and_validate_rounds(rounds: &mut Vec<SwapRound>) -> Result<(), ContractError> {
    // Ensure the rounds are sorted by start time, and not overlapping.
    rounds.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    for i in 0..rounds.len() - 1 {
        if rounds[i].end_time > rounds[i + 1].start_time {
            return Err(ContractError::InvalidRounds {});
        }
    }

    Ok(())
}

pub fn create_round(
    deps: DepsMut,
    info: MessageInfo,
    round: SwapRound,
) -> Result<Response, ContractError> {
    // Ensure only the owner can create a round.
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure the round is valid
    round.validate()?;

    // Ensure the id is unique.
    let mut rounds = STATE.load(deps.storage)?.rounds;
    if rounds.iter().any(|r| r.id == round.id) {
        return Err(ContractError::RoundIdAlreadyExists { round_id: round.id });
    }

    // Add the round to the state.
    rounds.push(round);

    // Ensure the rounds are sorted by start time, and not overlapping.
    sort_and_validate_rounds(&mut rounds)?;

    // Save the state.
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.rounds = rounds;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "create_round"))
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UpdateRoundParams {
    pub id: u64,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub output_token: Option<OutputToken>,
    pub x_liquidity: Option<Uint128>,
    pub y_liquidity: Option<Uint128>,
}

pub fn update_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    UpdateRoundParams {
        id,
        start_time,
        end_time,
        output_token,
        x_liquidity,
        y_liquidity,
    }: UpdateRoundParams,
) -> Result<Response, ContractError> {
    // Ensure only the owner can update a round.
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure the round exists.
    let mut rounds = STATE.load(deps.storage)?.rounds;
    let round_index = rounds
        .iter()
        .position(|r| r.id == id)
        .ok_or(ContractError::RoundNotFound { round_id: id })?;

    // Update the round.
    let mut round = rounds[round_index].clone();
    if let Some(start_time) = start_time {
        round.start_time = start_time;
    }
    if let Some(end_time) = end_time {
        round.end_time = end_time;
    }

    let now = env.block.time.seconds();
    if let Some(output_token) = output_token {
        // Cannot update output_token for active round
        if round.start_time <= now && now <= round.end_time {
            return Err(ContractError::CannotUpdateActiveRound {});
        }

        round.output_token = output_token;
    }
    if let Some(x_liquidity) = x_liquidity {
        // Cannot update x_liquidity for active round
        if round.start_time <= now && now <= round.end_time {
            return Err(ContractError::CannotUpdateActiveRound {});
        }

        round.x_liquidity = x_liquidity;
    }
    if let Some(y_liquidity) = y_liquidity {
        // Cannot update x_liquidity for active round
        if round.start_time <= now && now <= round.end_time {
            return Err(ContractError::CannotUpdateActiveRound {});
        }

        round.y_liquidity = y_liquidity;
    }

    // Ensure the round is valid
    round.validate()?;

    // Update the round in the state.
    rounds[round_index] = round;

    // Ensure the rounds are sorted by start time, and not overlapping.
    sort_and_validate_rounds(&mut rounds)?;

    // Save the state.
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.rounds = rounds;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "update_round"))
}

pub fn delete_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Ensure only the owner can delete a round.
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure the round exists.
    let mut rounds = STATE.load(deps.storage)?.rounds;
    let round_index = rounds
        .iter()
        .position(|r| r.id == id)
        .ok_or(ContractError::RoundNotFound { round_id: id })?;

    // Cannot delete active round
    let now = env.block.time.seconds();
    if rounds[round_index].start_time <= now && now <= rounds[round_index].end_time {
        return Err(ContractError::CannotDeleteActiveRound {});
    }

    // Delete the round.
    rounds.remove(round_index);

    // Save the state.
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.rounds = rounds;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "delete_round"))
}
