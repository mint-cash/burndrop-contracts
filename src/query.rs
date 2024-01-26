use cosmwasm_std::{Decimal, Deps, Env, Fraction, StdResult, Uint128};

use crate::error::ContractError;
use crate::msg::{PriceResponse, RoundsResponse, SimulateBurnResponse, UserInfoResponse};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE, user::USER};
use crate::types::swap_round::SwapRound;

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

pub fn calculate_round_price(round: &SwapRound) -> Decimal {
    Decimal::from_ratio(round.x_liquidity, round.y_liquidity)
}

// find the recent active round
// if no active round, use the first round
pub fn calculate_current_price(state: &State, now: u64) -> Decimal {
    let round = state.recent_active_round(now);

    match round {
        Some(round) => calculate_round_price(round),
        None => calculate_round_price(state.rounds.first().unwrap()),
    }
}

pub fn query_current_price(deps: Deps, env: Env) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    let now = env.block.time.seconds();

    Ok(PriceResponse {
        price: calculate_current_price(&state, now),
    })
}

pub fn query_simulate_burn(
    deps: Deps,
    env: Env,
    amount: Uint128,
) -> StdResult<SimulateBurnResponse> {
    let state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    let now = env.block.time.seconds();
    let round = state
        .recent_active_round(now)
        .ok_or(ContractError::NoActiveSwapRound {})?;
    let out_token = round.output_token;
    let price = calculate_round_price(round);

    let k = round.x_liquidity * round.y_liquidity;

    if round.y_liquidity + amount == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {}.into());
    }

    let swapped_out = round.x_liquidity - (k / (round.y_liquidity + amount));
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

pub fn query_rounds(deps: Deps) -> StdResult<RoundsResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(RoundsResponse {
        rounds: state.rounds,
    })
}
