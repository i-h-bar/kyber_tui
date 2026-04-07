use crate::domain::{Application, errors::routes::handshake::HandshakeError};
use crate::ports::services::cache::{Cache, CachedSession};
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use contracts::token::Token;
use kyber_crypto::keys::pair::KeyPair;
use kyber_crypto::keys::public::Public;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub async fn handshake(
        &self,
        payload: HandshakeRequest,
    ) -> Result<HandshakeResponse, HandshakeError> {
        let client_pub_key =
            Public::from_b64(&payload.pub_key).map_err(|_| HandshakeError::InvalidPublicKey)?;

        let key_pair = KeyPair::generate().map_err(|_| HandshakeError::KeyGenError)?;

        let session = CachedSession {
            id: Uuid::new_v4(),
            client_public_key: payload.pub_key,
            server_key_pair: key_pair.to_b64(),
            expiry: SystemTime::now().add(Duration::from_secs(30)),
            user_id: None,
        };

        self.cache
            .save_session(&session)
            .await
            .map_err(|_| HandshakeError::CacheError)?;

        let token = Token {
            session_id: session.id,
            expiry_s: session
                .expiry
                .duration_since(UNIX_EPOCH)
                .map_err(|_| HandshakeError::TokenCreationError)?
                .as_secs(),
        };

        Ok(HandshakeResponse {
            public_key: key_pair.public.to_b64(),
            token: key_pair
                .encrypt(&token.to_bytes(), &client_pub_key)
                .map_err(|_| HandshakeError::TokenEncryptionError)?
                .to_b64(),
        })
    }
}
