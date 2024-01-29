use cosmwasm_std::Order;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    Ascending,
    Descending,
}

impl From<OrderBy> for Order {
    fn from(order: OrderBy) -> Order {
        if order == OrderBy::Ascending {
            Order::Ascending
        } else {
            Order::Descending
        }
    }
}
