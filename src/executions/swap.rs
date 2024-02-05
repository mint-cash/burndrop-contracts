use cosmwasm_std::{attr, coin, BankMsg, Decimal, DepsMut, Env, Fraction, MessageInfo, Uint128};

use crate::error::ContractError;
use crate::executions::user::{ensure_user_initialized, process_first_referral};
use crate::query::calculate_round_swap_result;
use crate::states::config::CONFIG;
use crate::states::state::STATE;
use crate::states::user::USER;
use crate::types::output_token::OutputTokenMap;
use classic_bindings::{TerraMsg, TerraQuerier, TerraQuery};

pub type Response = cosmwasm_std::Response<TerraMsg>;

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: OutputTokenMap<Uint128>,
}

pub fn swap(
    deps: DepsMut<TerraQuery>,
    env: Env,
    info: MessageInfo,
) -> Result<SwapResult, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let round_index = state
        .rounds
        .iter()
        .position(|r| r.start_time <= now && now <= r.end_time)
        .ok_or(ContractError::NoActiveSwapRound {})?;

    let round = &mut state.rounds[round_index];
    let input_token_denom = "uusd";

    // burned_uusd
    let swapped_in = info
        .funds
        .iter()
        .find(|c| c.denom == input_token_denom)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);
    if swapped_in.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    if info.funds.len() > 1 {
        return Err(ContractError::NotAllowOtherDenoms {
            denom: input_token_denom.to_string(),
        });
    }

    let mut user = USER.load(deps.storage, info.sender.clone())?;

    // TODO: Add cap check

    let half_swapped_in = swapped_in / Uint128::new(2);

    let (swapped_out, virtual_slippage) = calculate_round_swap_result(half_swapped_in, round)?;

    user.burned_uusd += half_swapped_in * Uint128::new(2);
    user.swapped_out += swapped_out.clone().checked_sub(virtual_slippage)?;

    state.total_swapped += swapped_out.clone();

    round.oppamint_liquidity.x += half_swapped_in;
    round.oppamint_liquidity.y -= swapped_out.oppamint;

    round.ancs_liquidity.x += half_swapped_in;
    round.ancs_liquidity.y -= swapped_out.ancs;

    USER.save(deps.storage, info.sender, &user)?;
    STATE.save(deps.storage, &state)?;

    let deposit_result = SwapResult {
        swapped_in: half_swapped_in * Uint128::new(2),
        swapped_out: user.swapped_out.clone(),
    };
    Ok(deposit_result)
}

pub fn reverse_decimal(decimal: Decimal) -> Decimal {
    decimal.inv().unwrap_or_default()
}

pub fn deduct_tax(deps: &DepsMut<TerraQuery>, amount: Uint128) -> Result<Uint128, ContractError> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate = (terra_querier.query_tax_rate()?).rate;
    let tax_cap = (terra_querier.query_tax_cap("uusd")?).cap;

    let tax = std::cmp::min(
        amount - amount * reverse_decimal(Decimal::one() + tax_rate),
        tax_cap,
    );

    Ok(amount.checked_sub(tax)?)
}

pub fn burn_uusd(
    mut deps: DepsMut<TerraQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    referrer: Option<String>,
    min_amount_out: Option<OutputTokenMap<Uint128>>,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_first_referral(deps.branch(), &info.sender, &referrer)?;

    let sender = USER.load(deps.storage, info.sender.clone())?;

    let config = CONFIG.load(deps.storage)?;

    {
        let previously_burned = sender.burned_uusd;

        // slots_by_user(address) * config.slot_size
        let capped_uusd_by_user = config.slot_size * sender.slots();

        if amount + previously_burned > capped_uusd_by_user {
            return Err(ContractError::CapExceeded {});
        }
    }

    let amount_with_deducted_tax = match deduct_tax(&deps, amount) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount_with_deducted_tax.u128(), "uusd")],
    };

    let res = swap(deps, env, info)?;

    if let Some(min_amount_out) = min_amount_out {
        if res.swapped_out.oppamint < min_amount_out.oppamint
            || res.swapped_out.ancs < min_amount_out.ancs
        {
            return Err(ContractError::UnderMinAmountOut {});
        }
    }

    Ok(Response::new().add_message(burn_msg).add_attributes(vec![
        attr("action", "burn_uusd"),
        attr("amount", amount.to_string()),
        attr("swapped_in", res.swapped_in.to_string()),
        attr("swapped_out_oppamint", res.swapped_out.oppamint.to_string()),
        attr("swapped_out_ancs", res.swapped_out.ancs.to_string()),
    ]))
}
