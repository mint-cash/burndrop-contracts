use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw_multi_test::{App, AppResponse, Executor};

use crate::executions::round::UpdateRoundParams;
use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, QueryMsg, UserInfoResponse};
use crate::testing::{instantiate, ADMIN, NATIVE_DENOM, REFERRER, USER};

pub fn execute_swap(
    app: &mut App,
    burn_contract: &BurnContract,
    sender: &str,
    amount: Uint128,
    referrer: &str,
    time: Option<u64>,
) -> anyhow::Result<AppResponse> {
    // Try to burn some tokens for a user with a referrer.
    let sender_info = mock_info(
        sender,
        &[Coin {
            denom: NATIVE_DENOM.to_string(),
            amount,
        }],
    );

    // Create a burn tokens message
    let msg = ExecuteMsg::BurnUusd {
        amount,
        referrer: Some(referrer.to_string()),
    };
    if let Some(time) = time {
        app.update_block(|block| {
            block.time = Timestamp::from_seconds(time);
        });
    }
    app.execute_contract(
        Addr::unchecked(sender),
        burn_contract.addr(),
        &msg,
        &sender_info.funds,
    )
}

#[test]
fn success_during_period() {
    let (mut app, burn_contract) = instantiate::default();
    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        Some(1706001506),
    );
    assert!(execute_res.is_ok());

    // Query the burn info after burning tokens for the user.
    let query_res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    // Perform assertions based on the expected state after burning tokens.
    assert_eq!(query_res.slot_size, Uint128::new(1000));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000));

    assert_eq!(query_res.burned, Uint128::new(100));
    assert_eq!(query_res.burnable, Uint128::new(900));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(96)); // 100 - virtual_slippage (4)
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(96)); // 100 - virtual_slippage (4)
}

#[test]
fn success_odd_amount() {
    let (mut app, burn_contract) = instantiate::default();
    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(99);

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        Some(1706001506),
    );
    assert!(execute_res.is_ok());

    // Query the burn info after burning tokens for the user.
    let query_res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    // Perform assertions based on the expected state after burning tokens.
    assert_eq!(query_res.slot_size, Uint128::new(1000));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000));

    assert_eq!(query_res.burned, Uint128::new(98));
    assert_eq!(query_res.burnable, Uint128::new(902));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(94)); // 98 - virtual_slippage (4)
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(94)); // 98 - virtual_slippage (4)
}

#[test]
fn fail_not_period() {
    let (mut app, burn_contract) = instantiate::default();

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        Some(1706001653),
    );
    assert!(execute_res.is_err());
}

#[test]
fn fail_not_modified_period() {
    let (mut app, burn_contract) = instantiate::default();

    // Modify the executions period to be shorter.
    let modify_msg = ExecuteMsg::UpdateRound {
        params: UpdateRoundParams {
            id: 1,
            start_time: None,
            end_time: Some(1706001500), // modify end_time

            oppamint_liquidity: None,
            ancs_liquidity: None,
        },
    };
    let cosmos_msg = burn_contract.call(modify_msg).unwrap();
    let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        Some(1706001506),
    );

    assert!(burn_res.is_err());
}

#[test]
fn success_during_modified_period() {
    let (mut app, burn_contract) = instantiate::default();

    // Modify the executions period to be shorter.
    let modify_msg = ExecuteMsg::UpdateRound {
        params: UpdateRoundParams {
            id: 1,
            start_time: None,
            end_time: Some(1706001700), // modify end_time

            oppamint_liquidity: None,
            ancs_liquidity: None,
        },
    };
    let cosmos_msg = burn_contract.call(modify_msg).unwrap();
    let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        Some(1706001506),
    );
    assert!(burn_res.is_ok());

    // Query the burn info after burning tokens for the user.
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    // Perform assertions based on the expected state after burning tokens.
    assert_eq!(query_res.slot_size, Uint128::new(1000));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000));

    assert_eq!(query_res.burned, Uint128::new(100));
    assert_eq!(query_res.burnable, Uint128::new(900));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(96)); // 100 - virtual_slippage (4)
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(96)); // 100 - virtual_slippage (4)
}
