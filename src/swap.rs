use cosmwasm_std::{
    attr, to_binary, BankMsg, Coin, CosmosMsg, Decimal, DepsMut, Env, Fraction, MessageInfo,
    Response, Uint128, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Denom};
use pylon_utils::tax::deduct_tax;
use std::convert::TryFrom;

use crate::constants::EARN_LOCK_PERIOD;
use crate::error::ContractError;
use crate::states::config::Config;
use crate::states::state::State;
use crate::states::user::User;

pub struct DepositResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn deposit(deps: DepsMut, env: Env, info: MessageInfo) -> Result<DepositResult, ContractError> {
    let config = Config::load(deps.storage)?;
    let now = env.block.time.seconds();

    // if now < config.start {
    //     return Err(ContractError::SwapNotStarted {
    //         start: config.start,
    //     });
    // }
    // if config.finish < now {
    //     return Err(ContractError::SwapFinished {
    //         finish: config.finish,
    //     });
    // }

    // let input_token_denom = match config.input_token {
    //     Denom::Native(denom) => denom,
    //     Denom::Cw20(_) => panic!("input token as cw20 token not supported"),
    // };
    let input_token_denom = "uusd";

    // 1:1
    let swapped_in = info
        .funds
        .iter()
        .find(|c| c.denom == input_token_denom)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);
    if swapped_in.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    // if info.funds.len() > 1 {
    //     return Err(ContractError::NotAllowOtherDenoms {
    //         denom: input_token_denom,
    //     });
    // }

    let sender = &deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut user = User::load(deps.storage, sender);
    let mut state = State::load(deps.storage)?;

    // check whitelisted, or free to participate everyone
    if config.whitelist_enabled && !User::is_whitelisted(deps.storage, sender) {
        return Err(ContractError::NotAllowNonWhitelisted {
            address: info.sender.to_string(),
        });
    }

    if let Some(strategy) = config.deposit_cap_strategy {
        let (amount, unlimited) =
            strategy.available_cap_of(deps.querier, info.sender.to_string(), user.swapped_in);
        if !unlimited && swapped_in > amount {
            return Err(ContractError::AvailableCapExceeded { available: amount });
        }
    }

    let swapped_out = swapped_in * Uint128::from(config.price.denominator())
        / Uint128::from(config.price.numerator());
    if state.total_swapped + swapped_out > config.amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.amount - state.total_swapped,
        });
    }

    user.swapped_in += swapped_in;
    user.swapped_out += swapped_out;

    state.total_swapped += swapped_out;

    User::save(deps.storage, sender, &user)?;
    State::save(deps.storage, &state)?;

    // let deposit_response = Response::new().add_attributes(vec![
    //     attr("sender", info.sender.to_string()),
    //     attr("swapped_in", swapped_in.to_string()),
    //     attr("swapped_out", swapped_out.to_string()),
    // ]);

    let deposit_result = DepositResult {
        swapped_in,
        swapped_out,
    };
    Ok(deposit_result)
}

pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> super::ExecuteResult {
    let config = Config::load(deps.storage)?;

    let sender = &deps.api.addr_canonicalize(info.sender.as_str()).unwrap();
    let mut state = State::load(deps.storage)?;
    let mut user = User::load(deps.storage, sender);

    let claimable_token = calculate_claimable_tokens(&config, &user, env.block.time.seconds());

    user.swapped_out_claimed += claimable_token;

    state.total_claimed += claimable_token;

    User::save(deps.storage, sender, &user)?;
    State::save(deps.storage, &state)?;

    let output_token = match config.output_token {
        Denom::Native(_) => unreachable!("native as output_token is not supported"),
        Denom::Cw20(output_token) => output_token,
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: output_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: claimable_token,
            })?,
            funds: vec![],
        }))
        .add_attributes(vec![
            attr("action", "claim"),
            attr("sender", info.sender.to_string()),
            attr("amount", claimable_token.to_string()),
        ]))
}

pub fn calculate_claimable_tokens(config: &Config, user: &User, time: u64) -> Uint128 {
    let (count, mut ratio) = config.distribution_strategies.iter().fold(
        (0u64, Decimal::zero()),
        |(count, ratio), strategy| {
            let (release_amount, fulfilled) = strategy.release_amount_at(&time);
            (
                count + if fulfilled { 1 } else { 0 },
                ratio + release_amount,
            )
        },
    );
    if u64::try_from(config.distribution_strategies.len()).unwrap() == count {
        ratio = Decimal::one();
    }

    (user.swapped_out * ratio) - user.swapped_out_claimed
}
