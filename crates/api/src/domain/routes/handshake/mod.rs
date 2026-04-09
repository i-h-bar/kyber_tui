use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::{Cache, CachedSession};
use crate::ports::services::pw_store::PWStore;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::PreAuthToken;
use kyber_crypto::keys::pair::KeyPair;
use kyber_crypto::keys::public::Public;
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::Instant;
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
        let client_pub_key = Public::from_b64(&payload.pub_key).map_err(|error| {
            log::warn!("Failed to load public key {}", error);
            DomainError::PermissionError("Invalid public key".to_string())
        })?;

        let start = Instant::now();
        let key_pair = KeyPair::generate().map_err(|error| {
            log::error!("Failed to generate key pair {}", error);
            DomainError::KeyError("Failed to generate key pair".to_string())
        })?;
        log::info!("Key pair generated in {:?}ms", start.elapsed().as_millis());

        let session_id = Uuid::new_v4();
        let session_expiry = SystemTime::now().add(Duration::from_secs(30));

        let token = PreAuthToken {
            session_id,
            expiry_s: session_expiry
                .duration_since(UNIX_EPOCH)
                .map_err(|error| {
                    log::error!("Failed to get session expiry time {}", error);
                    DomainError::GenericError("Failed to get session expiry time".to_string())
                })?
                .as_secs(),
        };

        let start = Instant::now();
        let (encrypted_token, shared_secret) = key_pair
            .encrypt_with_secret(&token.to_bytes(), &client_pub_key)
            .map_err(|error| {
                log::error!("Failed to encrypt token {}", error);
                DomainError::EncryptionError("Failed to encrypt token".to_string())
            })?;
        log::info!("Token encrypted in {:?}ms", start.elapsed().as_millis());

        let session = CachedSession {
            id: token.session_id,
            client_public_key: payload.pub_key,
            server_key_pair: key_pair.to_b64(),
            expiry: session_expiry,
            token_key: shared_secret,
            user_id: None,
        };

        self.cache.save_session(&session).await?;

        Ok(HandshakeResponse {
            public_key: key_pair.public.to_b64(),
            token: encrypted_token.to_b64(),
        })
    }
}
