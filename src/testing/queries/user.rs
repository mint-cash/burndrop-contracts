use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;

use crate::msg::QueryMsg::{UserBalance, UserInfo, UsersInfo};
use crate::msg::{ExecuteMsg, UserBalanceResponse, UserInfoResponse, UsersInfoResponse};
use crate::testing::{instantiate, ADMIN, SECOND_REFERRER, USER};
use crate::types::common::OrderBy;

#[test]
fn test_query_users() {
    let (mut app, burn_contract) = instantiate::default();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: SECOND_REFERRER.to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: USER.to_string(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let query_res: UsersInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &UsersInfo {
                start: None,
                limit: Some(10),
                order: None,
            },
        )
        .unwrap();

    let users = query_res.users;

    assert_eq!(users.len(), 3);
    assert_eq!(users[0].0, "referrer1");
    assert_eq!(users[1].0, "referrer2");
    assert_eq!(users[2].0, "user1");

    let query_res: UsersInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &UsersInfo {
                start: None,
                limit: Some(10),
                order: Some(OrderBy::Descending),
            },
        )
        .unwrap();

    let users = query_res.users;

    assert_eq!(users.len(), 3);
    assert_eq!(users[2].0, "referrer1");
    assert_eq!(users[1].0, "referrer2");
    assert_eq!(users[0].0, "user1");

    let query_res: UsersInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &UsersInfo {
                start: Some("referrer1".to_string()),
                limit: Some(10),
                order: Some(OrderBy::Ascending),
            },
        )
        .unwrap();

    let users = query_res.users;

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].0, "referrer2");
    assert_eq!(users[1].0, "user1");
}

#[test]
fn test_query_compensation() {
    let (mut app, burn_contract) = instantiate::default();

    let address = "terra1rw43xy5388meu2pl3v02ckt8c754r73yhhv6qq".to_string();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: address.clone(),
    };
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    let query_res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &UserInfo {
                address: address.clone(),
            },
        )
        .unwrap();

    assert_eq!(query_res.compensation.oppamint, Uint128::new(365093610));
    assert_eq!(query_res.compensation.ancs, Uint128::new(365093610));

    let query_res: UserBalanceResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &UserBalance {
                address: address.clone(),
            },
        )
        .unwrap();

    assert_eq!(query_res.compensation.oppamint, Uint128::new(365093610));
    assert_eq!(query_res.compensation.ancs, Uint128::new(365093610));
}
