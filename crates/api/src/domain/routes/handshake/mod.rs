use crate::ports::services::cache::{Cache, CachedSession};
use crate::ports::services::pw_store::PWStore;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::PreAuthToken;
use kyber_crypto::keys::pair::KeyPair;
use kyber_crypto::keys::public::Public;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::domain::Application;
use crate::domain::errors::routes::DomainError;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn handshake(
        &self,
        payload: HandshakeRequest,
    ) -> Result<HandshakeResponse, DomainError> {
        let client_pub_key =
            Public::from_b64(&payload.pub_key).map_err(|error| {
                log::warn!("Failed to load public key {}", error);
                DomainError::PermissionError("Invalid public key".to_string())
            })?;

        let key_pair = KeyPair::generate().map_err(|error| {
            log::error!("Failed to generate key pair {}", error);
            DomainError::KeyError("Failed to generate key pair".to_string())
        })?;

        let session = CachedSession {
            id: Uuid::new_v4(),
            client_public_key: payload.pub_key,
            server_key_pair: key_pair.to_b64(),
            expiry: SystemTime::now().add(Duration::from_secs(30)),
            user_id: None,
        };

        self.cache
            .save_session(&session)
            .await?;

        let token = PreAuthToken {
            session_id: session.id,
            expiry_s: session
                .expiry
                .duration_since(UNIX_EPOCH)
                .map_err(|error| {
                    log::error!("Failed to get session expiry time {}", error);
                    DomainError::GenericError("Failed to get session expiry time".to_string())
                })?
                .as_secs(),
        };

        Ok(HandshakeResponse {
            public_key: key_pair.public.to_b64(),
            token: key_pair
                .encrypt(&token.to_bytes(), &client_pub_key)
                .map_err(|error| {
                    log::error!("Failed to encrypt token {}", error);
                    DomainError::EncryptionError("Failed to encrypt token".to_string())
                })?
                .to_b64(),
        })
    }
}
