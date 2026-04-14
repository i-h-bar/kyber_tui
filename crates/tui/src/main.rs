pub mod api;
pub mod utils;

use crate::api::ApiSession;
use dotenv::dotenv;
use std::env;
use contracts::new_credential::NewCredentialRequest;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut session = ApiSession::authed("Other Username".to_string(), env::var("PASSWORD").unwrap())
        .await
        .unwrap();

    // let Ok(added) = session.add_credential().await;

    println!("Session ID: {}", session.id());
}
