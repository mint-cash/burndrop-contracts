#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::error::ContractError;
use crate::executions::round::{
    create_round, delete_round, sort_and_validate_rounds, update_round,
};
use crate::executions::swap::burn_uusd;
use crate::executions::user::{register_2nd_referrer, register_starting_user};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{
    query_config, query_current_price, query_rounds, query_simulate_burn, query_user,
};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE};
use crate::types::output_token::OutputTokenMap;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:burndrop-contracts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.clone(),
        slot_size: msg.initial_slot_size,
        sale_amount: msg.sale_amount,
    };
    CONFIG.save(deps.storage, &config)?;

    // Ensure the rounds are sorted by start time, and not overlapping.
    let mut rounds = msg.rounds.clone();

    sort_and_validate_rounds(&mut rounds)?;

    let state = State {
        total_claimed: OutputTokenMap {
            oppamint: Uint128::zero(),
            ancs: Uint128::zero(),
        },
        total_swapped: OutputTokenMap {
            oppamint: Uint128::zero(),
            ancs: Uint128::zero(),
        },
        rounds,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("owner", info.sender),
        attr("initial_slot_size", msg.initial_slot_size.to_string()),
    ]))
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let stored = get_contract_version(deps.storage)?;
    let storage_contract_name = stored.contract;

    if storage_contract_name != CONTRACT_NAME {
        return Err(ContractError::Std(StdError::generic_err(
            "Can only upgrade from same contract type",
        )));
    }

    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_contract_version: Version = stored.version.parse()?;
    if storage_contract_version >= version {
        return Err(ContractError::Std(StdError::generic_err(
            "Cannot upgrade from a newer contract version",
        )));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BurnTokens { amount, referrer } => burn_uusd(deps, env, info, amount, referrer),
        ExecuteMsg::RegisterStartingUser { user } => register_starting_user(deps, info, user),
        ExecuteMsg::Register2ndReferrer { referrer } => register_2nd_referrer(deps, info, referrer),

        ExecuteMsg::UpdateSlotSize { slot_size } => {
            // Ensure only the owner can update the slot size.
            let config = CONFIG.load(deps.storage)?;
            if info.sender != config.owner {
                return Err(ContractError::Unauthorized {});
            }

            CONFIG.update(deps.storage, |mut config| -> StdResult<Config> {
                config.slot_size = slot_size;
                Ok(config)
            })?;

            Ok(Response::new().add_attribute("action", "update_slot_size"))
        }
        ExecuteMsg::CreateRound { round } => create_round(deps, info, round),
        ExecuteMsg::UpdateRound { params } => update_round(deps, env, info, params),
        ExecuteMsg::DeleteRound { id } => delete_round(deps, env, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::UserInfo { address } => to_json_binary(&query_user(deps, address)?),
        QueryMsg::CurrentPrice {} => to_json_binary(&query_current_price(deps, env)?),
        QueryMsg::SimulateBurn { amount } => {
            to_json_binary(&query_simulate_burn(deps, env, amount)?)
        }
        QueryMsg::Rounds {} => to_json_binary(&query_rounds(deps)?),
    }
}
