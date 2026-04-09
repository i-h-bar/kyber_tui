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
        let session = self.cache.load_session(&request.session_id).await?;
        let key_pair = KeyPair::from_b64(&session.server_key_pair).map_err(|err| {
            log::error!("Error deserialising key pair {}", err);
            DomainError::DeserialisationError("Error deserialising key pair".to_string())
        })?;
        self.check_pre_auth_token(&request.token, &session, &key_pair)?;

        let client_public = Public::from_b64(&session.client_public_key).map_err(|err| {
            log::error!("Error deserialising client public key {}", err);
            DomainError::DeserialisationError("Error deserialising client public key".to_string())
        })?;
        let message = key_pair
            .decrypt_b64(&request.body, &client_public)
            .map_err(|err| {
                log::warn!("Error decrypting message {}", err);
                DomainError::EncryptionError("Error decrypting message".to_string())
            })?;
        let new_user: NewUserRequest = serde_json::from_str(&message).map_err(|error| {
            log::warn!("Error deserialising NewUserRequest {}", error);
            DomainError::DeserialisationError("Error deserialising NewUserRequest".to_string())
        })?;
        let user_id = Uuid::new_v4();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_pw = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .map_err(|err| {
                log::warn!("Error hashing password {}", err);
                DomainError::GenericError("Error hashing password".to_string())
            })?
            .to_string();

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
            body: key_pair
                .encrypt_b64(&serde_json::to_string(&response).unwrap(), &client_public)
                .unwrap(),
            token: request.token,
        })
    }
}
