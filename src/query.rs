use cosmwasm_std::{Decimal, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{
    PriceResponse, RoundsResponse, SimulateBurnResponse, UserInfoResponse, UsersInfoResponse,
};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE, user::USER};
use crate::types::output_token::OutputTokenMap;
use crate::types::swap_round::{LiquidityPair, SwapRound};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;

    Ok(config)
}

pub fn query_user(deps: Deps, address: String) -> StdResult<UserInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let address = deps.api.addr_validate(&address)?;
    let user = USER.load(deps.storage, address)?;

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

pub fn query_users(
    deps: Deps,
    start: Option<String>,
    limit: Option<u32>,
    order: Option<Order>,
) -> StdResult<UsersInfoResponse> {
    let config = CONFIG.load(deps.storage)?;

    let start = start.map(|s| deps.api.addr_validate(&s)).transpose()?;
    let limit = limit
        .unwrap_or(config.default_query_limit)
        .min(config.max_query_limit) as usize;
    let order = order.unwrap_or(Order::Ascending);

    let (min, max) = match order {
        Order::Ascending => (start.map(Bound::exclusive), None),
        Order::Descending => (None, start.map(Bound::exclusive)),
    };

    let users: Vec<(String, UserInfoResponse)> = USER
        .range(deps.storage, min, max, order)
        .take(limit)
        .map(|item| {
            let (address, user) = item.unwrap();
            let previously_burned = user.burned_uusd;
            let cap = config.slot_size * user.slots;

            (
                address.to_string(),
                UserInfoResponse {
                    burned: previously_burned,
                    burnable: cap - previously_burned,
                    cap,
                    slots: user.slots,
                    slot_size: config.slot_size,
                    swapped_out: user.swapped_out,
                },
            )
        })
        .collect();

    Ok(UsersInfoResponse { users })
}

pub fn calculate_round_price(round: &SwapRound) -> OutputTokenMap<Decimal> {
    OutputTokenMap {
        oppamint: Decimal::from_ratio(round.oppamint_liquidity.x, round.oppamint_liquidity.y),
        ancs: Decimal::from_ratio(round.ancs_liquidity.x, round.ancs_liquidity.y),
    }
}

// find the recent active round
// if no active round, use the first round
pub fn calculate_current_price(state: &State, now: u64) -> OutputTokenMap<Decimal> {
    let round = state.recent_active_round(now).unwrap_or(&state.rounds[0]);

    calculate_round_price(round)
}

pub fn query_current_price(deps: Deps, env: Env) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    let now = env.block.time.seconds();

    Ok(PriceResponse {
        price: calculate_current_price(&state, now),
    })
}

pub fn calculate_swap_result(
    amount: Uint128,
    pair: &LiquidityPair,
    price: Decimal,
) -> StdResult<(Uint128, Uint128)> {
    let k = pair.x * pair.y;

    if pair.y + amount == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {}.into());
    }

    let swapped_out = pair.x - (k / (pair.y + amount));
    let virtual_slippage = (swapped_out * price) / amount;

    Ok((swapped_out, virtual_slippage))
}

pub fn query_simulate_burn(
    deps: Deps,
    env: Env,
    amount: Uint128,
) -> StdResult<SimulateBurnResponse> {
    let state = STATE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let round = state
        .recent_active_round(now)
        .ok_or(ContractError::NoActiveSwapRound {})?;
    let price = calculate_round_price(round);

    let half_amount = amount / Uint128::new(2);

    let (swapped_out_oppamint, virtual_slippage_oppamint) =
        calculate_swap_result(half_amount, &round.oppamint_liquidity, price.oppamint)?;

    let (swapped_out_ancs, virtual_slippage_ancs) =
        calculate_swap_result(half_amount, &round.ancs_liquidity, price.ancs)?;

    Ok(SimulateBurnResponse {
        swapped_out: OutputTokenMap {
            oppamint: swapped_out_oppamint,
            ancs: swapped_out_ancs,
        },
        virtual_slippage: OutputTokenMap {
            oppamint: virtual_slippage_oppamint,
            ancs: virtual_slippage_ancs,
        },
        final_amount: (half_amount * Uint128::new(2))
            - virtual_slippage_oppamint
            - virtual_slippage_ancs,
    })
}

pub fn query_rounds(deps: Deps) -> StdResult<RoundsResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(RoundsResponse {
        rounds: state.rounds,
    })
}
