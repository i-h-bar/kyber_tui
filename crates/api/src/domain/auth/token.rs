use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::token::PreAuthToken;
use kyber_crypto::keys::traits::TryFromBytes;
use kyber_crypto::keys::{EncryptedMessage, KeyPair};
use std::time::{SystemTime, UNIX_EPOCH};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub fn check_pre_auth_token(
        &self,
        token: &EncryptedMessage,
        session: &Session,
    ) -> Result<PreAuthToken, DomainError> {
        let token: PreAuthToken = session
            .server_key_pair
            .decrypt_with_secret(token, &session.token_key, &session.server_key_pair.public)
            .map_err(|error| {
                log::info!("Unable to decrypt token: {:?}", error);
                DomainError::DecryptionError("Invalid Token".to_string())
            })?;
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
