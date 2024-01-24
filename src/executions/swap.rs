use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::executions::user::{ensure_user_initialized, process_referral};
use crate::query::calculate_round_price;
use crate::states::config::CONFIG;
use crate::states::state::STATE;
use crate::states::user::USER;

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn swap(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let now = env.block.time.seconds();
    let round_index = state
        .rounds
        .iter()
        .position(|r| r.start_time <= now && now <= r.end_time)
        .ok_or(ContractError::NoActiveSwapRound {})?;

    let round = &mut state.rounds[round_index];
    let out_token = round.output_token;

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

    let mut user = USER.load(deps.storage, info.sender.as_bytes())?;

    let price = calculate_round_price(&round);

    // TODO: Add cap check

    let k = round.x_liquidity * round.y_liquidity;

    if round.y_liquidity + swapped_in == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {});
    }

    let swapped_out = round.x_liquidity - (k / (round.y_liquidity + swapped_in));
    if state.total_swapped.get(out_token) + swapped_out > config.sale_amount.get(out_token) {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount.get(out_token) - state.total_swapped.get(out_token),
        });
    }

    let virtual_slippage = (swapped_out * price) / swapped_in;
    user.burned_uusd += swapped_in;
    user.swapped_out
        .add(out_token, swapped_out - virtual_slippage);

    state.total_swapped.add(out_token, swapped_out);
    round.x_liquidity += swapped_in;
    round.y_liquidity -= swapped_out;

    USER.save(deps.storage, info.sender.as_bytes(), &user)?;
    STATE.save(deps.storage, &state)?;

    let deposit_result = SwapResult {
        swapped_in,
        swapped_out,
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
    ensure_user_initialized(&mut deps, info.sender.as_str())?;
    process_referral(deps.branch(), &referrer)?;

    let config = CONFIG.load(deps.storage)?;

    {
        let sender = USER.load(deps.storage, info.sender.as_bytes())?;
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
            attr("swapped_out", res.swapped_out.to_string()),
        ])),
        Err(e) => Err(e),
    }
}
