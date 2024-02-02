use std::cmp::min;
use cosmwasm_std::{attr, Addr, DepsMut, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::states::{config::CONFIG, user::User, user::USER};
use crate::types::output_token::OutputTokenMap;

pub fn ensure_user_initialized(
    deps: &mut DepsMut<'_>,
    user_address: &Addr,
) -> Result<(), ContractError> {
    let user_exists = USER.may_load(deps.storage, user_address.clone())?.is_some();
    if !user_exists {
        let new_user = User {
            burned_uusd: Uint128::zero(),
            swapped_out: OutputTokenMap {
                oppamint: Uint128::zero(),
                ancs: Uint128::zero(),
            },
            referral_a: 0,
            referral_b: false,
            referral_c: false,
        };
        USER.save(deps.storage, user_address.clone(), &new_user)?;
    }
    Ok(())
}

pub fn process_first_referral(deps: DepsMut<'_>, referrer: &str) -> Result<(), ContractError> {
    let referrer_addr = deps.api.addr_validate(referrer)?;
    let mut referrer_user = match USER.may_load(deps.storage, referrer_addr.clone())? {
        Some(state) => state,
        None => return Err(ContractError::ReferrerNotInitialized {}),
    };

    // Update first referral count
    referrer_user.referral_a = min(referrer_user.referral_a + 1, 8);

    USER.save(deps.storage, referrer_addr, &referrer_user)?;

    Ok(())
}

pub fn process_second_referral(deps: DepsMut<'_>, referrer: &str) -> Result<(), ContractError> {
    let referrer_addr = deps.api.addr_validate(referrer)?;
    let mut referrer_user = match USER.may_load(deps.storage, referrer_addr.clone())? {
        Some(state) => state,
        None => return Err(ContractError::ReferrerNotInitialized {}),
    };

    // Update second referral flag
    referrer_user.referral_b = true;

    USER.save(deps.storage, referrer_addr, &referrer_user)?;

    Ok(())
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

    let user_addr = deps.api.addr_validate(&user)?;

    // if user is already initialized, return error
    let user_exists = USER.may_load(deps.storage, user_addr.clone())?.is_some();
    if user_exists {
        return Err(ContractError::AlreadyRegistered {});
    }

    ensure_user_initialized(&mut deps, &user_addr)?;

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
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_second_referral(deps.branch(), &referrer)?;

    let mut sender = USER.load(deps.storage, info.sender.clone())?;

    // Ensure the second referrer is registered only once
    if sender.referral_c {
        return Err(ContractError::AlreadyRegistered {});
    }
    sender.referral_c = true;

    USER.save(deps.storage, info.sender, &sender)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register_2nd_referrer"),
        attr("referrer", referrer),
        attr("new_slot", "1"),
    ]))
}
