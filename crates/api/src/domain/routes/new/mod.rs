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
        let key_pair = KeyPair::from_b64(&session.server_key_pair).unwrap();
        self.check_pre_auth_token(&request.token, &key_pair)?;

        let client_public = Public::from_b64(&session.client_public_key).unwrap();
        let message = key_pair.decrypt_b64(&request.body, &client_public).unwrap();
        let new_user: NewUserRequest = serde_json::from_str(&message).unwrap();
        let user_id = Uuid::new_v4();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let create_user = CreateUser {
            id: user_id,
            username: new_user.username,
            hashed_pw: argon2
                .hash_password(new_user.password.as_bytes(), &salt)
                .unwrap()
                .to_string(),
        };
        let _ = self.pw_store.create_user(create_user).await.unwrap();

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
