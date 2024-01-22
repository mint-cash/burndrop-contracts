use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::query::calculate_current_price;
use crate::states::{config::CONFIG, state::STATE, user::User, user::USER};

fn ensure_user_initialized(
    deps: &mut DepsMut<'_>,
    user_address: &str,
) -> Result<(), ContractError> {
    let user_exists = USER
        .may_load(deps.storage, user_address.as_bytes())?
        .is_some();
    if !user_exists {
        let new_user = User {
            burned_uusd: Uint128::zero(),
            swapped_out: Uint128::zero(),
            referral_count: Uint128::zero(),
            slots: Uint128::from(1u128), // initial slot is 1
            second_referrer_registered: false,
        };
        USER.save(deps.storage, user_address.as_bytes(), &new_user)?;
    }
    Ok(())
}

// Double the slots for each referral up to the 8th and reset to 1 slot after the 8th referral
pub fn calculate_new_slots(referral_count: Uint128) -> Uint128 {
    let mut new_slots = Uint128::zero();

    if referral_count == Uint128::zero() {
        return new_slots;
    }

    if referral_count <= Uint128::from(8u128) {
        new_slots = Uint128::from(2u128).pow(referral_count.u128() as u32);
    } else {
        new_slots = Uint128::from(2u128).pow(8u32);
    }

    new_slots
}

fn process_referral(deps: DepsMut<'_>, referrer: &str) -> Result<(), ContractError> {
    let referrer_addr = deps.api.addr_validate(referrer)?;
    let mut referrer_user = match USER.may_load(deps.storage, referrer_addr.as_bytes())? {
        Some(state) => state,
        None => return Err(ContractError::ReferrerNotInitialized {}),
    };

    // Update referral count
    referrer_user.referral_count += Uint128::from(1u8);

    // Calculate new slots and update
    let new_slots = calculate_new_slots(referrer_user.referral_count);
    referrer_user.slots += new_slots;

    USER.save(deps.storage, referrer.as_bytes(), &referrer_user)?;

    Ok(())
}

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn swap(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let now = env.block.time.seconds();

    if state.start_time == 0 || state.end_time == 0 || now < state.start_time {
        return Err(ContractError::SwapNotStarted { start: state.start_time });
    }
    if now > state.end_time {
        return Err(ContractError::SwapFinished { end: state.end_time });
    }

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

    let price = calculate_current_price(&state);

    // TODO: Add cap check

    let k = state.x_liquidity * state.y_liquidity;

    if state.y_liquidity + swapped_in == Uint128::zero() {
        return Err(ContractError::DivisionByZeroError {});
    }

    let swapped_out = state.x_liquidity - (k / (state.y_liquidity + swapped_in));
    if state.total_swapped + swapped_out > config.sale_amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount - state.total_swapped,
        });
    }

    let virtual_slippage = (swapped_out * price) / swapped_in;
    user.burned_uusd += swapped_in;
    user.swapped_out += swapped_out - virtual_slippage;

    state.total_swapped += swapped_out;
    state.x_liquidity += swapped_in;
    state.y_liquidity -= swapped_out;

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

    Ok(Response::new().add_message(burn_msg).add_attributes(vec![
        attr("action", "burn_uusd"),
        attr("amount", amount.to_string()),
        attr("swapped_in", res.as_ref().unwrap().swapped_in.to_string()),
        attr("swapped_out", res.as_ref().unwrap().swapped_out.to_string()),
    ]))
}

// fn register_starting_user (only owner)
// owner can allow specific address to bypass referral requirement
// which means just init'ing the new User with initial slots is 1, so that the address can be used as referrer
pub fn register_starting_user(
    mut deps: DepsMut,
    info: MessageInfo,
    user: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // if user is already initialized, return error
    let user_exists = USER.may_load(deps.storage, user.as_bytes())?.is_some();
    if user_exists {
        return Err(ContractError::AlreadyRegistered {});
    }

    ensure_user_initialized(&mut deps, &user)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register_starting_user"),
        attr("referrer", user),
    ]))
}

pub fn register_2nd_referrer(
    mut deps: DepsMut,
    info: MessageInfo,
    referrer: String,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, info.sender.as_str())?;
    process_referral(deps.branch(), &referrer)?;

    let mut sender = USER.load(deps.storage, info.sender.as_bytes())?;

    // Ensure the second referrer is registered only once
    if sender.second_referrer_registered {
        return Err(ContractError::AlreadyRegistered {});
    }
    sender.second_referrer_registered = true;

    // Logic similar to the first referrer, but without incrementing the referral count
    // add one slot to the user
    // FIXME: make it dynamic because this additional slot must be excluded from the doubling logic
    sender.slots += Uint128::from(1u128);

    USER.save(deps.storage, info.sender.as_bytes(), &sender)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register_2nd_referrer"),
        attr("referrer", referrer),
        attr("new_slot", "1"),
    ]))
}