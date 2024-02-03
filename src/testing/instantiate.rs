use super::terra_bindings::TerraApp;
use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::{ADMIN, NATIVE_DENOM, REFERRER, SECOND_REFERRER, USER};
use crate::types::swap_round::{LiquidityPair, SwapRound};
use classic_bindings::{TerraMsg, TerraQuery};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_multi_test::{Contract, ContractWrapper, Executor};

// fn contract_template() -> Box<dyn Contract<TerraMsg, TerraQuery>> {
//     let contract = ContractWrapper::new(
//         crate::contract::execute,
//         crate::contract::instantiate,
//         crate::contract::query,
//     );
//     Box::new(contract)
// }

fn contract_template() -> Box<dyn Contract<TerraMsg, TerraQuery>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    // .with_sudo(crate::contract::sudo);
    Box::new(contract)
}

struct UserBalance {
    address: Addr,
    balance: Uint128,
}

fn mock_app(user_balances: Vec<UserBalance>) -> TerraApp {
    let mut app = TerraApp::new(Addr::unchecked(ADMIN).as_str());

    app.init_modules(|router, _, storage| {
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
    });

    app
}

pub fn default() -> (TerraApp, BurnContract) {
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
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    (app, burn_contract)
}

pub fn default_with_users(users: Vec<String>) -> (TerraApp, BurnContract) {
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
    app.execute_contract(Addr::unchecked(ADMIN), burn_contract.addr(), &msg, &[])
        .unwrap();

    (app, burn_contract)
}
