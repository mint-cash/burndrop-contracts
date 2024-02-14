use crate::contract::Response;
use crate::error::ContractError;
use crate::executions::user::{ensure_user_initialized, process_first_referral};
use crate::states::guild::{Guild, GUILD};
use crate::states::state::STATE;
use crate::states::user::USER;
use classic_bindings::TerraQuery;
use cosmwasm_std::{attr, DepsMut, MessageInfo, Uint128};

pub fn create_guild(
    mut deps: DepsMut<TerraQuery>,
    info: MessageInfo,
    name: String,
    referrer: Option<String>,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_first_referral(deps.branch(), &info.sender, &referrer)?;

    let mut state = STATE.load(deps.storage)?;
    state.guild_count += 1;
    STATE.save(deps.storage, &state)?;

    let new_guild = Guild {
        name,
        users: vec![],
        burned_uusd: Uint128::zero(),
    };

    GUILD.save(deps.storage, state.guild_count, &new_guild)?;

    let mut user = USER.load(deps.storage, info.sender.clone())?;
    let old_guild_id = user.guild_id;

    user.guild_id = state.guild_count;
    user.guild_contributed_uusd = Uint128::zero();
    USER.save(deps.storage, info.sender.clone(), &user)?;

    let mut attributes = vec![
        attr("action", "create_guild"),
        attr("sender", info.sender),
        attr("old_guild_id", old_guild_id.to_string()),
        attr("new_guild_id", state.guild_count.to_string()),
        attr("new_guild_name", new_guild.name),
    ];

    if let Some(referrer) = referrer {
        attributes.push(attr("referrer", referrer));
    }

    Ok(Response::new() //
        .add_attributes(attributes))
}

pub fn migrate_guild(
    mut deps: DepsMut<TerraQuery>,
    info: MessageInfo,
    guild_id: u64,
    referrer: Option<String>,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_first_referral(deps.branch(), &info.sender, &referrer)?;

    let mut user = USER.load(deps.storage, info.sender.clone())?;

    let old_guild_id = user.guild_id;
    let mut old_guild = GUILD.load(deps.storage, old_guild_id)?;
    old_guild.users.retain(|u| u.address != info.sender);
    GUILD.save(deps.storage, old_guild_id, &old_guild)?;

    user.guild_id = guild_id;
    user.guild_contributed_uusd = Uint128::zero();
    USER.save(deps.storage, info.sender.clone(), &user)?;

    let mut guild = GUILD.load(deps.storage, guild_id)?;
    guild.users.push(user);
    GUILD.save(deps.storage, guild_id, &guild)?;

    let mut attributes = vec![
        attr("action", "migrate_guild"),
        attr("sender", info.sender),
        attr("old_guild_id", old_guild_id.to_string()),
        attr("new_guild_id", guild_id.to_string()),
    ];

    if let Some(referrer) = referrer {
        attributes.push(attr("referrer", referrer));
    }

    Ok(Response::new() //
        .add_attributes(attributes))
}
