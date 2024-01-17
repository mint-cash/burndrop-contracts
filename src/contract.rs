#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{burn_uusd, register_2nd_referrer, register_starting_user};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_config, query_current_price, query_simulate_burn, query_user};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE};

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

    let state = State {
        x_liquidity: msg.x_liquidity,
        y_liquidity: msg.y_liquidity,
        total_claimed: Uint128::zero(),
        total_swapped: Uint128::zero(),
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
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::UserInfo { address } => to_json_binary(&query_user(deps, address)?),
        QueryMsg::CurrentPrice {} => to_json_binary(&query_current_price(deps)?),
        QueryMsg::SimulateBurn { amount } => to_json_binary(&query_simulate_burn(deps, amount)?),
    }
}
