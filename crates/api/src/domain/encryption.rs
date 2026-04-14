use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub fn construct_secret(
        &self,
        user_id: &Uuid,
        credential_id: &Uuid,
    ) -> Result<[u8; 32], DomainError> {
        let mut hash = HmacSha256::new_from_slice(&self.root_secret).map_err(|error| {
            log::error!("Error creating HMAC_SHA256 from slice: {error:?}");
            DomainError::Hashing("Failed to construct HMAC_SHA256 from slice".into())
        })?;

        hash.update(user_id.as_bytes());
        hash.update(credential_id.as_bytes());

        Ok(hash.finalize().into_bytes().into())
    }
}

