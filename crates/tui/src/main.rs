use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use kyber_crypto::keys::KeyPair;

#[tokio::main]
async fn main() {
    let key_pair = KeyPair::generate().unwrap();

    let request = ExchangeRequest {
        pub_key: key_pair.public.to_b64(),
    };
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/exchange")
        .json(&request)
        .send()
        .await
        .unwrap();


    println!("{response:?}");
}
