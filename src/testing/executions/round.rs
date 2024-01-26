use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_multi_test::Executor;

use crate::executions::round::UpdateRoundParams;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::{instantiate, ADMIN};

#[test]
fn update_active_round() {
    let (mut app, burn_contract) = instantiate::default();

    // Query the rounds
    let query_res: crate::msg::RoundsResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::Rounds {})
        .unwrap();

    let prev_rounds = query_res.rounds;

    // Try to update active round
    let update_msg = ExecuteMsg::UpdateRound {
        params: UpdateRoundParams {
            id: 1,
            start_time: None,
            end_time: None,
            output_token: None,

            x_liquidity: None,
            y_liquidity: Some(Uint128::new(100_000)),
        },
    };
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1706001506); // in the executions period
    });
    let update_res = app.execute_contract(
        Addr::unchecked(ADMIN),
        burn_contract.addr(),
        &update_msg,
        &[],
    );

    assert!(update_res.is_err());

    // Query the rounds
    let query_res: crate::msg::RoundsResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::Rounds {})
        .unwrap();

    let rounds = query_res.rounds;

    assert_eq!(prev_rounds, rounds);
}
