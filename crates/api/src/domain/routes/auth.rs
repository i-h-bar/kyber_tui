use contracts::{GenericRequest, GenericResponse};
use contracts::auth::AuthRequest;
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
        let session = self.cache.load_session(&request.session_id).await?;
        auth::token::check_pre_auth_token(&request.token, &session)?;
        
        let body: AuthRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error decrypting message {error}");
                DomainError::Decryption("Error decrypting message".to_string())
            })?;
        
        todo!()
    }
}