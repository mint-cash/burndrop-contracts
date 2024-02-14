use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Coin, Event, Timestamp, Uint128};
use cw_multi_test::{AppResponse, Executor};

use crate::executions::round::UpdateRoundParams;
use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, QueryMsg, UserInfoResponse};
use crate::testing::terra_bindings::TerraApp;
use crate::testing::{instantiate, ADMIN, NATIVE_DENOM, REFERRER, USER};
use crate::types::output_token::OutputTokenMap;

pub fn execute_swap(
    app: &mut TerraApp,
    burn_contract: &BurnContract,
    sender: &str,
    amount: Uint128,
    referrer: Option<&str>,
    time: Option<u64>,
    min_amount_out: Option<OutputTokenMap<Uint128>>,
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
        referrer: referrer.map(String::from),
        min_amount_out,
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
        Some(REFERRER),
        Some(1706001506),
        None,
    );
    assert!(execute_res.is_ok());
    let execute_res: AppResponse = execute_res.unwrap();
    execute_res.assert_event(&Event::new("wasm").add_attributes(vec![
        ("action", "burn_uusd"),
        ("sender", USER),
        ("sender_guild_id", "0"),
        ("amount", &burn_amount.to_string()),
        ("swapped_in", "100"),
        ("swapped_out_oppamint", "120"),
        ("swapped_out_ancs", "120"),
    ]));

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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(120));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(120));

    // balance of burn address terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu
    let balance = app
        .wrap()
        .query_balance(
            Addr::unchecked("terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu"),
            "uusd",
        )
        .unwrap();
    assert_eq!(balance.denom, "uusd");
    assert_eq!(balance.amount, Uint128::new(95)); // 5% deducted tax
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
        Some(REFERRER),
        Some(1706001506),
        None,
    );
    assert!(execute_res.is_ok());
    let execute_res: AppResponse = execute_res.unwrap();
    execute_res.assert_event(&Event::new("wasm").add_attributes(vec![
        ("action", "burn_uusd"),
        ("sender", USER),
        ("sender_guild_id", "0"),
        ("amount", &burn_amount.to_string()),
        ("swapped_in", "98"),
        ("swapped_out_oppamint", "118"),
        ("swapped_out_ancs", "117"),
    ]));

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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(118));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(117));
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
        Some(REFERRER),
        Some(1706001653),
        None,
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
    let modify_res = app.execute_contract(
        Addr::unchecked(ADMIN),
        burn_contract.addr(),
        &modify_msg,
        &[],
    );
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        Some(REFERRER),
        Some(1706001506),
        None,
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
    let modify_res = app.execute_contract(
        Addr::unchecked(ADMIN),
        burn_contract.addr(),
        &modify_msg,
        &[],
    );
    assert!(modify_res.is_ok());

    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);
    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        Some(REFERRER),
        Some(1706001506),
        None,
    );
    assert!(burn_res.is_ok());
    let burn_res: AppResponse = burn_res.unwrap();
    burn_res.assert_event(&Event::new("wasm").add_attributes(vec![
        ("action", "burn_uusd"),
        ("sender", USER),
        ("sender_guild_id", "0"),
        ("amount", &burn_amount.to_string()),
        ("swapped_in", "100"),
        ("swapped_out_oppamint", "120"),
        ("swapped_out_ancs", "120"),
    ]));

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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(120));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(120));
}

#[test]
pub fn success_over_min_amount_out() {
    let (mut app, burn_contract) = instantiate::default();
    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        Some(REFERRER),
        Some(1706001506),
        Some(OutputTokenMap {
            oppamint: Uint128::new(98),
            ancs: Uint128::new(98),
        }),
    );
    assert!(execute_res.is_ok());
    let execute_res: AppResponse = execute_res.unwrap();
    execute_res.assert_event(&Event::new("wasm").add_attributes(vec![
        ("action", "burn_uusd"),
        ("sender", USER),
        ("sender_guild_id", "0"),
        ("amount", &burn_amount.to_string()),
        ("swapped_in", "100"),
        ("swapped_out_oppamint", "120"),
        ("swapped_out_ancs", "120"),
    ]));

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
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(120));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(120));
}

#[test]
pub fn fail_under_min_amount_out() {
    let (mut app, burn_contract) = instantiate::default();
    // Try to burn some tokens for a user with a referrer.
    let burn_amount = Uint128::new(100);

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        Some(REFERRER),
        Some(1706001506),
        Some(OutputTokenMap {
            oppamint: Uint128::new(120),
            ancs: Uint128::new(130),
        }),
    );
    assert!(execute_res.is_err());
}
