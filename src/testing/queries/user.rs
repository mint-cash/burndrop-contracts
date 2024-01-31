use cosmwasm_std::Addr;
use cw_multi_test::Executor;

use crate::msg::QueryMsg::UsersInfo;
use crate::msg::{ExecuteMsg, UsersInfoResponse};
use crate::testing::{instantiate, ADMIN, SECOND_REFERRER, USER};
use crate::types::common::OrderBy;

#[test]
fn test_query_users() {
    let (mut app, burn_contract) = instantiate::default();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: SECOND_REFERRER.to_string(),
    };
    let cosmos_msg = burn_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: USER.to_string(),
    };
    let cosmos_msg = burn_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

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
