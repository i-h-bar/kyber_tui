use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::Token;
use kyber_crypto::keys::pair::KeyPair;
use kyber_crypto::keys::public::Public;
use kyber_crypto::message::EncryptedMessage;

#[tokio::main]
async fn main() {
    let key_pair = KeyPair::generate().unwrap();

    let request = HandshakeRequest {
        pub_key: key_pair.public.to_b64(),
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

    let server_pub_key = Public::from_b64(&response.public_key).unwrap();
    let encrypted_message = EncryptedMessage::from_b64(&response.token).unwrap();
    let message = key_pair
        .decrypt(&encrypted_message, &server_pub_key)
        .unwrap();
    let token = Token::from_bytes(&message);

    println!("{token:?}");
}
