use crate::executions::overridden_rounds::CreateOverriddenRoundParams;
use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::executions::swap::execute_swap;
use crate::testing::terra_bindings::TerraApp;
use crate::testing::{instantiate, ADMIN, REFERRER, SECOND_REFERRER, USER};
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_multi_test::Executor;

pub fn assert_current_user_info(
    app: &mut TerraApp,
    burn_contract: &BurnContract,
    expected_cap: Uint128,
    expected_burnable: Uint128,
    expected_burned: Uint128,
) {
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_res.cap, expected_cap);
    assert_eq!(query_res.burnable, expected_burnable);
    assert_eq!(query_res.burned, expected_burned);
}

#[test]
fn have_overridden_rounds() {
    let event_start_time = 1706001506;
    let event_end_time = 1706001550; // some time in the future

    let (mut app, burn_contract) = instantiate::default();

    // slots: 1, slot_size: 1000, cap: 1000, burnable: 1000
    // assert_current_burnable(
    //     &mut app,
    //     &burn_contract,
    //     Uint128::new(1000 * (10u128).pow(6)),
    // );

    // modify - burn 999
    // [post burn] slots: 1, slot_size: 1000, cap: 1000, burnable: 1
    let user_execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        Uint128::new(999 * (10u128).pow(6)),
        Some(REFERRER),
        Some(event_start_time),
        None,
    );
    assert!(user_execute_res.is_ok());
    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(1000 * (10u128).pow(6)),
        Uint128::new(1 * (10u128).pow(6)),
        Uint128::new(999 * (10u128).pow(6)),
    );

    // modify - add one slot (another user (`SECOND_REFERRER`) registers with referrer as USER)
    // slots: 3, slot_size: 1000, cap: 3000, burnable: 2001
    let extra_execute_res = app.execute_contract(
        Addr::unchecked(SECOND_REFERRER),
        burn_contract.addr(),
        &ExecuteMsg::CreateGuild {
            name: "First Commit".to_string(),
            referrer: Some(USER.to_string()),
        },
        &[],
    );
    assert!(extra_execute_res.is_ok());
    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(3000 * (10u128).pow(6)),
        Uint128::new(2001 * (10u128).pow(6)),
        Uint128::new(999 * (10u128).pow(6)),
    );

    // 10x EVENT!
    // slots: 3, slot_size: 10000, cap: 30000, burnable: 29001
    let admin_execute_res = app.execute_contract(
        Addr::unchecked(ADMIN),
        burn_contract.addr(),
        &ExecuteMsg::CreateOverriddenRound(CreateOverriddenRoundParams {
            start_time: event_start_time,
            end_time: event_end_time,
            slot_size: Uint128::new(10000 * (10u128).pow(6)),
        }),
        &[],
    );
    assert!(admin_execute_res.is_ok());

    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(30000 * (10u128).pow(6)),
        Uint128::new(29001 * (10u128).pow(6)),
        Uint128::new(999 * (10u128).pow(6)),
    );

    // modify - burn 9000
    // [post burn] slots: 3, slot_size: 10000, cap: 30000, burnable: 20001
    let user_execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        Uint128::new(9000 * (10u128).pow(6)),
        None,
        Some(event_start_time + 10),
        None,
    );
    assert!(user_execute_res.is_ok());
    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(30000 * (10u128).pow(6)),
        Uint128::new(20001 * (10u128).pow(6)),
        Uint128::new(9999 * (10u128).pow(6)),
    );

    // 10x EVENT OVER!
    // end event by natural time passing
    // slots: 3, slot_size: 1000, cap: 3000, burnable: 2001 (이벤트 전에 999 태웠으니), total burned: 9999
    app.update_block(|block| {
        // event time is inclusive range
        block.time = Timestamp::from_seconds(event_end_time + 1);
    });
    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(3000 * (10u128).pow(6)),
        Uint128::new(2001 * (10u128).pow(6)),
        Uint128::new(9999 * (10u128).pow(6)),
    );

    // modify - burn 2001
    // [post burn] slots: 3, slot_size: 1000, cap: 3000, burnable: 0, total burned: 12000
    let user_execute_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        Uint128::new(2001 * (10u128).pow(6)),
        None,
        Some(event_end_time + 2),
        None,
    );
    assert!(user_execute_res.is_ok());
    assert_current_user_info(
        &mut app,
        &burn_contract,
        Uint128::new(3000 * (10u128).pow(6)),
        Uint128::zero(),
        Uint128::new(12000 * (10u128).pow(6)),
    );
}
