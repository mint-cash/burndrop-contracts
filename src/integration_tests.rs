#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use crate::helpers::BurnContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::types::output_token::{OutputToken, OutputTokenMap};
    use crate::types::swap_round::SwapRound;

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "user1";
    const REFERRER: &str = "referrer1";
    const SECOND_REFERRER: &str = "referrer2";
    const ADMIN: &str = "admin";
    const NATIVE_DENOM: &str = "uusd";

    // Set up a mock app environment.
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

    // Function to properly instantiate your contract in the mock app.
    fn proper_instantiate() -> (App, BurnContract) {
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

    mod execute_tests {
        use cosmwasm_std::testing::mock_info;
        use cosmwasm_std::Timestamp;

        use crate::executions::round::UpdateRoundParams;

        use super::*;

        #[test]
        fn test_burn_tokens_during_period() {
            let (mut app, burn_contract) = proper_instantiate();
            // Try to burn some tokens for a user with a referrer.
            let burn_amount = Uint128::new(100);
            let sender_info = mock_info(
                USER,
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: burn_amount,
                }],
            );

            // Create a burn tokens message
            let msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: REFERRER.to_string(),
            };
            app.update_block(|block| {
                block.time = Timestamp::from_seconds(1706001506); // in the swap period
            });
            let execute_res = app.execute_contract(
                Addr::unchecked(USER),
                burn_contract.addr(),
                &msg,
                &sender_info.funds,
            );
            assert!(execute_res.is_ok());

            // Query the burn info after burning tokens for the user.
            let query_res: crate::msg::UserInfoResponse = app
                .wrap()
                .query_wasm_smart(
                    burn_contract.addr(),
                    &QueryMsg::UserInfo {
                        address: USER.to_string(),
                    },
                )
                .unwrap();

            // Perform assertions based on the expected state after burning tokens.
            assert_eq!(query_res.slot_size, Uint128::new(1000));
            assert_eq!(query_res.slots, Uint128::new(1));
            assert_eq!(query_res.cap, Uint128::new(1000));

            assert_eq!(query_res.burned, Uint128::new(100));
            assert_eq!(query_res.burnable, Uint128::new(900));
            assert_eq!(query_res.swapped_out.oppamint, Uint128::new(196)); // 200 - virtual_slippage (4)
            assert_eq!(query_res.swapped_out.ancs, Uint128::new(0));
        }

        #[test]
        fn test_burn_tokens_not_period() {
            let (mut app, burn_contract) = proper_instantiate();
            // Try to burn some tokens for a user with a referrer.

            let burn_amount = Uint128::new(100);
            let sender_info = mock_info(
                USER,
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: burn_amount,
                }],
            );

            // Create a burn tokens message
            let msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: REFERRER.to_string(),
            };
            app.update_block(|block| {
                block.time = Timestamp::from_seconds(1706001653); // not in time
            });
            let execute_res = app.execute_contract(
                Addr::unchecked(USER),
                burn_contract.addr(),
                &msg,
                &sender_info.funds,
            );
            assert!(execute_res.is_err());
        }

        #[test]
        fn test_burn_tokens_not_modified_period() {
            let (mut app, burn_contract) = proper_instantiate();

            // Modify the swap period to be shorter.
            let modify_msg = ExecuteMsg::UpdateRound {
                params: UpdateRoundParams {
                    id: 1,
                    start_time: None,
                    end_time: Some(1706001500), // modify end_time
                    output_token: None,

                    x_liquidity: None,
                    y_liquidity: None,
                },
            };
            let cosmos_msg = burn_contract.call(modify_msg).unwrap();
            let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
            assert!(modify_res.is_ok());

            // Try to burn some tokens for a user with a referrer.
            let burn_amount = Uint128::new(100);
            let sender_info = mock_info(
                USER,
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: burn_amount,
                }],
            );

            // Create a burn tokens message
            let burn_msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: REFERRER.to_string(),
            };
            app.update_block(|block| {
                block.time = Timestamp::from_seconds(1706001506); // in the previous swap period, not in the modified period
            });
            let burn_res = app.execute_contract(
                Addr::unchecked(USER),
                burn_contract.addr(),
                &burn_msg,
                &sender_info.funds,
            );
            assert!(burn_res.is_err());
        }

        #[test]
        fn test_burn_tokens_during_modified_period() {
            let (mut app, burn_contract) = proper_instantiate();

            // Modify the swap period to be shorter.
            let modify_msg = ExecuteMsg::UpdateRound {
                params: UpdateRoundParams {
                    id: 1,
                    start_time: None,
                    end_time: Some(1706001700), // modify end_time
                    output_token: None,

                    x_liquidity: None,
                    y_liquidity: None,
                },
            };
            let cosmos_msg = burn_contract.call(modify_msg).unwrap();
            let modify_res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
            assert!(modify_res.is_ok());

            // Try to burn some tokens for a user with a referrer.
            let burn_amount = Uint128::new(100);
            let sender_info = mock_info(
                USER,
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: burn_amount,
                }],
            );

            // Create a burn tokens message
            let burn_msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: REFERRER.to_string(),
            };
            app.update_block(|block| {
                block.time = Timestamp::from_seconds(1706001680); // in the modified swap period
            });
            let burn_res = app.execute_contract(
                Addr::unchecked(USER),
                burn_contract.addr(),
                &burn_msg,
                &sender_info.funds,
            );
            assert!(burn_res.is_ok());

            // Query the burn info after burning tokens for the user.
            let query_res: crate::msg::UserInfoResponse = app
                .wrap()
                .query_wasm_smart(
                    burn_contract.addr(),
                    &QueryMsg::UserInfo {
                        address: USER.to_string(),
                    },
                )
                .unwrap();

            // Perform assertions based on the expected state after burning tokens.
            assert_eq!(query_res.slot_size, Uint128::new(1000));
            assert_eq!(query_res.slots, Uint128::new(1));
            assert_eq!(query_res.cap, Uint128::new(1000));

            assert_eq!(query_res.burned, Uint128::new(100));
            assert_eq!(query_res.burnable, Uint128::new(900));
            assert_eq!(query_res.swapped_out.oppamint, Uint128::new(196)); // 200 - virtual_slippage (4)
            assert_eq!(query_res.swapped_out.ancs, Uint128::new(0));
        }

        #[test]
        fn update_active_round() {
            let (mut app, burn_contract) = proper_instantiate();

            // Query the rounds
            let query_res: crate::msg::RoundsResponse = app
                .wrap()
                .query_wasm_smart(burn_contract.addr(), &QueryMsg::Rounds {})
                .unwrap();

            let prev_rounds = query_res.rounds;
            println!("rounds (prev): {:?}", prev_rounds);

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
                block.time = Timestamp::from_seconds(1706001506); // in the swap period
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

            println!("rounds (after): {:?}", rounds);

            assert_eq!(prev_rounds, rounds);
        }
    }

    mod query_tests {
        use crate::states::config::Config;

        use super::*;

        #[test]
        fn test_query_config() {
            let (app, burn_contract) = proper_instantiate();

            let query_res: Config = app
                .wrap()
                .query_wasm_smart(burn_contract.addr(), &QueryMsg::Config {})
                .unwrap();

            assert_eq!(query_res.owner, Addr::unchecked(ADMIN));
            assert_eq!(query_res.slot_size, Uint128::new(1_000));
            assert_eq!(query_res.sale_amount.oppamint, Uint128::new(1_000_000));
            assert_eq!(query_res.sale_amount.ancs, Uint128::new(1_000_000));
        }
    }
}
