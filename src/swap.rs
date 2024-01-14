use cosmwasm_std::{DepsMut, Env, Fraction, MessageInfo, Uint128};

use crate::contract::calculate_current_price;
use crate::error::ContractError;
use crate::states::config::CONFIG;
use crate::states::state::STATE;

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn swap(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
    let config = CONFIG.load(deps.storage)?;
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
    let mut state = STATE.load(deps.storage)?;

    let price = calculate_current_price(&state);

    // if let Some(strategy) = config.deposit_cap_strategy {
    //     let (amount, unlimited) =
    //         strategy.available_cap_of(deps.querier, info.sender.to_string(), user.swapped_in);
    //     if !unlimited && swapped_in > amount {
    //         return Err(ContractError::AvailableCapExceeded { available: amount });
    //     }
    // }

    let swapped_out =
        swapped_in * Uint128::from(price.denominator()) / Uint128::from(price.numerator());
    if state.total_swapped + swapped_out > config.amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.amount - state.total_swapped,
        });
    }

    user.swapped_in += swapped_in;
    user.swapped_out += swapped_out;

    state.total_swapped += swapped_out;

    User::save(deps.storage, sender, &user)?;
    STATE.save(deps.storage, &state)?;

    // let deposit_response = Response::new().add_attributes(vec![
    //     attr("sender", info.sender.to_string()),
    //     attr("swapped_in", swapped_in.to_string()),
    //     attr("swapped_out", swapped_out.to_string()),
    // ]);

    let deposit_result = SwapResult {
        swapped_in,
        swapped_out,
    };
    Ok(deposit_result)
}
