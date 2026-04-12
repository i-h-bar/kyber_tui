use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::GenericRequest;
use contracts::token::PreAuthToken;
use pqcrypto::EncryptedMessage;
use std::time::{SystemTime, UNIX_EPOCH};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn verify_pre_auth_session_token(
        &self,
        request: &GenericRequest,
    ) -> Result<(Session, PreAuthToken), DomainError> {
        let session = self.cache.load_session(&request.session_id).await?;
        let token = check_pre_auth_token(&request.token, &session)?;

        Ok((session, token))
    }
}

fn check_pre_auth_token(
    token: &EncryptedMessage,
    session: &Session,
) -> Result<PreAuthToken, DomainError> {
    let token: PreAuthToken = session
        .server_key_pair
        .decrypt_with_secret(token, &session.token_key, &session.server_key_pair.public)
        .map_err(|error| {
            log::info!("Unable to decrypt token: {error:?}");
            DomainError::Decryption("Invalid Token".to_string())
        })?;
    if token.session_id != session.id {
        log::warn!("Session id mismatch {} - {}", token.session_id, session.id);
        return Err(DomainError::Permission("Session id mismatch".to_string()));
    }

    let session_expiry = session
        .expiry
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            log::error!("Session expiry time before UNIX_EPOCH - {error:?}");
            DomainError::Permission("Session expiry time before UNIX_EPOCH".to_string())
        })?
        .as_secs();
    if token.expiry_s != session_expiry {
        log::warn!("Mismatched expiry {} - {}", token.expiry_s, session_expiry);
        return Err(DomainError::Permission("Mismatch Expiry".to_string()));
    }

    if SystemTime::now() > session.expiry {
        log::info!("Session expired {:?}", session.expiry);
        return Err(DomainError::Permission("Session expired".to_string()));
    }

    Ok(token)
}
