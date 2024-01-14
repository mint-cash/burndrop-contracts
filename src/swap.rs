use cosmwasm_std::{DepsMut, Env, Fraction, MessageInfo, Uint128};

use crate::contract::calculate_current_price;
use crate::error::ContractError;
use crate::states::config::CONFIG;
use crate::states::state::STATE;

pub struct SwapResult {
    pub swapped_in: Uint128,
    pub swapped_out: Uint128,
}

pub fn swap(deps: DepsMut, env: Env, info: MessageInfo) -> Result<SwapResult, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let now = env.block.time.seconds();

    // TODO: Add time check

    let input_token_denom = "uusd";

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

    let sender = &deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut user = User::load(deps.storage, sender);
    let mut state = STATE.load(deps.storage)?;

    let price = calculate_current_price(&state);

    // TODO: Add cap check

    let swapped_out =
        swapped_in * Uint128::from(price.denominator()) / Uint128::from(price.numerator());
    if state.total_swapped + swapped_out > config.sale_amount {
        return Err(ContractError::PoolSizeExceeded {
            available: config.sale_amount - state.total_swapped,
        });
    }

    user.swapped_in += swapped_in;
    user.swapped_out += swapped_out;

    state.total_swapped += swapped_out;

    User::save(deps.storage, sender, &user)?;
    STATE.save(deps.storage, &state)?;

    let deposit_result = SwapResult {
        swapped_in,
        swapped_out,
    };
    Ok(deposit_result)
}
