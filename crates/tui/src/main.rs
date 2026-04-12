pub mod api;

use contracts::GenericRequest;
use contracts::auth::AuthRequest;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::PreAuthToken;
use dotenv::dotenv;
use pqcrypto::keys::pair::KeyPair;
use std::env;
use crate::api::ApiSession;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let session = ApiSession::handshake().await.unwrap();
    session.create_new_user("Other Username".to_string(), env::var("PASSWORD").unwrap()).await.unwrap();


    // let new_user = AuthRequest {
    //     username: "Other Username".to_string(),
    //     password: env::var("PASSWORD").unwrap(),
    // };
    //
    // println!("{}", token.session_id);
    // let new_user_request = GenericRequest {
    //     session_id: token.session_id,
    //     body: key_pair.encrypt(&new_user, &server_pub_key).unwrap(),
    //     token: response.token,
    // };
    //
    // let new_user_response = client
    //     .post("http://localhost:3000/auth")
    //     .json(&new_user_request)
    //     .send()
    //     .await
    //     .unwrap();
    //
    // println!("Response: {:?}", &new_user_response);

    // let new_user_response = new_user_response.json::<GenericResponse>().await.unwrap();
}
