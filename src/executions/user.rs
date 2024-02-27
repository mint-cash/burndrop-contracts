use crate::error::ContractError;
use crate::states::{config::CONFIG, user::User, user::USER};
use crate::types::output_token::OutputTokenMap;
use classic_bindings::{TerraMsg, TerraQuery};
use cosmwasm_std::{attr, Addr, DepsMut, MessageInfo, Uint128};

pub type Response = cosmwasm_std::Response<TerraMsg>;

pub fn ensure_user_initialized(
    deps: &mut DepsMut<TerraQuery>,
    user_address: &Addr,
) -> Result<(), ContractError> {
    let user_exists = USER.may_load(deps.storage, user_address.clone())?.is_some();
    if !user_exists {
        let new_user = User {
            address: user_address.clone(),
            burned_uusd: Uint128::zero(),
            swapped_out: OutputTokenMap {
                oppamint: Uint128::zero(),
                ancs: Uint128::zero(),
            },
            referral_a: 0,
            first_referrer: None,
            guild_id: 0, // genesis guild
            guild_contributed_uusd: Uint128::zero(),
        };
        USER.save(deps.storage, user_address.clone(), &new_user)?;
    }
    Ok(())
}

pub fn process_first_referral(
    deps: DepsMut<TerraQuery>,
    user_addr: &Addr,
    referrer: &Option<String>,
) -> Result<(), ContractError> {
    let mut user = USER.load(deps.storage, user_addr.clone())?;
    if user.first_referrer.is_some() {
        return match referrer {
            Some(_) => Err(ContractError::AlreadyRegistered {}),
            None => Ok(()),
        };
    }

    let referrer = match referrer {
        Some(r) => r,
        None => return Err(ContractError::ReferrerNotProvided {}),
    };

    if referrer.as_str() == *user_addr {
        return Err(ContractError::ReferrerIsSelf {});
    }

    let referrer_addr = deps.api.addr_validate(referrer)?;
    let mut referrer_user = match USER.may_load(deps.storage, referrer_addr.clone())? {
        Some(state) => state,
        None => return Err(ContractError::ReferrerNotInitialized {}),
    };

    // Update first referral count
    referrer_user.referral_a += 1;

    // Update user's first referrer
    user.first_referrer = Some(referrer_addr.clone());

    USER.save(deps.storage, referrer_addr, &referrer_user)?;
    USER.save(deps.storage, user_addr.clone(), &user)?;

    Ok(())
}

// fn register_starting_user (only owner)
// owner can allow specific address to bypass referral requirement
// which means just init'ing the new User with initial slots is 1, so that the address can be used as referrer
pub fn register_starting_user(
    mut deps: DepsMut<TerraQuery>,
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
        attr("sender", info.sender),
        attr("referrer", user),
    ]))
}
