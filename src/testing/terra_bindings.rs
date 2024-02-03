use anyhow::{bail, Result as AnyResult};
use classic_bindings::{TaxCapResponse, TaxRateResponse, TerraMsg, TerraQuery};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    to_json_binary, Addr, Api, Binary, BlockInfo, CosmosMsg, CustomMsg, CustomQuery, Decimal,
    Empty, OwnedDeps, Querier, QuerierResult, StdResult, Storage, Timestamp, Uint128,
};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::cmp::max;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

// use tg_bindings::{
//     Evidence, GovProposal, ListPrivilegedResponse, Privilege, PrivilegeChangeMsg, PrivilegeMsg,
//     TerraMsg, TerraQuery, TerraSudoMsg, ValidatorDiff, ValidatorVote, ValidatorVoteResponse,
// };

// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum TerraQuery {
//     TaxCap { denom: String },
//     TaxRate {},
// }

// impl CustomQuery for TerraQuery {}

// impl From<ClassicQuery> for TerraQuery {
//     fn from(q: ClassicQuery) -> Self {
//         match q {
//             ClassicQuery::TaxCap { denom } => TerraQuery::TaxCap { denom },
//             ClassicQuery::TaxRate {} => TerraQuery::TaxRate {},
//             _ => panic!("Unsupported query: {:?}", q),
//         }
//     }
// }

pub struct TerraModule {}

/// How many seconds per block
/// (when we increment block.height, use this multiplier for block.time)
pub const BLOCK_TIME: u64 = 5;

pub type TerraDeps = OwnedDeps<MockStorage, MockApi, MockQuerier, TerraQuery>;

pub fn mock_deps_terra() -> TerraDeps {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}

impl TerraModule {
    pub fn set_owner(&self, storage: &mut dyn Storage, owner: &Addr) -> StdResult<()> {
        Ok(())
    }
}

impl Module for TerraModule {
    type ExecT = TerraMsg;
    type QueryT = TerraQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: TerraMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            _ => {
                // FIXME? We don't do anything here
                Ok(AppResponse::default())
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("sudo not implemented for TerraModule")
    }

    fn query(
        &self,
        _api: &dyn Api,
        storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: TerraQuery,
    ) -> AnyResult<Binary> {
        match request {
            TerraQuery::TaxCap { denom: _ } => Ok(to_json_binary(&TaxCapResponse {
                cap: Uint128::new(100),
            })
            .unwrap()),
            TerraQuery::TaxRate {} => Ok(to_json_binary(&TaxRateResponse {
                rate: Decimal::from_str("0.01").unwrap(),
            })
            .unwrap()),
            TerraQuery::ExchangeRates {
                base_denom,
                quote_denoms,
            } => todo!(),
            TerraQuery::Swap {
                offer_coin,
                ask_denom,
            } => todo!(),
            // _ => err_unsupported_query(request),
        }
    }
}

// #[derive(Error, Debug, PartialEq)]
// pub enum TerraError {
//     #[error("{0}")]
//     Std(#[from] StdError),

//     #[error("Unauthorized: {0}")]
//     Unauthorized(String),
// }

pub type TerraAppWrapped =
    App<BankKeeper, MockApi, MockStorage, TerraModule, WasmKeeper<TerraMsg, TerraQuery>>;

pub struct TerraApp(TerraAppWrapped);

impl Deref for TerraApp {
    type Target = TerraAppWrapped;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TerraApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for TerraApp {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl TerraApp {
    pub fn new(owner: &str) -> Self {
        let owner = Addr::unchecked(owner);
        Self(
            BasicAppBuilder::<TerraMsg, TerraQuery>::new_custom()
                .with_custom(TerraModule {})
                .build(|router, _, storage| {
                    router.custom.set_owner(storage, &owner).unwrap();
                }),
        )
    }

    pub fn new_genesis(owner: &str) -> Self {
        let owner = Addr::unchecked(owner);
        let block_info = BlockInfo {
            height: 0,
            time: Timestamp::from_nanos(1_571_797_419_879_305_533),
            chain_id: "Terra-testnet-14002".to_owned(),
        };

        Self(
            BasicAppBuilder::<TerraMsg, TerraQuery>::new_custom()
                .with_custom(TerraModule {})
                .with_block(block_info)
                .build(|router, _, storage| {
                    router.custom.set_owner(storage, &owner).unwrap();
                }),
        )
    }

    pub fn block_info(&self) -> BlockInfo {
        self.0.block_info()
    }

    // pub fn promote(&mut self, owner: &str, contract: &str) -> AnyResult<AppResponse> {
    //     let msg = TerraMsg::ExecuteGovProposal {
    //         title: "Promote Contract".to_string(),
    //         description: "Promote Contract".to_string(),
    //         proposal: GovProposal::PromoteToPrivilegedContract {
    //             contract: contract.to_string(),
    //         },
    //     };
    //     self.execute(Addr::unchecked(owner), msg.into())
    // }

    /// This reverses to genesis (based on current time/height)
    pub fn back_to_genesis(&mut self) {
        self.update_block(|block| {
            block.time = block.time.minus_seconds(BLOCK_TIME * block.height);
            block.height = 0;
        });
    }

    /// This advances BlockInfo by given number of blocks.
    /// It does not do any callbacks, but keeps the ratio of seconds/blokc
    pub fn advance_blocks(&mut self, blocks: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(BLOCK_TIME * blocks);
            block.height += blocks;
        });
    }

    /// This advances BlockInfo by given number of seconds.
    /// It does not do any callbacks, but keeps the ratio of seconds/blokc
    pub fn advance_seconds(&mut self, seconds: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += max(1, seconds / BLOCK_TIME);
        });
    }

    // /// next_block will call the end_blocker, increment block info 1 height and 5 seconds,
    // /// and then call the begin_blocker (with no evidence) in the next block.
    // /// It returns the validator diff if any.
    // ///
    // /// Simple iterator when you don't care too much about the details and just want to
    // /// simulate forward motion.
    // pub fn next_block(&mut self) -> AnyResult<Option<ValidatorDiff>> {
    //     let (_, diff) = self.end_block()?;
    //     self.update_block(|block| {
    //         block.time = block.time.plus_seconds(BLOCK_TIME);
    //         block.height += 1;
    //     });
    //     self.begin_block(vec![])?;
    //     Ok(diff)
    // }

    // /// Returns a list of all contracts that have the requested privilege
    // pub fn with_privilege(&self, requested: Privilege) -> AnyResult<Vec<Addr>> {
    //     let ListPrivilegedResponse { privileged } = self
    //         .wrap()
    //         .query(&TerraQuery::ListPrivileged(requested).into())?;
    //     Ok(privileged)
    // }

    // fn valset_updater(&self) -> AnyResult<Option<Addr>> {
    //     let mut updaters = self.with_privilege(Privilege::ValidatorSetUpdater)?;
    //     if updaters.len() > 1 {
    //         bail!("Multiple ValidatorSetUpdater registered")
    //     } else {
    //         Ok(updaters.pop())
    //     }
    // }

    // /// Make the BeginBlock sudo callback on all contracts that have registered
    // /// with the BeginBlocker Privilege
    // pub fn begin_block(&mut self, evidence: Vec<Evidence>) -> AnyResult<Vec<AppResponse>> {
    //     let to_call = self.with_privilege(Privilege::BeginBlocker)?;
    //     let msg = TerraSudoMsg::<Empty>::BeginBlock { evidence };
    //     let res = to_call
    //         .into_iter()
    //         .map(|contract: Addr| self.wasm_sudo(contract, &msg))
    //         .collect::<AnyResult<_>>()?;
    //     Ok(res)
    // }

    // /// Make the EndBlock sudo callback on all contracts that have registered
    // /// with the EndBlocker Privilege. Then makes the EndWithValidatorUpdate callback
    // /// on any registered valset_updater.
    // pub fn end_block(&mut self) -> AnyResult<(Vec<AppResponse>, Option<ValidatorDiff>)> {
    //     let to_call = self.with_privilege(Privilege::EndBlocker)?;
    //     let msg = TerraSudoMsg::<Empty>::EndBlock {};

    //     let mut res: Vec<AppResponse> = to_call
    //         .into_iter()
    //         .map(|contract| self.wasm_sudo(contract, &msg))
    //         .collect::<AnyResult<_>>()?;

    //     let diff = match self.valset_updater()? {
    //         Some(contract) => {
    //             let mut r =
    //                 self.wasm_sudo(contract, &TerraSudoMsg::<Empty>::EndWithValidatorUpdate {})?;
    //             let data = r.data.take();
    //             res.push(r);
    //             match data {
    //                 Some(b) if !b.is_empty() => Some(from_slice(&b)?),
    //                 _ => None,
    //             }
    //         }
    //         None => None,
    //     };
    //     Ok((res, diff))
    // }
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct UpgradePlan {
//     name: String,
//     height: u64,
//     info: String,
// }

// impl UpgradePlan {
//     pub fn new(name: impl ToString, height: u64, info: impl ToString) -> Self {
//         Self {
//             name: name.to_string(),
//             height,
//             info: info.to_string(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::msg::ExecuteMsg;

//     use super::*;
//     use cosmwasm_std::coin;
//     use cw_multi_test::Executor;

//     #[test]
//     fn init_and_owner_mints_tokens() {
//         let owner = Addr::unchecked("govner");
//         let rcpt = Addr::unchecked("townies");

//         let mut app = TerraApp::new(owner.as_str());

//         // no tokens
//         let start = app.wrap().query_all_balances(rcpt.as_str()).unwrap();
//         assert_eq!(start, vec![]);

//         // prepare to mint
//         let mintable = coin(123456, "shilling");
//         let msg = ExecuteMsg::RegisterStartingUser {
//             user: owner.into_string(),
//         };

//         // townies cannot
//         let _ = app.execute_contract(rcpt.clone(), '', &msg, &[]).unwrap_err();

//         // Gov'ner can
//         app.execute(owner, msg.into()).unwrap();

//         // we got tokens!
//         let end = app
//             .wrap()
//             .query_balance(rcpt.as_str(), &mintable.denom)
//             .unwrap();
//         assert_eq!(end, mintable);
//     }

//     // TODO: Delegate / Undelegate tests
// }
