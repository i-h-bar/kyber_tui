pub mod add;
mod auth;
pub mod new;
pub mod get;

use crate::adapters::services::request::Client;
use crate::ports::services::request::RequestClient;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::{AuthToken, PreAuthToken};
use pqcrypto::EncryptedMessage;
use pqcrypto::asym::KeyPair;
use pqcrypto::asym::public::Public;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Handshake Error")]
    Handshake,

    #[error("Error authenticating")]
    Authenticate,

    #[error("Error encrypting message")]
    Encryption,

    #[error("Error decrypting message")]
    Decryption,

    #[error("Api response error {0}")]
    ApiResponse(String),

    #[error("Error sending request")]
    Send,

    #[error("Error deserialising response")]
    Deserialisation,
}

pub struct ApiSession {
    session_id: Uuid,
    key_pair: KeyPair,
    server_public_key: Public,
    encrypted_pre_auth_token: Option<EncryptedMessage>,
    encrypted_auth_token: Option<EncryptedMessage>,
    auth_token: Option<AuthToken>,
    client: Client,
    secret: Option<[u8; 32]>,
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
            .post("http://localhost:3000/handshake", &request)
            .await?;

        let server_public_key = response.public_key;
        let token: PreAuthToken = key_pair
            .decrypt(&response.token, &server_public_key)
            .map_err(|error| {
                log::error!("Error creating handshake {error}");
                ApiError::Decryption
            })?;
        let session_id = token.session_id;

        Ok(Self {
            session_id,
            key_pair,
            server_public_key,
            encrypted_pre_auth_token: Some(response.token),
            encrypted_auth_token: None,
            auth_token: None,
            client,
            secret: None,
        })
    }

    pub async fn authed(username: String, password: String) -> Result<Self, ApiError> {
        let mut session = ApiSession::handshake().await?;
        if session.authenticate(username, password).await? {
            Ok(session)
        } else {
            Err(ApiError::Authenticate)
        }
    }

    #[must_use]
    pub(crate) fn id(&self) -> &Uuid {
        &self.session_id
    }
}
