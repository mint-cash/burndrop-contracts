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
    slug: String,
    referrer: Option<String>,
) -> Result<Response, ContractError> {
    ensure_user_initialized(&mut deps, &info.sender)?;
    process_first_referral(deps.branch(), &info.sender, &referrer)?;

    let mut state = STATE.load(deps.storage)?;
    state.guild_count += 1;
    STATE.save(deps.storage, &state)?;

    let new_guild = Guild {
        slug,
        name,
        users: vec![],
        burned_uusd: Uint128::zero(),
    };

    GUILD.save(deps.storage, state.guild_count, &new_guild)?;

    let mut user = USER.load(deps.storage, info.sender.clone())?;
    user.guild_id = state.guild_count;
    user.guild_contributed_uusd = Uint128::zero();
    USER.save(deps.storage, info.sender, &user)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "create_guild"),
        attr("id", state.guild_count.to_string()),
    ]))
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

    let mut old_guild = GUILD.load(deps.storage, user.guild_id)?;
    old_guild.users.retain(|u| u.address != info.sender);
    GUILD.save(deps.storage, user.guild_id, &old_guild)?;

    user.guild_id = guild_id;
    user.guild_contributed_uusd = Uint128::zero();
    USER.save(deps.storage, info.sender, &user)?;

    let mut guild = GUILD.load(deps.storage, guild_id)?;
    guild.users.push(user);
    GUILD.save(deps.storage, guild_id, &guild)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "migrate_guild"),
        attr("id", guild_id.to_string()),
    ]))
}
