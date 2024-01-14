use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use cosmwasm_std::Fraction;

use crate::error::ContractError;
use crate::query::calculate_current_price;
use crate::states::state::State;
use crate::states::{config::CONFIG, state::STATE, user::USER};

pub fn calculate_new_slots(referral_count: u32) -> u32 {
    match referral_count {
        1..=8 => 2u32.pow(referral_count - 1), // Double the slots for each referral up to the 8th
        _ => 1,                                // Reset to 1 slot after the 8th referral
    }
}

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

pub fn burn_uusd(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    referrer: String,
) -> Result<Response, ContractError> {
    let mut state: State = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    {
        // Register referrer and calculate new slots
        let mut state: State = STATE.load(deps.storage)?;

        // Update referral count
        let referrer_count = state.referral_count_by_user.entry(referrer).or_insert(0);
        *referrer_count += 1;

        // Calculate new slots
        let new_slots = calculate_new_slots(*referrer_count).into();
        state
            .slots_by_user
            .entry(info.sender.to_string())
            .and_modify(|e| *e += new_slots)
            .or_insert(new_slots);

        STATE.save(deps.storage, &state)?;
    }

    let previously_burned: Uint128 = state
        .burned_uusd_by_user
        .get(&info.sender.to_string())
        .copied()
        .unwrap_or_default();

    // slots_by_user(address) * config.slot_size
    let capped_uusd_by_user: Uint128 = {
        let slots: Uint128 = {
            state
                .slots_by_user
                .get(&info.sender.to_string())
                .copied()
                .unwrap_or_else(|| Uint128::new(1))
        };
        config.slot_size * slots
    };

    if amount + previously_burned > capped_uusd_by_user {
        return Err(ContractError::CapExceeded {});
    }

    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount.u128(), "uusd")],
    };

    state
        .burned_uusd_by_user
        .insert(info.sender.to_string(), previously_burned + amount);
    STATE.save(deps.storage, &state)?;

    // Assuming deposit function can handle the deposit post-burn
    let res = swap(deps, env, info);

    Ok(Response::new().add_message(burn_msg).add_attributes(vec![
        attr("action", "burn_uusd"),
        attr("amount", amount.to_string()),
        attr("swapped_in", res.as_ref().unwrap().swapped_in.to_string()),
        attr("swapped_out", res.as_ref().unwrap().swapped_out.to_string()),
    ]))
}

pub fn register_2nd_referrer(
    deps: DepsMut,
    info: MessageInfo,
    referrer: String,
) -> Result<Response, ContractError> {
    let mut state: State = STATE.load(deps.storage)?;

    // Ensure the second referrer is registered only once
    if state
        .second_referrer_registered
        .get(&info.sender.to_string())
        .copied()
        .unwrap_or(false)
    {
        return Err(ContractError::AlreadyRegistered {});
    }

    state
        .second_referrer_registered
        .insert(info.sender.to_string(), true);

    // Logic similar to the first referrer, but without incrementing the referral count
    let current_slots = state
        .slots_by_user
        .entry(info.sender.to_string())
        .or_insert(Uint128::zero());
    *current_slots += Uint128::from(1u128); // Add one slot

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register_2nd_referrer"),
        attr("referrer", referrer),
        attr("new_slot", "1"),
    ]))
}
