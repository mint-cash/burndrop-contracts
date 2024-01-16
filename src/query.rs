use cosmwasm_std::{Decimal, Deps, Fraction, StdResult, Uint128};

use crate::error::ContractError;
use crate::msg::{BurnInfoResponse, PriceResponse, SimulateBurnResponse};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE, user::USER};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;

    Ok(Config {
        owner: config.owner,
        slot_size: config.slot_size,
        sale_amount: config.sale_amount,
    })
}

pub fn query_burn_info(deps: Deps, address: String) -> StdResult<BurnInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let user: crate::states::user::User = USER.load(deps.storage, address.as_bytes())?;

    let previously_burned = user.burned_uusd;
    let cap = config.slot_size * user.slots;

    Ok(BurnInfoResponse {
        burned: previously_burned,
        burnable: cap - previously_burned,
        cap,
        slots: user.slots,
        slot_size: config.slot_size,
    })
}

pub fn calculate_current_price(state: &State) -> Decimal {
    Decimal::from_ratio(state.x_liquidity, state.y_liquidity)
}

pub fn query_current_price(deps: Deps) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(PriceResponse {
        price: calculate_current_price(&state),
    })
}

pub fn query_simulate_burn(deps: Deps, amount: Uint128) -> StdResult<SimulateBurnResponse> {
    let state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;
    let price = calculate_current_price(&state);

    let k = state.x_liquidity * state.y_liquidity;

    if state.y_liquidity + amount == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {}.into());
    }

    let swapped_out = state.x_liquidity - (k / (state.y_liquidity + amount));
    if state.total_swapped + swapped_out > config.sale_amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount - state.total_swapped,
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
