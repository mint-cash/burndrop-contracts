#[cfg(test)]
mod tests {
    use crate::helpers::BurnContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

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

        let instantiate_msg = InstantiateMsg {
            initial_slot_size: Uint128::new(1_000),
            sale_amount: Uint128::new(1_000_000),

            // Mocked initial liquidity.
            x_liquidity: Uint128::new(1_000_000),
            y_liquidity: Uint128::new(500_000),
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

    mod burn {
        use cosmwasm_std::testing::mock_info;

        use super::*;

        #[test]
        fn burn_tokens() {
            let (mut app, burn_contract) = proper_instantiate();
            // Try to burn some tokens for a user with a referrer.

            let burn_amount = Uint128::new(100);
            let sender_info = mock_info(
                USER,
                &vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: burn_amount,
                }],
            );

            // Create a burn tokens message
            let msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: REFERRER.to_string(),
            };
            let res = app.execute_contract(
                Addr::unchecked(USER),
                burn_contract.addr(),
                &msg,
                &sender_info.funds,
            );
            assert!(res.is_ok());

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
            assert_eq!(query_res.burned, burn_amount);

            // Test burning with the second referrer.
            let msg = ExecuteMsg::Register2ndReferrer {
                referrer: SECOND_REFERRER.to_string(),
            };
            let cosmos_msg = burn_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Query the burn info after registering the second referrer.
            // Similar query and assertions as above.

            // Query the user's swapped_out amount.
        }
        // Add more tests for other functionalities like error cases.
    }

    mod query_tests {
        use crate::states::config::Config;

        use super::*;

        #[test]
        fn query_config_test() {
            let (app, burn_contract) = proper_instantiate();

            let query_res: Config = app
                .wrap()
                .query_wasm_smart(burn_contract.addr(), &QueryMsg::Config {})
                .unwrap();

            assert_eq!(query_res.owner, Addr::unchecked(ADMIN));
            assert_eq!(query_res.slot_size, Uint128::new(1_000));
            assert_eq!(query_res.sale_amount, Uint128::new(1_000_000));
        }
    }
}
