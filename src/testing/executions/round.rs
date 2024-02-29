use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_multi_test::Executor;

use crate::executions::round::UpdateRoundParams;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::{instantiate, ADMIN};
use crate::types::swap_round::{LiquidityPair, SwapRound};

#[test]
fn success_update_active_round() {
    let (mut app, burn_contract) = instantiate::default();

    // Try to update active round
    let update_msg = ExecuteMsg::UpdateRound {
        params: UpdateRoundParams {
            id: 1,
            start_time: None,
            end_time: None,

            oppamint_liquidity: Some(LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(100_000),
            }),
            ancs_liquidity: Some(LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(100_000),
            }),

            oppamint_weight: None,
            ancs_weight: None,
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

    assert!(update_res.is_ok());

    // Query the rounds
    let query_res: crate::msg::RoundsResponse = app
        .wrap()
        .query_wasm_smart(burn_contract.addr(), &QueryMsg::Rounds {})
        .unwrap();

    let rounds = query_res.rounds;

    assert_eq!(
        rounds,
        vec![SwapRound {
            id: 1,
            start_time: 1706001400,
            end_time: 1706001650,
            oppamint_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(100_000),
            },
            ancs_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(100_000),
            },
            oppamint_weight: 3,
            ancs_weight: 2,
        }]
    );
}
