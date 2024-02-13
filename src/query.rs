use crate::error::ContractError;
use crate::msg::{
    PriceResponse, RoundsResponse, SimulateBurnResponse, UserInfoResponse, UsersInfoResponse,
};
use crate::states::{config::Config, config::CONFIG, state::State, state::STATE, user::USER};
use crate::types::output_token::OutputTokenMap;
use crate::types::swap_round::{LiquidityPair, SwapRound};
use classic_bindings::TerraQuery;
use cosmwasm_std::{Decimal, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;

pub fn query_config(deps: Deps<TerraQuery>) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;

    Ok(config)
}

pub fn query_user(deps: Deps<TerraQuery>, address: String) -> StdResult<UserInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let address = deps.api.addr_validate(&address)?;
    let user = USER.load(deps.storage, address)?;

    let previously_burned = user.burned_uusd;
    let cap = config.slot_size * user.slots();

    Ok(UserInfoResponse {
        burned: previously_burned,
        burnable: cap - previously_burned,
        cap,
        slots: user.slots(),
        slot_size: config.slot_size,
        swapped_out: user.swapped_out,
    })
}

pub fn query_users(
    deps: Deps<TerraQuery>,
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
            let cap = config.slot_size * user.slots();

            (
                address.to_string(),
                UserInfoResponse {
                    burned: previously_burned,
                    burnable: cap - previously_burned,
                    cap,
                    slots: user.slots(),
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

pub fn query_current_price(deps: Deps<TerraQuery>, env: Env) -> StdResult<PriceResponse> {
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

    if pair.x + amount == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {}.into());
    }

    let swapped_out = pair.y - (k / (pair.x + amount));
    let virtual_slippage = (swapped_out * price) / amount;

    Ok((swapped_out, virtual_slippage))
}

pub fn calculate_round_swap_result(
    amount: &OutputTokenMap<Uint128>,
    round: &SwapRound,
) -> StdResult<(OutputTokenMap<Uint128>, OutputTokenMap<Uint128>)> {
    let price = calculate_round_price(round);

    let (swapped_out_oppamint, virtual_slippage_oppamint) =
        calculate_swap_result(amount.oppamint, &round.oppamint_liquidity, price.oppamint)?;

    let (swapped_out_ancs, virtual_slippage_ancs) =
        calculate_swap_result(amount.ancs, &round.ancs_liquidity, price.ancs)?;

    Ok((
        OutputTokenMap {
            oppamint: swapped_out_oppamint,
            ancs: swapped_out_ancs,
        },
        OutputTokenMap {
            oppamint: virtual_slippage_oppamint,
            ancs: virtual_slippage_ancs,
        },
    ))
}

pub fn split_swapped_in(
    total: Uint128,
    oppamint_weight: u32,
    ancs_weight: u32,
) -> OutputTokenMap<Uint128> {
    let denom = Uint128::new(oppamint_weight as u128 + ancs_weight as u128);
    OutputTokenMap {
        oppamint: total * Uint128::new(oppamint_weight as u128) / denom,
        ancs: total * Uint128::new(ancs_weight as u128) / denom,
    }
}

pub fn query_simulate_burn(
    deps: Deps<TerraQuery>,
    env: Env,
    total_amount: Uint128,
) -> StdResult<SimulateBurnResponse> {
    let state = STATE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let round = state
        .recent_active_round(now)
        .ok_or(ContractError::NoActiveSwapRound {})?;

    let amount = split_swapped_in(total_amount, round.oppamint_weight, round.ancs_weight);

    let (swapped_out, virtual_slippage) = calculate_round_swap_result(&amount, round)?;

    Ok(SimulateBurnResponse {
        swapped_out: swapped_out.checked_sub(virtual_slippage.clone())?,
        virtual_slippage: virtual_slippage.clone(),
        final_amount: amount.oppamint + amount.ancs
            - virtual_slippage.oppamint
            - virtual_slippage.ancs,
    })
}

pub fn query_rounds(deps: Deps<TerraQuery>) -> StdResult<RoundsResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(RoundsResponse {
        rounds: state.rounds,
    })
}
