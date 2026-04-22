use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::get_credential::{GetCredentialRequest, GetCredentialResponse};
use contracts::{GenericRequest, GenericResponse};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn get_credential(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let (session, _) = self.verify_pre_auth_session_token(&request).await?;
        let Some(user_id) = session.user_id.as_ref() else {
            log::warn!("No user id in session");
            return Err(DomainError::Session("No user id in session".to_string()));
        };

        let creds_request: GetCredentialRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error decrypting message {error}");
                DomainError::Decryption("Error decrypting message".to_string())
            })?;

        let credential = self
            .pw_store
            .get_credential(&creds_request.service_index, user_id)
            .await?;

        let response = GetCredentialResponse {
            id: credential.id,
            service: credential.service,
            username: credential.username,
            password: credential.password,
            notes: credential.notes,
        };

        Ok(GenericResponse {
            body: session
                .server_key_pair
                .encrypt(&response, &session.client_public_key)
                .map_err(|error| {
                    log::error!("Error encrypting new user body {error:?}");
                    DomainError::Encryption("Error encrypting new user body".to_string())
                })?,
            token: request.token,
        })
    }
}
