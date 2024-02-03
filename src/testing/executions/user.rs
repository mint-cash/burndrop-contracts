use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, QueryMsg, UserInfoResponse};
use crate::testing::executions::swap::execute_swap;
use crate::testing::terra_bindings::TerraApp;
use crate::testing::{instantiate, ADMIN, REFERRER};
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;

fn assert_slots(app: &mut TerraApp, burn_contract: &BurnContract, user: &str, slots: Uint128) {
    let query_res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: user.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.slots, slots);
}

#[test]
fn success_first_referral() {
    let users = (1..=9)
        .map(|i| format!("user{}", i))
        .collect::<Vec<String>>();

    let (mut app, burn_contract) = instantiate::default_with_users(users.clone());

    // (user1, user2, user3, ..., user8) -> referrer1(REFERRER)
    for i in 1..=8 {
        let user = &users[i - 1];
        let msg = ExecuteMsg::RegisterStartingUser {
            user: user.to_string(),
        };
        app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
            .unwrap();

        let execute_res = execute_swap(
            &mut app,
            &burn_contract,
            &user,
            Uint128::new(100),
            REFERRER,
            Some(1706001506),
        );
        assert!(execute_res.is_ok());

        assert_slots(
            &mut app,
            &burn_contract,
            REFERRER,
            Uint128::new(2u128.pow(i as u32)),
        );
    }

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user9".to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        "user9",
        Uint128::new(100),
        REFERRER,
        Some(1706001506),
    );
    assert!(execute_res.is_ok());

    assert_slots(
        &mut app,
        &burn_contract,
        REFERRER,
        Uint128::new(2u128.pow(8)),
    );
}

#[test]
fn success_first_and_second_referral() {
    let users = (1..=4)
        .map(|i| format!("user{}", i))
        .collect::<Vec<String>>();

    let (mut app, burn_contract) = instantiate::default_with_users(users.clone());

    // (user1, user2, user3) -> referrer1(REFERRER)
    // user4 -> user2
    // user4 => referrer1(REFERRER)
    // -> : first , => : second

    for i in 1..=3 {
        let user = &users[i - 1];
        let msg = ExecuteMsg::RegisterStartingUser {
            user: user.to_string(),
        };
        app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
            .unwrap();

        let execute_res = execute_swap(
            &mut app,
            &burn_contract,
            &user,
            Uint128::new(100),
            REFERRER,
            Some(1706001506),
        );
        assert!(execute_res.is_ok());

        assert_slots(
            &mut app,
            &burn_contract,
            REFERRER,
            Uint128::new(2u128.pow(i as u32)),
        ); // 2 ^ i
    }

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user4".to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        "user4",
        Uint128::new(100),
        "user2",
        Some(1706001506),
    );
    assert!(execute_res.is_ok());

    let second_referral_msg = ExecuteMsg::Register2ndReferrer {
        referrer: REFERRER.to_string(),
    };
    let second_referral_res = app.execute_contract(
        Addr::unchecked("user4"),
        burn_contract.addr(),
        &second_referral_msg,
        &[],
    );
    assert!(second_referral_res.is_ok());

    assert_slots(&mut app, &burn_contract, REFERRER, Uint128::new(9)); // 2 ^ 3 + 1
}

#[test]
fn fail_when_first_referral_equals_second() {
    let (mut app, burn_contract) = instantiate::default();

    // user1 -> referrer1(REFERRER)
    // user1 => referrer1(REFERRER)
    // -> : first , => : second

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user1".to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        "user1",
        Uint128::new(100),
        REFERRER,
        Some(1706001506),
    );

    assert!(execute_res.is_ok());

    let second_referral_msg = ExecuteMsg::Register2ndReferrer {
        referrer: REFERRER.to_string(),
    };

    let second_referral_res = app.execute_contract(
        Addr::unchecked("user1"),
        burn_contract.addr(),
        &second_referral_msg,
        &[],
    );
    assert!(second_referral_res.is_err());
}

#[test]
fn fail_second_referral_with_no_first() {
    let (mut app, burn_contract) = instantiate::default();

    // user1 -> referrer1(REFERRER)
    // user2 => referrer1(REFERRER)
    // user2 => referrer1(REFERRER)
    // -> : first , => : second

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user1".to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        "user1",
        Uint128::new(100),
        REFERRER,
        Some(1706001506),
    );

    assert!(execute_res.is_ok());

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user2".to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    assert!(execute_res.is_ok());

    let second_referral_msg = ExecuteMsg::Register2ndReferrer {
        referrer: REFERRER.to_string(),
    };

    let second_referral_res = app.execute_contract(
        Addr::unchecked("user2"),
        burn_contract.addr(),
        &second_referral_msg,
        &[],
    );
    println!("{:#?}", second_referral_res);
    assert!(second_referral_res.is_err());
}
