use axum::{http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

mod db;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Nika" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
