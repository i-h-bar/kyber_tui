use argon2::{Argon2, PasswordHash, PasswordVerifier};
use contracts::{GenericRequest, GenericResponse};
use contracts::auth::{AuthRequest, AuthResponse};
use contracts::token::AuthToken;
use crate::domain::{Application, auth};
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn authenticate(&self, request: GenericRequest) -> Result<GenericResponse, DomainError> {
        let mut session = self.cache.load_session(&request.session_id).await?;
        let token = auth::token::check_pre_auth_token(&request.token, &session)?;

        let body: AuthRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error decrypting message {error}");
                DomainError::Decryption("Error decrypting message".to_string())
            })?;

        let auth_credentials = self.pw_store.get_auth_credentials(&body.username).await?;
        if !self.verify_pw_hash(body.password.clone(), auth_credentials.pw_hash.clone()).await? {
            log::info!("Authentication credentials not verified");
            return Err(DomainError::Permission("Invalid password".to_string()));
        }

        session.user_id = Some(auth_credentials.id);

        self.cache.save_session(&session).await?;

        let auth_return = AuthResponse { success: true };
        let token = AuthToken {
            session_id: token.session_id,
            expiry_s: token.expiry_s,
            user_id: auth_credentials.id,
        };

        Ok(
            GenericResponse {
                body: session.server_key_pair.encrypt(&auth_return, &session.client_public_key).unwrap(),
                token: session.server_key_pair.encrypt(&token, &session.client_public_key).unwrap()
            }
        )
    }
}