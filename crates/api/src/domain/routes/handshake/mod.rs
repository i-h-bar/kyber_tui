use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::PreAuthToken;
use pqcrypto::keys::pair::KeyPair;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn handshake(
        &self,
        payload: HandshakeRequest,
    ) -> Result<HandshakeResponse, DomainError> {
        let client_public_key = payload.pub_key;
        let server_key_pair = KeyPair::generate().map_err(|error| {
            log::error!("Failed to generate key pair {error}");
            DomainError::KeyError("Failed to generate key pair".to_string())
        })?;

        let session_id = Uuid::new_v4();
        let session_expiry = SystemTime::now().add(Duration::from_secs(30));

        let token = PreAuthToken {
            session_id,
            expiry_s: session_expiry
                .duration_since(UNIX_EPOCH)
                .map_err(|error| {
                    log::error!("Failed to get session expiry time {error}");
                    DomainError::Generic("Failed to get session expiry time".to_string())
                })?
                .as_secs(),
        };

        let (encrypted_token, shared_secret) = server_key_pair
            .encrypt_with_secret(&token, &client_public_key)
            .map_err(|error| {
                log::error!("Failed to encrypt token {error}");
                DomainError::Encryption("Failed to encrypt token".to_string())
            })?;

        let session = Session {
            id: token.session_id,
            client_public_key,
            server_key_pair,
            expiry: session_expiry,
            token_key: shared_secret,
            user_id: None,
        };

        self.cache.save_session(&session).await?;

        Ok(HandshakeResponse {
            public_key: session.server_key_pair.public,
            token: encrypted_token,
        })
    }
}
