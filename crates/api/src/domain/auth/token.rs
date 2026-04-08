use std::time::SystemTime;
use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::{Cache, CachedSession};
use crate::ports::services::pw_store::PWStore;
use contracts::token::PreAuthToken;
use kyber_crypto::keys::KeyPair;
use kyber_crypto::message::EncryptedMessage;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub fn check_pre_auth_token(
        &self,
        token: &String,
        key_pair: &KeyPair,
    ) -> Result<(), DomainError> {
        if !key_pair.verify_b64(token) {
            return Err(DomainError::PermissionError("Permission denied".to_string()));
        }

        Ok(())
    }
}
