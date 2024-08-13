use std::sync::Arc;

use cafe::coffee_state::State;
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(State::default()));

    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    println!(
        "Listening on port {}",
        listener.local_addr().unwrap().port()
    );

    let state_clone = state.clone();
    let connection_handler = async move {
        let state = state_clone.clone();
        loop {
            // Handle connections
        }
    };

    let state_clone = state.clone();
    tokio::spawn(async move {
        let state = state_clone.clone();
        // Check if it's coffee time for any of our orders
        loop {}
    });

    tokio::spawn(async move {
        let state = state.clone();
        // Export Coffee orders to JSON every 30 minutes
    });

    connection_handler.await
}
