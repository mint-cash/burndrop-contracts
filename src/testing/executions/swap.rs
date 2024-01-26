use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw_multi_test::Executor;

use crate::executions::round::UpdateRoundParams;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::{instantiate, ADMIN, NATIVE_DENOM, REFERRER, USER};

#[test]
fn success_during_period() {
    let (mut app, burn_contract) = instantiate::default();
    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let sender_info = mock_info(
        USER,
        &[Coin {
            denom: NATIVE_DENOM.to_string(),
            amount: burn_amount,
        }],
    );

    // Create a burn tokens message
    let msg = ExecuteMsg::BurnTokens {
        amount: burn_amount,
        referrer: REFERRER.to_string(),
    };
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1706001506); // in the executions period
    });
    let execute_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &msg,
        &sender_info.funds,
    );
    assert!(execute_res.is_ok());

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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(196)); // 200 - virtual_slippage (4)
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(0));
}

#[test]
fn fail_not_period() {
    let (mut app, burn_contract) = instantiate::default();

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let sender_info = mock_info(
        USER,
        &[Coin {
            denom: NATIVE_DENOM.to_string(),
            amount: burn_amount,
        }],
    );

    // Create a burn tokens message
    let msg = ExecuteMsg::BurnTokens {
        amount: burn_amount,
        referrer: REFERRER.to_string(),
    };
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1706001653); // not in time
    });
    let execute_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &msg,
        &sender_info.funds,
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
            output_token: None,

            x_liquidity: None,
            y_liquidity: None,
        },
    };
    let cosmos_msg = burn_contract.call(modify_msg).unwrap();
    let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let sender_info = mock_info(
        USER,
        &[Coin {
            denom: NATIVE_DENOM.to_string(),
            amount: burn_amount,
        }],
    );

    // Create a burn tokens message
    let burn_msg = ExecuteMsg::BurnTokens {
        amount: burn_amount,
        referrer: REFERRER.to_string(),
    };
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1706001506); // in the previous executions period, not in the modified period
    });
    let burn_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &burn_msg,
        &sender_info.funds,
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
            output_token: None,

            x_liquidity: None,
            y_liquidity: None,
        },
    };
    let cosmos_msg = burn_contract.call(modify_msg).unwrap();
    let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let sender_info = mock_info(
        USER,
        &[Coin {
            denom: NATIVE_DENOM.to_string(),
            amount: burn_amount,
        }],
    );

    // Create a burn tokens message
    let burn_msg = ExecuteMsg::BurnTokens {
        amount: burn_amount,
        referrer: REFERRER.to_string(),
    };
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1706001680); // in the modified executions period
    });
    let burn_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &burn_msg,
        &sender_info.funds,
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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(196)); // 200 - virtual_slippage (4)
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(0));
}
