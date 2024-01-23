use cosmwasm_std::{Decimal, Deps, Fraction, StdResult, Uint128};

use crate::error::ContractError;
use crate::msg::{PriceResponse, SimulateBurnResponse, UserInfoResponse};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE, user::USER};
use crate::types::output_token::OutputToken;

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;

    Ok(config)
}

pub fn query_user(deps: Deps, address: String) -> StdResult<UserInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let user = USER.load(deps.storage, address.as_bytes())?;

    let previously_burned = user.burned_uusd;
    let cap = config.slot_size * user.slots;

    Ok(UserInfoResponse {
        burned: previously_burned,
        burnable: cap - previously_burned,
        cap,
        slots: user.slots,
        slot_size: config.slot_size,
        swapped_out: user.swapped_out,
    })
}

pub fn calculate_current_price(state: &State, token: OutputToken) -> Decimal {
    Decimal::from_ratio(state.x_liquidity, state.y_liquidity.get(token))
}

pub fn query_current_price(deps: Deps, token: OutputToken) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(PriceResponse {
        price: calculate_current_price(&state, token),
    })
}

pub fn query_simulate_burn(
    deps: Deps,
    amount: Uint128,
    out_token: OutputToken,
) -> StdResult<SimulateBurnResponse> {
    let state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;
    let price = calculate_current_price(&state, out_token);

    let k = state.x_liquidity * state.y_liquidity.get(out_token);

    if state.y_liquidity.get(out_token) + amount == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {}.into());
    }

    let swapped_out = state.x_liquidity - (k / (state.y_liquidity.get(out_token) + amount));
    if state.total_swapped.get(out_token) + swapped_out > config.sale_amount.get(out_token) {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount.get(out_token) - state.total_swapped.get(out_token),
        }
        .into());
    }

    let virtual_slippage = swapped_out * price.numerator() / price.denominator() - amount;

    Ok(SimulateBurnResponse {
        swapped_out,
        virtual_slippage,
        final_amount: amount - virtual_slippage,
    })
}
