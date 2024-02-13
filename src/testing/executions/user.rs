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
    let mut slots = 1;
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
            Some(REFERRER),
            Some(1706001506),
            None,
        );
        assert!(execute_res.is_ok());

        slots += 2u128.pow(i as u32);

        assert_slots(&mut app, &burn_contract, REFERRER, Uint128::new(slots));
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
        Some(REFERRER),
        Some(1706001506),
        None,
    );
    assert!(execute_res.is_ok());

    assert_slots(&mut app, &burn_contract, REFERRER, Uint128::new(512));
}
