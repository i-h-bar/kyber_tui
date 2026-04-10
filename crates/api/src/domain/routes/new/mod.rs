use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::{CreateUser, PWStore};
use contracts::new_user::{NewUserRequest, NewUserResponse};
use contracts::{GenericRequest, GenericResponse};
use uuid::Uuid;

use crate::domain::errors::routes::DomainError;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn create_user(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let session = self.cache.load_session(&request.session_id).await?;
        self.check_pre_auth_token(&request.token, &session)?;

        let new_user: NewUserRequest = session
            .server_key_pair
            .decrypt(&request.body, &session.client_public_key)
            .map_err(|err| {
                log::warn!("Error decrypting message {}", err);
                DomainError::DecryptionError("Error decrypting message".to_string())
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
                log::warn!("User creation failed: {:?}", error);
                DomainError::GenericError("User creation failed".to_string())
            })?;

        let response = NewUserResponse { success: true };

        Ok(GenericResponse {
            session_id: session.id,
            body: session
                .server_key_pair
                .encrypt(&response, &session.client_public_key)
                .map_err(|error| {
                    log::error!("Error encrypting new user body {}", error);
                    DomainError::EncryptionError("Error encrypting new user body".to_string())
                })?,
            token: request.token,
        })
    }
}
