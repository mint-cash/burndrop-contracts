use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Coin, Event, Timestamp, Uint128};
use cw_multi_test::{AppResponse, Executor};

use crate::executions::round::UpdateRoundParams;
use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, QueryMsg, RoundsResponse, UserInfoResponse};
use crate::testing::terra_bindings::TerraApp;
use crate::testing::utils::assert_strict_event_attributes;
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

pub fn assert_consistent_k(
    app: &mut TerraApp,
    burn_contract: &BurnContract,
    expected_k: OutputTokenMap<Uint128>,
) {
    // Query the burn info after burning tokens for the user.
    let query_res: RoundsResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::Rounds {})
        .unwrap();

    for round in query_res.rounds {
        let oppamint_k = round.oppamint_liquidity.x * round.oppamint_liquidity.y;
        let ancs_k = round.ancs_liquidity.x * round.ancs_liquidity.y;

        assert!(
            expected_k
                .oppamint
                .checked_sub(round.oppamint_liquidity.x)
                .unwrap_or(Uint128::zero())
                <= oppamint_k
                && oppamint_k <= expected_k.oppamint + round.oppamint_liquidity.x
        );
        assert!(
            expected_k
                .ancs
                .checked_sub(round.ancs_liquidity.x)
                .unwrap_or(Uint128::zero())
                <= ancs_k
                && ancs_k <= expected_k.ancs + round.ancs_liquidity.x
        );
    }
}

#[test]
fn success_during_period() {
    let (mut app, burn_contract) = instantiate::default();
    let burn_amount = Uint128::new(999 * (10u128).pow(6)); // 999 USTC

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
    let execute_res = execute_res.unwrap();
    assert_strict_event_attributes(
        execute_res.clone(),
        "wasm",
        vec![
            ("action", "burn_uusd"),
            ("sender", USER),
            ("sender_guild_id", "0"),
            ("referrer", REFERRER),
            ("amount", &burn_amount.to_string()),
            ("swapped_in", "999000000"),
            ("swapped_out_oppamint", "1197364599"),
            ("swapped_out_ancs", "1196886895"),
            ("_contract_address", burn_contract.addr().as_str()),
        ],
    );

    assert_consistent_k(
        &mut app,
        &burn_contract,
        OutputTokenMap {
            oppamint: Uint128::new(500_000_000000u128 * 1_000_000_000000u128),
            ancs: Uint128::new(250_000_000000u128 * 750_000_000000u128),
        },
    );

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
    assert_eq!(query_res.slot_size, Uint128::new(1000 * (10u128).pow(6))); // 1000000000
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000 * (10u128).pow(6)));

    assert_eq!(query_res.burned, Uint128::new(999 * (10u128).pow(6)));
    assert_eq!(query_res.burnable, Uint128::new(1 * (10u128).pow(6)));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(1197364599));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(1196886895));

    // balance of burn address terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu
    let balance = app
        .wrap()
        .query_balance(
            Addr::unchecked("terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu"),
            "uusd",
        )
        .unwrap();
    assert_eq!(balance.denom, "uusd");
    assert_eq!(balance.amount, Uint128::new(994029850)); // 994.02985 burned (deducted tax)
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
    execute_res
        .unwrap()
        .assert_event(&Event::new("wasm").add_attributes(vec![
            ("action", "burn_uusd"),
            ("sender", USER),
            ("sender_guild_id", "0"),
            ("referrer", REFERRER),
            ("amount", &burn_amount.to_string()),
            ("swapped_in", "98"),
            ("swapped_out_oppamint", "117"),
            ("swapped_out_ancs", "116"),
        ]));

    assert_consistent_k(
        &mut app,
        &burn_contract,
        OutputTokenMap {
            oppamint: Uint128::new(500_000_000000u128 * 1_000_000_000000u128),
            ancs: Uint128::new(250_000_000000u128 * 750_000_000000u128),
        },
    );

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
    assert_eq!(query_res.slot_size, Uint128::new(1000 * (10u128).pow(6)));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000 * (10u128).pow(6)));

    assert_eq!(query_res.burned, Uint128::new(98));
    assert_eq!(query_res.burnable, Uint128::new(999999902));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(117));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(116));
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

            oppamint_weight: None,
            ancs_weight: None,
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

            oppamint_weight: None,
            ancs_weight: None,
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

    assert_strict_event_attributes(
        burn_res.unwrap(),
        "wasm",
        vec![
            ("action", "burn_uusd"),
            ("sender", USER),
            ("sender_guild_id", "0"),
            ("referrer", REFERRER),
            ("amount", &burn_amount.to_string()),
            ("swapped_in", "100"),
            ("swapped_out_oppamint", "119"),
            ("swapped_out_ancs", "119"),
            ("_contract_address", burn_contract.addr().as_str()),
        ],
    );

    assert_consistent_k(
        &mut app,
        &burn_contract,
        OutputTokenMap {
            oppamint: Uint128::new(500_000_000000u128 * 1_000_000_000000u128),
            ancs: Uint128::new(250_000_000000u128 * 750_000_000000u128),
        },
    );

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
    assert_eq!(query_res.slot_size, Uint128::new(1000 * (10u128).pow(6)));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000 * (10u128).pow(6)));

    assert_eq!(query_res.burned, Uint128::new(100));
    assert_eq!(query_res.burnable, Uint128::new(999999900));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(119));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(119));
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
    assert_strict_event_attributes(
        execute_res.unwrap(),
        "wasm",
        vec![
            ("action", "burn_uusd"),
            ("sender", USER),
            ("sender_guild_id", "0"),
            ("referrer", REFERRER),
            ("amount", &burn_amount.to_string()),
            ("swapped_in", "100"),
            ("swapped_out_oppamint", "119"),
            ("swapped_out_ancs", "119"),
            ("_contract_address", burn_contract.addr().as_str()),
        ],
    );

    assert_consistent_k(
        &mut app,
        &burn_contract,
        OutputTokenMap {
            oppamint: Uint128::new(500_000_000000u128 * 1_000_000_000000u128),
            ancs: Uint128::new(250_000_000000u128 * 750_000_000000u128),
        },
    );

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
    assert_eq!(query_res.slot_size, Uint128::new(1000 * (10u128).pow(6)));
    assert_eq!(query_res.slots, Uint128::new(1));
    assert_eq!(query_res.cap, Uint128::new(1000 * (10u128).pow(6)));

    assert_eq!(query_res.burned, Uint128::new(100));
    assert_eq!(query_res.burnable, Uint128::new(999999900));
    assert_eq!(query_res.swapped_out.oppamint, Uint128::new(119));
    assert_eq!(query_res.swapped_out.ancs, Uint128::new(119));
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
