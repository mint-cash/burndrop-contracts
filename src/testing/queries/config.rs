use cosmwasm_std::{Addr, Uint128};

use crate::msg::QueryMsg;
use crate::states::config::Config;
use crate::testing::{instantiate, ADMIN};

#[test]
fn test_query_config() {
    let (app, burn_contract) = instantiate::default();

    let query_res: Config = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::Config {})
        .unwrap();

    assert_eq!(query_res.owner, Addr::unchecked(ADMIN));
    assert_eq!(query_res.slot_size, Uint128::new(1_000));
}
