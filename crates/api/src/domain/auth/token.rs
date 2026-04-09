use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::{Cache, CachedSession};
use crate::ports::services::pw_store::PWStore;
use contracts::token::PreAuthToken;
use kyber_crypto::keys::KeyPair;
use std::time::{SystemTime, UNIX_EPOCH};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub fn check_pre_auth_token(
        &self,
        token: &String,
        session: &CachedSession,
        key_pair: &KeyPair,
    ) -> Result<PreAuthToken, DomainError> {
        let token_bytes = key_pair
            .decrypt_with_secret_from_b64(token, &session.token_key, &key_pair.public)
            .map_err(|error| {
                log::info!("Unable to decrypt token: {:?}", error);
                DomainError::PermissionError("Invalid Token".to_string())
            })?;
        let token = PreAuthToken::from_bytes(&token_bytes);
        if token.session_id != session.id {
            log::warn!("Session id mismatch {} - {}", token.session_id, session.id);
            return Err(DomainError::PermissionError(
                "Session id mismatch".to_string(),
            ));
        }

        let session_expiry = session.expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();
        if token.expiry_s != session_expiry {
            log::warn!("Mismatched expiry {} - {}", token.expiry_s, session_expiry);
            return Err(DomainError::PermissionError("Mismatch Expiry".to_string()));
        }

        if SystemTime::now() > session.expiry {
            log::info!("Session expired {:?}", session.expiry);
            return Err(DomainError::PermissionError("Session expired".to_string()));
        }

        Ok(token)
    }
}
