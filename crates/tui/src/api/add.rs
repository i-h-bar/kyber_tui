use crate::api::{ApiError, ApiSession};
use crate::ports::services::request::RequestClient;
use contracts::new_credential::{NewCredentialRequest, NewCredentialResponse};
use contracts::token::AuthToken;
use contracts::{GenericRequest, GenericResponse};
use pqcrypto::sym::Symmetric;
use pqcrypto::sym::aes::AesCipher;

pub struct NewCredential {
    pub service: String,
    pub username: String,
    pub password: String,
    pub notes: Option<Vec<String>>,
}

impl ApiSession {
    pub async fn add_credential(&mut self, credential: NewCredential) -> Result<u16, ApiError> {
        let Some(token) = std::mem::take(&mut self.encrypted_auth_token) else {
            log::error!("Not authenticated");
            return Err(ApiError::Authenticate);
        };

        let Some(secret) = self.secret.as_ref() else {
            log::error!("Not authenticated");
            return Err(ApiError::Authenticate);
        };
        let cipher = AesCipher::new(secret);
        let service = cipher
            .encrypt(credential.service.as_bytes())
            .map_err(|error| {
                log::error!("Error encrypting service {error:?}");
                ApiError::Encryption
            })?;
        let username = cipher
            .encrypt(credential.username.as_bytes())
            .map_err(|error| {
                log::error!("Error encrypting username {error:?}");
                ApiError::Encryption
            })?;
        let password = cipher
            .encrypt(credential.password.as_bytes())
            .map_err(|error| {
                log::error!("Error encrypting password {error:?}");
                ApiError::Encryption
            })?;
        let notes: Option<Vec<Vec<u8>>> = if let Some(notes) = credential.notes {
            Some(
                notes
                    .iter()
                    .map(|note| -> Result<Vec<u8>, ApiError> {
                        cipher.encrypt(note.as_bytes()).map_err(|error| {
                            log::error!("Error encrypting note {error:?}");
                            ApiError::Encryption
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )
        } else {
            None
        };

        let credential = NewCredentialRequest {
            service,
            username,
            password,
            notes,
            service_name: credential.service,
        };

        let request = GenericRequest {
            session_id: self.session_id,
            token,
            body: self
                .key_pair
                .encrypt(&credential, &self.server_public_key)
                .map_err(|error| {
                    log::error!("Error encrypting credentials {error:?}");
                    ApiError::Encryption
                })?,
        };

        let response: GenericResponse = self
            .client
            .post("http://localhost:3000/add", &request)
            .await?;

        if let Err(why) = self
            .key_pair
            .decrypt::<AuthToken>(&response.token, &self.server_public_key)
        {
            log::error!("Error decrypting auth token: {why}");
            return Err(ApiError::Authenticate);
        }

        let response: NewCredentialResponse = self
            .key_pair
            .decrypt(&response.body, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error decrypting New credential response: {error:?}");
                ApiError::Authenticate
            })?;

        Ok(response.added)
    }
}
