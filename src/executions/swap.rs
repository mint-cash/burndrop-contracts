use cosmwasm_std::{
    attr, coin, BankMsg, Decimal, Deps, DepsMut, Env, Fraction, MessageInfo, Uint128,
};

use crate::error::ContractError;
use crate::executions::user::{ensure_user_initialized, process_first_referral};
use crate::query::{calculate_round_swap_result, split_swapped_in};
use crate::states::config::CONFIG;
use crate::states::guild::GUILD;
use crate::states::overridden_rounds::{OVERRIDDEN_BURNED_UUSD, OVERRIDDEN_ROUNDS};
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
    info: &MessageInfo,
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
    let tentative_swapped_in = info
        .funds
        .iter()
        .find(|c| c.denom == input_token_denom)
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);
    if tentative_swapped_in.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    if info.funds.len() > 1 {
        return Err(ContractError::NotAllowOtherDenoms {
            denom: input_token_denom.to_string(),
        });
    }

    let mut user = USER.load(deps.storage, info.sender.clone())?;
    let mut guild = GUILD.load(deps.storage, user.guild_id)?;

    // TODO: Add cap check

    let swapped_in = split_swapped_in(
        tentative_swapped_in,
        round.oppamint_weight,
        round.ancs_weight,
    );
    let swapped_out = calculate_round_swap_result(&swapped_in, round)?;

    let total_swapped_in = swapped_in.oppamint + swapped_in.ancs;

    user.burned_uusd += total_swapped_in;
    user.guild_contributed_uusd += total_swapped_in;
    guild.burned_uusd += total_swapped_in;
    user.swapped_out += swapped_out.clone();

    state.total_swapped += swapped_out.clone();

    round.oppamint_liquidity.x += swapped_in.oppamint;
    round.oppamint_liquidity.y -= swapped_out.oppamint;

    round.ancs_liquidity.x += swapped_in.ancs;
    round.ancs_liquidity.y -= swapped_out.ancs;

    let overridden_rounds = OVERRIDDEN_ROUNDS.load(deps.storage)?;
    if let Some((_, current_round_index)) = overridden_rounds.current_round(now) {
        let prev = OVERRIDDEN_BURNED_UUSD
            .may_load(deps.storage, (current_round_index, user.address.clone()))?
            .unwrap_or(Uint128::zero());

        let burned_uusd = prev + total_swapped_in;

        OVERRIDDEN_BURNED_UUSD.save(
            deps.storage,
            (current_round_index, user.address.clone()),
            &burned_uusd,
        )?;
    }

    USER.save(deps.storage, info.sender.clone(), &user)?;
    STATE.save(deps.storage, &state)?;
    GUILD.save(deps.storage, user.guild_id, &guild)?;

    let deposit_result = SwapResult {
        swapped_in: total_swapped_in,
        swapped_out,
    };
    Ok(deposit_result)
}

pub fn reverse_decimal(decimal: Decimal) -> Decimal {
    decimal.inv().unwrap_or_default()
}

pub fn deduct_tax(deps: &Deps<TerraQuery>, amount: Uint128) -> Result<Uint128, ContractError> {
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

        let now = env.block.time.seconds();
        let overridden_rounds = OVERRIDDEN_ROUNDS.load(deps.storage)?;
        let (recent_overridden_round, recent_overridden_round_index) =
            match overridden_rounds.recent_active_round(now) {
                Some((round, index)) => (Some(round), Some(index)),
                None => (None, None),
            };

        let slots = sender.slots();
        let slot_size = if overridden_rounds.is_active(recent_overridden_round, now) {
            recent_overridden_round.unwrap().slot_size
        } else {
            config.slot_size
        };

        let overridden_burned_uusd = if overridden_rounds.is_active(recent_overridden_round, now) {
            // active: prev (i - 1)
            match recent_overridden_round_index {
                Some(0) => Uint128::zero(),
                Some(index) => {
                    OVERRIDDEN_BURNED_UUSD.load(deps.storage, (index - 1, sender.address))?
                }
                None => Uint128::zero(),
            }
        } else {
            // inactive: current (i)
            match recent_overridden_round_index {
                Some(index) => {
                    OVERRIDDEN_BURNED_UUSD.load(deps.storage, (index, sender.address))?
                }
                None => Uint128::zero(),
            }
        };

        let capped_uusd_by_user = slot_size * slots + overridden_burned_uusd;
        if amount + previously_burned > capped_uusd_by_user {
            return Err(ContractError::CapExceeded {});
        }
    }

    let amount_with_deducted_tax = match deduct_tax(&deps.as_ref(), amount) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let burn_address = "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu";
    let burn_msg = BankMsg::Send {
        to_address: burn_address.to_string(),
        amount: vec![coin(amount_with_deducted_tax.u128(), "uusd")],
    };

    let res = swap(deps, env, &info)?;

    if let Some(min_amount_out) = min_amount_out {
        if res.swapped_out.oppamint < min_amount_out.oppamint
            || res.swapped_out.ancs < min_amount_out.ancs
        {
            return Err(ContractError::UnderMinAmountOut {});
        }
    }

    let mut attributes = vec![
        attr("action", "burn_uusd"),
        attr("sender", info.sender),
        attr("sender_guild_id", sender.guild_id.to_string()),
        attr("amount", amount.to_string()),
        attr("swapped_in", res.swapped_in.to_string()),
        attr("swapped_out_oppamint", res.swapped_out.oppamint.to_string()),
        attr("swapped_out_ancs", res.swapped_out.ancs.to_string()),
    ];

    if let Some(referrer) = referrer {
        attributes.push(attr("referrer", referrer));
    }

    Ok(Response::new()
        .add_message(burn_msg)
        .add_attributes(attributes))
}
