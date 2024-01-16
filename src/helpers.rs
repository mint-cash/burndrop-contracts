use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};

use crate::msg::{ExecuteMsg, QueryMsg, UserInfoResponse};

/// BurnContract is a wrapper around Addr that provides a lot of helpers
/// for working with this contract specifically.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BurnContract(pub Addr);

impl BurnContract {
    /// Returns the contract address.
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    /// Helper to create a CosmosMsg to call a function on this contract.
    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Helper to query the burn information of a specific address.
    pub fn get_burn_info<Q, T, CQ>(&self, querier: &Q, address: T) -> StdResult<UserInfoResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::UserInfo {
            address: address.into(),
        };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg)?,
        }
        .into();
        let res: UserInfoResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
