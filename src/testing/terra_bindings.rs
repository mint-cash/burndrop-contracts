use anyhow::{bail, Result as AnyResult};
use classic_bindings::{TaxCapResponse, TaxRateResponse, TerraMsg, TerraQuery};
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    to_json_binary, Addr, Api, Binary, BlockInfo, CustomQuery, Decimal, Empty, Querier,
    QuerierResult, StdResult, Storage, Uint128,
};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub struct TerraModule {}

impl TerraModule {
    pub fn set_owner(&self, _storage: &mut dyn Storage, _owner: &Addr) -> StdResult<()> {
        Ok(())
    }
}

impl Module for TerraModule {
    type ExecT = TerraMsg;
    type QueryT = TerraQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        msg: TerraMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            _ => Ok(AppResponse::default()),
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
        _storage: &dyn Storage,
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
                base_denom: _,
                quote_denoms: _,
            } => todo!(),
            TerraQuery::Swap {
                offer_coin: _,
                ask_denom: _,
            } => todo!(),
        }
    }
}

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
}
