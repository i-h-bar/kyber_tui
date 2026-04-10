use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
use contracts::token::PreAuthToken;
use pqcrypto::EncryptedMessage;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn check_pre_auth_token(
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

    let session_expiry = session.expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();
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
