use cosmwasm_std::{Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

use crate::helpers::BurnContract;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::{ADMIN, NATIVE_DENOM, REFERRER, SECOND_REFERRER, USER};
use crate::types::output_token::{OutputToken, OutputTokenMap};
use crate::types::swap_round::SwapRound;

fn contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(USER),
                vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(REFERRER),
                vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(500_000),
                }],
            )
            .unwrap();
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(SECOND_REFERRER),
                vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(500_000),
                }],
            )
            .unwrap();
    })
}

pub fn default() -> (App, BurnContract) {
    let mut app = mock_app();
    let contract_burn_id = app.store_code(contract_template());

    let instantiate_msg: InstantiateMsg = InstantiateMsg {
        initial_slot_size: Uint128::new(1_000),
        sale_amount: OutputTokenMap {
            oppamint: Uint128::new(1_000_000),
            ancs: Uint128::new(1_000_000),
        },

        rounds: vec![SwapRound {
            id: 1,

            start_time: 1706001400,
            end_time: 1706001650,

            output_token: OutputToken::OppaMINT,

            x_liquidity: Uint128::new(1_000_000),
            y_liquidity: Uint128::new(500_000),
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
