#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use std::collections::HashMap;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetBurnInfoResponse, InstantiateMsg, QueryMsg};
use crate::states::config::{Config, CONFIG};
use crate::states::state::{State, STATE};

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
    };
    config.save(deps.storage)?;

    let state = State {
        burned_uusd_by_user: HashMap::new(),
        slots_by_user: HashMap::new(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("owner", info.sender),
        attr("initial_slot_size", msg.initial_slot_size.to_string()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BurnTokens { amount } => burn_uusd(deps, env, info, amount),

        ExecuteMsg::UpdateSlotSize { slot_size } => {
            // Ensure only the owner can update the slot size.
            let mut config = Config::load(deps.storage)?;
            if info.sender != config.owner {
                return Err(ContractError::Unauthorized {});
            }

            config.update_slot_size(deps.storage, slot_size)?;
            Ok(Response::new().add_attribute("action", "update_slot_size"))
        }
    }
}

fn burn_uusd(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut state: State = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    let previously_burned: Uint128 = state
        .burned_uusd_by_user
        .get(&info.sender.to_string())
        .copied()
        .unwrap_or_default();

    // slots_by_user(address) * config.slot_size
    let capped_uusd_by_user: Uint128 = {
        let slots: Uint128 = {
            state
                .slots_by_user
                .get(&info.sender.to_string())
                .copied()
                .unwrap_or_default()
        };
        config.slot_size * slots
    };

    if amount + previously_burned > capped_uusd_by_user {
        return Err(ContractError::CapExceeded {});
    }

    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount.u128(), "uusd")],
    };

    state
        .burned_uusd_by_user
        .insert(info.sender.to_string(), previously_burned + amount);
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_message(burn_msg).add_attributes(vec![
        attr("action", "burn_uusd"),
        attr("amount", amount.to_string()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetBurnInfo { address } => to_json_binary(&query_burn_info(deps, address)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(Config {
        owner: config.owner,
        slot_size: config.slot_size,
    })
}

fn query_burn_info(deps: Deps, address: String) -> StdResult<GetBurnInfoResponse> {
    let state: State = STATE.load(deps.storage)?;
    let config: Config = CONFIG.load(deps.storage)?;

    let previously_burned: Uint128 = state
        .burned_uusd_by_user
        .get(&address)
        .copied()
        .unwrap_or_default();
    let slots: Uint128 = state
        .slots_by_user
        .get(&address)
        .copied()
        .unwrap_or_default();
    let cap: Uint128 = config.slot_size * slots;

    Ok(GetBurnInfoResponse {
        burned: previously_burned,
        burnable: cap - previously_burned,
        cap,
        slots,
        slot_size: config.slot_size,
    })
}
