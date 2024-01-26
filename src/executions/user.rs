use cosmwasm_std::{attr, DepsMut, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::states::{config::CONFIG, user::User, user::USER};
use crate::types::output_token::OutputTokenMap;

pub fn ensure_user_initialized(
    deps: &mut DepsMut<'_>,
    user_address: &str,
) -> Result<(), ContractError> {
    let user_exists = USER
        .may_load(deps.storage, user_address.as_bytes())?
        .is_some();
    if !user_exists {
        let new_user = User {
            burned_uusd: Uint128::zero(),
            swapped_out: OutputTokenMap {
                oppamint: Uint128::zero(),
                ancs: Uint128::zero(),
            },
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

pub fn process_referral(deps: DepsMut<'_>, referrer: &str) -> Result<(), ContractError> {
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
