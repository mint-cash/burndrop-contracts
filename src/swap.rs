use cosmwasm_std::{DepsMut, Env, Fraction, MessageInfo, Uint128};

use crate::contract::calculate_current_price;
use crate::error::ContractError;
use crate::states::{config::CONFIG, state::STATE, user::USER};

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn swap(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    // let now = env.block.time.seconds();

    // TODO: Add time check

    let input_token_denom = "uusd";

    // swapped_in
    let burned_uusd = info
        .funds
        .iter()
        .find(|c| c.denom == input_token_denom)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);
    if burned_uusd.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    if info.funds.len() > 1 {
        return Err(ContractError::NotAllowOtherDenoms {
            denom: input_token_denom.to_string(),
        });
    }

    let sender = &deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut user = USER.load(deps.storage, sender)?;
    let mut state = STATE.load(deps.storage)?;

    let price = calculate_current_price(&state);

    // TODO: Add cap check

    let k = state.x_liquidity * state.y_liquidity;

    if state.y_liquidity + burned_uusd == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {});
    }

    let swapped_out = state.x_liquidity - (k / (state.y_liquidity + burned_uusd));
    if state.total_swapped + swapped_out > config.sale_amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount - state.total_swapped,
        });
    }

    let virtual_slippage = swapped_out * price.numerator() / price.denominator() - burned_uusd;

    user.burned_uusd += burned_uusd;
    user.swapped_out += swapped_out - virtual_slippage;

    state.total_swapped += swapped_out;
    state.x_liquidity += burned_uusd;
    state.y_liquidity -= swapped_out;

    USER.save(deps.storage, sender, &user)?;
    STATE.save(deps.storage, &state)?;

    let deposit_result = SwapResult {
        swapped_in: burned_uusd,
        swapped_out,
    };
    Ok(deposit_result)
}
