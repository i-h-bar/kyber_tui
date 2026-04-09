use std::time::Instant;
use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::{CreateUser, PWStore};
use contracts::new_user::{NewUserRequest, NewUserResponse};
use contracts::{GenericRequest, GenericResponse};
use kyber_crypto::keys::KeyPair;
use kyber_crypto::keys::public::Public;
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
        let session = self.load_session_info(request.session_id).await?;
        self.check_pre_auth_token(&request.token, &session)?;

        let message = session
            .server_key_pair
            .decrypt_b64(&request.body, &session.client_public_key)
            .map_err(|err| {
                log::warn!("Error decrypting message {}", err);
                DomainError::DecryptionError("Error decrypting message".to_string())
            })?;
        let new_user: NewUserRequest = serde_json::from_str(&message).map_err(|error| {
            log::warn!("Error deserialising NewUserRequest {}", error);
            DomainError::DeserialisationError("Error deserialising NewUserRequest".to_string())
        })?;
        let user_id = Uuid::new_v4();

        let start = Instant::now();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_pw = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .map_err(|err| {
                log::warn!("Error hashing password {}", err);
                DomainError::GenericError("Error hashing password".to_string())
            })?
            .to_string();
        log::info!("Hashing password took {:?}ms", start.elapsed().as_millis());

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
        let body = serde_json::to_string(&response).map_err(|error| {
            log::warn!("Error serializing NewUserResponse {}", error);
            DomainError::SerialisationError("Error serializing NewUserResponse".to_string())
        })?;

        Ok(GenericResponse {
            session_id: session.id,
            body: session
                .server_key_pair
                .encrypt_to_b64(&body,&session.client_public_key)
                .map_err(| error | {
                    log::error!("Error encrypting new user body {}", error);
                    DomainError::EncryptionError("Error encrypting new user body".to_string())
                })?,
            token: request.token,
        })
    }
}
