pub mod api;

use crate::api::ApiSession;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let session = ApiSession::authed("Other Username".to_string(), env::var("PASSWORD").unwrap())
        .await
        .unwrap();

    println!("Session ID: {}", session.id());
}
