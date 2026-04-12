use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::{CreateUser, PWStore};
use contracts::new_user::{NewUserRequest, NewUserResponse};
use contracts::{GenericRequest, GenericResponse};
use uuid::Uuid;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn create_user(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let (session, _) = self.verify_pre_auth_session_token(&request).await?;

        let new_user: NewUserRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::warn!("Error decrypting message {error}");
                DomainError::Decryption("Error decrypting message".to_string())
            })?;

        let user_id = Uuid::new_v4();
        let hashed_pw = self.create_pw_hash(new_user.password.into_bytes()).await?;

        let create_user = CreateUser {
            id: user_id,
            username: new_user.username,
            hashed_pw,
        };
        let _ = self
            .pw_store
            .create_user(create_user)
            .await
            .map_err(|error| {
                log::warn!("User creation failed: {error:?}");
                DomainError::Generic("User creation failed".to_string())
            })?;

        let response = NewUserResponse { success: true };

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
