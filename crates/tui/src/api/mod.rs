pub mod new;
mod auth;

use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::{AuthToken, PreAuthToken};
use pqcrypto::EncryptedMessage;
use pqcrypto::keys::KeyPair;
use pqcrypto::keys::public::Public;
use reqwest::Client;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Handshake Error")]
    Handshake,
}

pub struct ApiSession {
    session_id: Uuid,
    key_pair: KeyPair,
    server_public_key: Public,
    encrypted_pre_auth_token: Option<EncryptedMessage>,
    pre_auth_token: Option<PreAuthToken>,
    encrypted_auth_token: Option<EncryptedMessage>,
    auth_token: Option<AuthToken>,
    client: Client,
}

impl ApiSession {
    pub async fn handshake() -> Result<Self, ApiError> {
        let key_pair = KeyPair::generate().map_err(|error| {
            log::error!("Error creating KeyPair {error}");
            ApiError::Handshake
        })?;
        let client = Client::new();
        let request = HandshakeRequest {
            pub_key: key_pair.public.clone(),
        };

        let response: HandshakeResponse = client
            .post("http://localhost:3000/handshake")
            .json(&request)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let server_public_key = response.public_key;
        let token: PreAuthToken = key_pair
            .decrypt(&response.token, &server_public_key)
            .unwrap();
        let session_id = token.session_id;

        Ok(Self {
            session_id,
            key_pair,
            server_public_key,
            encrypted_pre_auth_token: Some(response.token),
            pre_auth_token: Some(token),
            encrypted_auth_token: None,
            auth_token: None,
            client,
        })
    }
}
