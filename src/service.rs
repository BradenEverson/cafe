use std::sync::Arc;

use tokio::sync::RwLock;

use crate::coffee_state::State;

pub struct CoffeeService {
    order_state: Arc<RwLock<State>>,
}
