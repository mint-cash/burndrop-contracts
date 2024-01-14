use cosmwasm_std::{Decimal, Deps, Fraction, StdResult, Uint128};

use crate::error::ContractError;
use crate::msg::{BurnInfoResponse, PriceResponse, SimulateBurnResponse};
use crate::states::config::{Config, CONFIG};
use crate::states::state::{State, STATE};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(Config {
        owner: config.owner,
        slot_size: config.slot_size,
        sale_amount: config.sale_amount,
    })
}

pub fn query_burn_info(deps: Deps, address: String) -> StdResult<BurnInfoResponse> {
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
        .unwrap_or_else(|| Uint128::new(1));
    let cap: Uint128 = config.slot_size * slots;

    Ok(BurnInfoResponse {
        burned: previously_burned,
        burnable: cap - previously_burned,
        cap,
        slots,
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
