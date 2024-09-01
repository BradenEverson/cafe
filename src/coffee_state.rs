use std::{fs::File, future::Future, io::Read, pin::Pin, sync::Arc};

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::order::Order;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct State {
    orders: Vec<Order>,
}

impl State {
    pub fn all_orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn add_order(&mut self, new_order: Order) {
        self.orders.push(new_order)
    }

    pub fn delete_order(&mut self, index: usize) -> bool {
        if index <= self.orders.len() {
            self.orders.remove(index);
            true
        } else {
            false
        }
    }
}

pub struct CoffeeService {
    state: Arc<RwLock<State>>,
}

impl CoffeeService {
    pub async fn add_order(&mut self, new_order: Order) {
        self.state.write().await.add_order(new_order)
    }

    pub async fn delete_order(&mut self, index: usize) -> bool {
        self.state.write().await.delete_order(index)
    }
}

impl CoffeeService {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }
}

impl Service<Request<body::Incoming>> for CoffeeService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder().status(StatusCode::OK);
        let state = self.clone();

        let res = match req.method() {
            &Method::GET => {
                let path = match req.uri().path() {
                    "/" => "frontend/index.html",
                    _ => "frontend/404.html",
                };

                let mut page = File::open(path).expect("Failed to open test file");
                let mut buf = vec![];
                page.read_to_end(&mut buf).expect("Failed to read file");

                response.body(Full::new(Bytes::copy_from_slice(&buf)))
            }
            &Method::POST => match req.uri().path() {
                "/new_coffee" => {
                    todo!("Match query parameters for size, special mode and time");
                    response.body(Full::new(Bytes::copy_from_slice(b"Added!")))
                }
                "/delete" => {
                    todo!("Match query parameters to get index of deleted request, delete it");
                }
                _ => response.body(Full::new(Bytes::copy_from_slice(&[]))),
            },
            _ => response.body(Full::new(Bytes::copy_from_slice(&[]))),
        };

        Box::pin(async { res })
    }
}
