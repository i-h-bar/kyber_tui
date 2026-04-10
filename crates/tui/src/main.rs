use contracts::GenericRequest;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::new_user::NewUserRequest;
use contracts::token::PreAuthToken;
use dotenv::dotenv;
use kyber_crypto::keys::pair::KeyPair;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let key_pair = KeyPair::generate().unwrap();

    let request = HandshakeRequest {
        pub_key: key_pair.public.clone(),
    };
    let client = reqwest::Client::new();

    let response: HandshakeResponse = client
        .post("http://localhost:3000/handshake")
        .json(&request)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let server_pub_key = response.public_key;
    let token: PreAuthToken = key_pair.decrypt(&response.token, &server_pub_key).unwrap();

    let new_user = NewUserRequest {
        username: "Other Username".to_string(),
        password: env::var("PASSWORD").unwrap(),
    };

    println!("{}", token.session_id);
    let new_user_request = GenericRequest {
        session_id: token.session_id,
        body: key_pair.encrypt(&new_user, &server_pub_key).unwrap(),
        token: response.token,
    };

    let new_user_response = client
        .post("http://localhost:3000/new")
        .json(&new_user_request)
        .send()
        .await
        .unwrap();

    println!("Response: {:?}", &new_user_response);

    // let new_user_response = new_user_response.json::<GenericResponse>().await.unwrap();
}
