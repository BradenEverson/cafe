use serde::{Deserialize, Serialize};

use crate::order::Order;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct State {
    orders: Vec<Order>,
}
