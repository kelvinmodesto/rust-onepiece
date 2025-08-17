use serde::{Deserialize, Serialize};

mod config;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();
}
