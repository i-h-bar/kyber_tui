mod ports;
mod domain;
mod adapters;

use axum::{routing::{get, post}, Router};
use domain::exchange::exchange;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(ready))
        .route("/exchange", post(exchange));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ready() -> &'static str {
    "Ready"
}

