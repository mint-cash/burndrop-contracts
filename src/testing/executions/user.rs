use crate::msg::{ExecuteMsg, QueryMsg, UserInfoResponse};
use crate::testing::{ADMIN, instantiate, REFERRER};
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;
use crate::testing::executions::swap::execute_swap;

#[test]
fn success_first_referral() {
    let users = (1..=9).map(|i| format!("user{}", i)).collect::<Vec<String>>();

    let (mut app, burn_contract) = instantiate::default_with_users(users.clone());

    // (user1, user2, user3, ..., user8) -> referrer1(REFERRER)
    for i in 1..=8 {
        let user= &users[i - 1];
        let msg = ExecuteMsg::RegisterStartingUser {
            user: user.to_string(),
        };
        let cosmos_msg = burn_contract.call(msg).unwrap();
        app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

        let execute_res = execute_swap(
            &mut app,
            &burn_contract,
            &user,
            Uint128::new(100),
            REFERRER,
            Some(1706001506),
        );
        assert!(execute_res.is_ok());

        let query_res: UserInfoResponse = app
            .wrap()
            .query_wasm_smart(
                burn_contract.addr(),
                &QueryMsg::UserInfo {
                    address: REFERRER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(query_res.slots, Uint128::new(2u128.pow(i as u32))); // 2 ^ i
    }

    let msg = ExecuteMsg::RegisterStartingUser {
        user: "user9".to_string(),
    };
    let cosmos_msg = burn_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

    let execute_res = execute_swap(
        &mut app,
        &burn_contract,
        "user9",
        Uint128::new(100),
        REFERRER,
        Some(1706001506),
    );
    assert!(execute_res.is_ok());

    let query_res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: REFERRER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.slots, Uint128::new(2u128.pow(8))); // 2 ^ 8
}
