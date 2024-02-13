use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;
use crate::msg::QueryMsg;
use crate::testing::{instantiate, REFERRER, USER};
use crate::testing::executions::swap::execute_swap;

// `create_guild` test
// 1. USER1 : check USER1's current guild index
// 2. USER1 : burn 100 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
// 3. USER1 : `create_guild` then check guild index and `guild_contributed_uusd` of user
// 4. USER1 : burn 200 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
#[test]
fn create_guild() {
    let (mut app, burn_contract) = instantiate::default();

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::UserInfo {
            address: USER.to_string(),
        })
        .unwrap();
    let prev_guild_index = query_res.guild_id;

    assert_eq!(prev_guild_index, 0);

    // Burn 100 uusd for the user
    let burn_amount = Uint128::new(100);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        None,
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::UserInfo {
            address: USER.to_string(),
        })
        .unwrap();

    let guild_contributed_uusd = query_res.guild_contributed_uusd;
    assert_eq!(guild_contributed_uusd, Uint128::new(100));

    // Query the guild's burned uusd

    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo {
            guild_id: 1,
        })
        .unwrap();

    let burned_uusd = query_res.burned_uusd;
    assert_eq!(burned_uusd, Uint128::new(100));

    // Create a guild for the user
    let create_guild_res = app.execute_contract(
        Addr::unchecked(USER),
        burn_contract.addr(),
        &crate::msg::ExecuteMsg::CreateGuild {
            name: "Test Guild".to_string(),
            slug: "test".to_string(),
        },
        &[]
    );

    assert!(create_guild_res.is_ok());

    // Query the user's guild index
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::UserInfo {
            address: USER.to_string(),
        })
        .unwrap();

    let guild_index = query_res.guild_id;
    assert_eq!(guild_index, 1);

    // Query the user's guild contributed uusd
    let guild_contributed_uusd = query_res.guild_contributed_uusd;
    assert_eq!(guild_contributed_uusd, Uint128::new(0));

    // Burn 200 uusd for the user
    let burn_amount = Uint128::new(200);

    let burn_res = execute_swap(
        &mut app,
        &burn_contract,
        USER,
        burn_amount,
        REFERRER,
        None,
        None,
    );

    assert!(burn_res.is_ok());

    // Query the user's guild contributed uusd
    let query_res: crate::msg::UserInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::UserInfo {
            address: USER.to_string(),
        })
        .unwrap();

    let guild_contributed_uusd = query_res.guild_contributed_uusd;
    assert_eq!(guild_contributed_uusd, Uint128::new(200));

    // Query the guild's burned uusd

    let query_res: crate::msg::GuildInfoResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::GuildInfo {
            guild_id: 1,
        })
        .unwrap();

    let burned_uusd = query_res.burned_uusd;
    assert_eq!(burned_uusd, Uint128::new(300));
}



// `migrate_guild` test
// 1. USER1 : `create_guild` (guild_id=1)
// 2. USER1 : burn 100 uusd then check `guild_contributed_uusd` of USER1 and `burned_uusd` of guild
// 3. USER2 : check current guild index
// 4. USER2 : burn 200 uusd then check `guild_contributed_uusd` of USER2 and `burned_uusd` of guild
// 5. USER2 : `migrate_guild` to guild_id=1 then check guild index and `guild_contributed_uusd`
// 6. USER2 : burn 300 uusd then check `guild_contributed_uusd` of USER2 and `burned_uusd` of guild