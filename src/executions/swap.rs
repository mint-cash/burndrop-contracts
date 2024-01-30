use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::executions::user::{ensure_user_initialized, process_referral};
use crate::query::{calculate_round_price, calculate_swap_result};
use crate::states::config::CONFIG;
use crate::states::state::STATE;
use crate::states::user::USER;
use crate::types::output_token::OutputTokenMap;

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: OutputTokenMap<Uint128>,
}

pub fn swap(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
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

    let price = calculate_round_price(round);

    // TODO: Add cap check

    let half_swapped_in = swapped_in / Uint128::new(2);

    let (swapped_out_oppamint, virtual_slippage_oppamint) =
        calculate_swap_result(half_swapped_in, &round.oppamint_liquidity, price.oppamint)?;

    let (swapped_out_ancs, virtual_slippage_ancs) =
        calculate_swap_result(half_swapped_in, &round.ancs_liquidity, price.ancs)?;

    println!("swapped_out_oppamint: {}", swapped_out_oppamint);
    println!("swapped_out_ancs: {}", swapped_out_ancs);

    println!("virtual_slippage_oppamint: {}", virtual_slippage_oppamint);
    println!("virtual_slippage_ancs: {}", virtual_slippage_ancs);

    user.burned_uusd += half_swapped_in * Uint128::new(2);
    user.swapped_out.oppamint += swapped_out_oppamint - virtual_slippage_oppamint;
    user.swapped_out.ancs += swapped_out_ancs - virtual_slippage_ancs;

    state.total_swapped.oppamint += swapped_out_oppamint;
    state.total_swapped.ancs += swapped_out_ancs;

    round.oppamint_liquidity.x += half_swapped_in;
    round.oppamint_liquidity.y -= swapped_out_oppamint;

    round.ancs_liquidity.x += half_swapped_in;
    round.ancs_liquidity.y -= swapped_out_ancs;

    USER.save(deps.storage, info.sender, &user)?;
    STATE.save(deps.storage, &state)?;

    let deposit_result = SwapResult {
        swapped_in: half_swapped_in * Uint128::new(2),
        swapped_out: OutputTokenMap {
            oppamint: swapped_out_oppamint,
            ancs: swapped_out_ancs,
        },
    };
    Ok(deposit_result)
}

pub fn burn_uusd(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    referrer: String,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_referral(deps.branch(), &referrer)?;

    let config = CONFIG.load(deps.storage)?;

    {
        let sender = USER.load(deps.storage, info.sender.clone())?;
        let previously_burned = sender.burned_uusd;

        // slots_by_user(address) * config.slot_size
        let capped_uusd_by_user = {
            let slots = sender.slots;
            config.slot_size * slots
        };

        if amount + previously_burned > capped_uusd_by_user {
            return Err(ContractError::CapExceeded {});
        }
    }

    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount.u128(), "uusd")],
    };

    let res = swap(deps, env, info);

    match res {
        Ok(res) => Ok(Response::new().add_message(burn_msg).add_attributes(vec![
            attr("action", "burn_uusd"),
            attr("amount", amount.to_string()),
            attr("swapped_in", res.swapped_in.to_string()),
            attr("swapped_out_oppamint", res.swapped_out.oppamint.to_string()),
            attr("swapped_out_ancs", res.swapped_out.ancs.to_string()),
        ])),
        Err(e) => Err(e),
    }
}
