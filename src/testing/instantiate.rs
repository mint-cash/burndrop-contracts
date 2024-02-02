use cosmwasm_std::{Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::{ADMIN, NATIVE_DENOM, REFERRER, SECOND_REFERRER, USER};
use crate::types::swap_round::{LiquidityPair, SwapRound};

fn contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

struct UserBalance {
    address: Addr,
    balance: Uint128,
}

fn mock_app(user_balances: Vec<UserBalance>) -> App {
    AppBuilder::new().build(|router, _, storage| {
        for user_balance in user_balances {
            router
                .bank
                .init_balance(
                    storage,
                    &user_balance.address,
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: user_balance.balance,
                    }],
                )
                .unwrap();
        }
    })
}

pub fn default() -> (App, BurnContract) {
    let mut app = mock_app(vec![
        UserBalance {
            address: Addr::unchecked(USER),
            balance: Uint128::new(1_000_000),
        },
        UserBalance {
            address: Addr::unchecked(REFERRER),
            balance: Uint128::new(500_000),
        },
        UserBalance {
            address: Addr::unchecked(SECOND_REFERRER),
            balance: Uint128::new(500_000),
        },
    ]);

    let contract_burn_id = app.store_code(contract_template());

    let instantiate_msg: InstantiateMsg = InstantiateMsg {
        initial_slot_size: Uint128::new(1_000),

        rounds: vec![SwapRound {
            id: 1,

            start_time: 1706001400,
            end_time: 1706001650,

            oppamint_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(500_000),
            },
            ancs_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(500_000),
            },
        }],

        max_query_limit: 30,
        default_query_limit: 10,
    };
    let contract_addr = app
        .instantiate_contract(
            contract_burn_id,
            Addr::unchecked(ADMIN),
            &instantiate_msg,
            &[],
            "Burn Contract",
            None,
        )
        .unwrap();

    // owner should register REFERRER as starting_user
    let burn_contract = BurnContract(contract_addr);
    let msg = ExecuteMsg::RegisterStartingUser {
        user: REFERRER.to_string(),
    };
    let cosmos_msg = burn_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

    (app, burn_contract)
}

pub fn default_with_users(users: Vec<String>) -> (App, BurnContract) {
    let user_balances = users
        .iter()
        .map(|user| UserBalance {
            address: Addr::unchecked(user.clone()),
            balance: Uint128::new(500_000),
        })
        .collect();

    let mut app = mock_app(user_balances);

    let contract_burn_id = app.store_code(contract_template());

    let instantiate_msg: InstantiateMsg = InstantiateMsg {
        initial_slot_size: Uint128::new(1_000),

        rounds: vec![SwapRound {
            id: 1,

            start_time: 1706001400,
            end_time: 1706001650,

            oppamint_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(500_000),
            },
            ancs_liquidity: LiquidityPair {
                x: Uint128::new(1_000_000),
                y: Uint128::new(500_000),
            },
        }],

        max_query_limit: 30,
        default_query_limit: 10,
    };
    let contract_addr = app
        .instantiate_contract(
            contract_burn_id,
            Addr::unchecked(ADMIN),
            &instantiate_msg,
            &[],
            "Burn Contract",
            None,
        )
        .unwrap();

    // owner should register REFERRER as starting_user
    let burn_contract = BurnContract(contract_addr);
    let msg = ExecuteMsg::RegisterStartingUser {
        user: REFERRER.to_string(),
    };
    let cosmos_msg = burn_contract.call(msg).unwrap();
    app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

    (app, burn_contract)
}
