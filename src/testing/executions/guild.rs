use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::executions::swap::execute_swap;
use crate::testing::{instantiate, ADMIN, REFERRER, SECOND_REFERRER, USER};
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;

// `create_guild` test
// 1. USER1 : burn 100 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
// 2. USER1 : `create_guild` then check guild index and `guild_contributed_uusd` of user
// 3. USER1 : burn 200 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
#[test]
fn success_create_guild() {
    let (mut app, burn_contract) = instantiate::default();

    // Burn 100 uusd for the user
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

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 0);
    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(100));

    // Query the guild's burned uusd
    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 0 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(100));

    // Create a guild for the user
    let create_guild_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &crate::msg::ExecuteMsg::CreateGuild {
            name: "Test Guild".to_string(),
            slug: "test".to_string(),
            referrer: None,
        },
        &[],
    );

    assert!(create_guild_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 1);

    // Query the user's guild contributed uusd
    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(0));

    // Burn 200 uusd for the user
    let burn_amount = Uint128::new(200);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        None,
        Some(1706001507),
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild contributed uusd
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(200));

    // Query the guild's burned uusd

    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 1 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(200));
}

// `migrate_guild` test
// 1. USER1 : `create_guild` (guild_id=1)
// 2. USER1 : burn 100 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
// 3. USER2 : check current guild index
// 4. USER2 : burn 200 uusd then check `guild_contributed_uusd` of USER2 and `burned_uusd` of guild
// 5. USER2 : `migrate_guild` to guild_id=1 then check guild index and `guild_contributed_uusd`
// 6. USER2 : burn 300 uusd then check `guild_contributed_uusd` of USER2 and `burned_uusd` of guild
#[test]
fn success_migrate_guild() {
    let (mut app, burn_contract) = instantiate::default();

    let msg = ExecuteMsg::RegisterStartingUser {
        user: SECOND_REFERRER.to_string(),
    };
    let register_res =
        app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[]);

    assert!(register_res.is_ok());

    // Create a guild for the user
    let create_guild_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &crate::msg::ExecuteMsg::CreateGuild {
            name: "Test Guild".to_string(),
            slug: "test".to_string(),
            referrer: Some(REFERRER.to_string()),
        },
        &[],
    );

    assert!(create_guild_res.is_ok());

    // Burn 100 uusd for the user
    let burn_amount = Uint128::new(100);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        None,
        Some(1706001506),
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: USER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 1);
    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(100));

    // Query the guild's burned uusd
    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 1 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(100));

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: REFERRER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 0);

    // Burn 200 uusd for the user
    let burn_amount = Uint128::new(200);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        REFERRER,
        burn_amount,
        Some(SECOND_REFERRER),
        Some(1706001507),
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: REFERRER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 0);
    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(200));

    // Query the guild's burned uusd
    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 0 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(200));

    // Migrate the user's guild index
    let migrate_guild_res = app.execute_contract(
        Addr::unchecked(REFERRER),
        burn_contract.addr(),
        &crate::msg::ExecuteMsg::MigrateGuild {
            guild_id: 1,
            referrer: None,
        },
        &[],
    );

    assert!(migrate_guild_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: REFERRER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_id, 1);

    // Query the user's guild contributed uusd
    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(0));

    // Burn 300 uusd for the user
    let burn_amount = Uint128::new(300);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        REFERRER,
        burn_amount,
        None,
        Some(1706001508),
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild contributed uusd
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(
            burn_contract.addr(),
            &QueryMsg::UserInfo {
                address: REFERRER.to_string(),
            },
        )
        .unwrap();

    assert_eq!(query_res.guild_contributed_uusd, Uint128::new(300));

    // Query the guild's burned uusd
    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 1 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(400));

    // Query the guild's burned uusd
    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo { guild_id: 0 })
        .unwrap();

    assert_eq!(query_res.burned_uusd, Uint128::new(200));
}
