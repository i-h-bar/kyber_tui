use crate::domain::errors::routes::DomainError;
use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::auth::{AuthRequest, AuthResponse};
use contracts::token::AuthToken;
use contracts::{GenericRequest, GenericResponse};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn authenticate(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let (mut session, token) = self.verify_pre_auth_session_token(&request).await?;

        let body: AuthRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error decrypting message {error}");
                DomainError::Decryption("Error decrypting message".to_string())
            })?;

        let auth_credentials = self.pw_store.get_auth_credentials(&body.username).await?;
        if !self
            .verify_pw_hash(body.password.clone(), auth_credentials.pw_hash.clone())
            .await?
        {
            log::info!("Authentication credentials not verified");
            return Err(DomainError::Permission("Invalid password".to_string()));
        }

        let auth_return = AuthResponse { success: true };
        let token = AuthToken {
            session_id: token.session_id,
            expiry_s: token.expiry_s,
            user_id: auth_credentials.id,
        };

        let body = session
            .server_key_pair
            .encrypt(&auth_return, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error encrypting message {error}");
                DomainError::Encryption("Error encrypting message".to_string())
            })?;

        let token = session
            .server_key_pair
            .encrypt(&token, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error encrypting message {error}");
                DomainError::Encryption("Error encrypting message".to_string())
            })?;

        session.user_id = Some(auth_credentials.id);
        self.cache.save_session(&session).await?;

        Ok(GenericResponse { body, token })
    }
}
