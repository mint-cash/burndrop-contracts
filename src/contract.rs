#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{attr, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, coin};
use cw2::set_contract_version;
use std::collections::HashMap;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetBurnInfoResponse, InstantiateMsg, QueryMsg};
use crate::states::config::Config;
use crate::states::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:burndrop-contracts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        slot_size: Uint128::from(1000),
    };
    config.save(deps.storage)?;

    let state = State {
        user_caps: HashMap::new(), // Added from your new code
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
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
            let config = Config::load(deps.storage)?;
            if info.sender != config.owner {
                return Err(ContractError::Unauthorized {});
            }

            Config::update_slot_size(deps.storage, slot_size)?;
            Ok(Response::new().add_attribute("method", "update_slot_size"))
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
    let cap = state.user_caps.get(&info.sender.to_string()).copied().unwrap_or_default();

    if amount + cap.1 > cap.0 {
        return Err(ContractError::CapExceeded {});
    }

    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount.u128(), "uusd")],
    };

    state.user_caps.insert(info.sender.to_string(), (cap.0, cap.1 + amount));
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_message(burn_msg).add_attributes(vec![
        attr("action", "burn_tokens"),
        attr("amount", amount.to_string()),
        // Add additional attributes related to the swap deposit here
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBurnInfo { address } => to_json_binary(&query_burn_info(deps, address)?),
    }
}

fn query_burn_info(deps: Deps, address: String) -> StdResult<GetBurnInfoResponse> {
    let state: State = STATE.load(deps.storage)?;
    let cap = state.user_caps.get(&address).copied().unwrap_or_default();

    Ok(GetBurnInfoResponse {
        burned: cap.1,
        remaining_cap: cap.0 - cap.1,
    })
}
