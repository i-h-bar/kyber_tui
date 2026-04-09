use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use kyber_crypto::keys::KeyPair;
use kyber_crypto::keys::public::Public;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

pub struct Session {
    pub id: Uuid,
    pub client_public_key: Public,
    pub server_key_pair: KeyPair,
    pub expiry: SystemTime,
    pub token_key: [u8; 32],
    pub user_id: Option<Uuid>,
}

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn load_session_info(&self, session_id: Uuid) -> Result<Session, DomainError> {
        let session = self.cache.load_session(&session_id).await?;
        let server_key_pair = KeyPair::from_b64(&session.server_key_pair).map_err(|err| {
            log::error!("Error deserialising key pair {}", err);
            DomainError::DeserialisationError("Error deserialising key pair".to_string())
        })?;

        let client_public_key = Public::from_b64(&session.client_public_key).map_err(|err| {
            log::error!("Error deserialising client public key {}", err);
            DomainError::DeserialisationError("Error deserialising client public key".to_string())
        })?;

        Ok(Session {
            id: session.id,
            expiry: session.expiry,
            token_key: session.token_key,
            user_id: session.user_id,
            server_key_pair,
            client_public_key,
        })
    }
}
