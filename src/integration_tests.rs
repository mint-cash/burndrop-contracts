#[cfg(test)]
mod tests {
    use crate::helpers::BurnContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    // Define a function that returns the contract wrapper for your burn contract.
    pub fn contract_burn() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "uusd"; // Change to your token's denom.

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
                        amount: Uint128::new(1_000_000), // Set an initial amount.
                    }],
                )
                .unwrap();
        })
    }

    // Function to properly instantiate your contract in the mock app.
    fn proper_instantiate() -> (App, BurnContract) {
        let mut app = mock_app();
        let contract_burn_id = app.store_code(contract_burn());

        let instantiate_msg = InstantiateMsg {
            initial_slot_size: Uint128::new(1_000),
            sale_amount: Uint128::new(1_000_000),
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

        (app, BurnContract(contract_addr))
    }

    mod burn {
        use super::*;

        #[test]
        fn burn_tokens() {
            let (mut app, burn_contract) = proper_instantiate();

            // Try to burn some tokens.
            let burn_amount = Uint128::new(100); // Set the burn amount.
            let msg = ExecuteMsg::BurnTokens {
                amount: burn_amount,
                referrer: String::from(""), // TODO: Add mocked Terra address
            };
            let cosmos_msg = burn_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Query the burn info after burning tokens.
            let query_res: crate::msg::BurnInfoResponse = app
                .wrap()
                .query_wasm_smart(
                    burn_contract.addr(),
                    &QueryMsg::BurnInfo {
                        address: USER.to_string(),
                    },
                )
                .unwrap();

            // Perform assertions based on the expected state after burning tokens.
            assert_eq!(query_res.burned, burn_amount);
            // Add more assertions as needed.
        }
    }
}
