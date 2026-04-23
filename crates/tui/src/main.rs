pub mod adapters;
pub mod api;
pub mod ports;
pub mod utils;

use crate::api::ApiSession;
use crate::api::add::NewCredential;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut session =
        ApiSession::authed("Other Username".to_string(), env::var("PASSWORD").unwrap())
            .await
            .unwrap();
    let credential = NewCredential {
        service: "FakeService".to_string(),
        username: "FakeUsername".to_string(),
        password: "FakePassword".to_string(),
        notes: Some(vec![
            "FakeNotes1".to_string(),
            "FakeNotes2".to_string(),
            "FakeNotes3".to_string(),
        ]),
    };
    let credential_1 = session.get_credential_from_service(credential.service.clone()).await.unwrap();

    println!("Credential {credential_1:?}");
}
