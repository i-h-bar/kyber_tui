use crate::api::add::NewCredential;
use crate::api::{ApiError, ApiSession};
use crate::ports::services::request::RequestClient;
use contracts::get_credential::{GetCredentialRequest, GetCredentialResponse};
use contracts::token::AuthToken;
use contracts::{GenericRequest, GenericResponse};
use pqcrypto::sym::Symmetric;
use pqcrypto::sym::aes::AesCipher;

impl ApiSession {
    pub async fn get_credential_from_service(
        &mut self,
        service: String,
    ) -> Result<NewCredential, ApiError> {
        let Some(token) = std::mem::take(&mut self.encrypted_auth_token) else {
            log::error!("Not authenticated");
            return Err(ApiError::Authenticate);
        };

        let Some(secret) = self.secret.as_ref() else {
            log::error!("Not authenticated");
            return Err(ApiError::Authenticate);
        };

        let cipher = AesCipher::new(secret);

        let request = GetCredentialRequest { service };
        let request = GenericRequest {
            session_id: self.session_id,
            token,
            body: self
                .key_pair
                .encrypt(&request, &self.server_public_key)
                .map_err(|error| {
                    log::error!("Error encrypting credentials {error:?}");
                    ApiError::Encryption
                })?,
        };

        let response: GenericResponse = self
            .client
            .post("http://localhost:3000/get", &request)
            .await?;

        if let Err(why) = self
            .key_pair
            .decrypt::<AuthToken>(&response.token, &self.server_public_key)
        {
            log::error!("Error decrypting auth token: {why}");
            return Err(ApiError::Authenticate);
        }

        let response: GetCredentialResponse = self
            .key_pair
            .decrypt(&response.body, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error decrypting New credential response: {error:?}");
                ApiError::Authenticate
            })?;

        let service = String::from_utf8(cipher.decrypt(&response.service).map_err(|error| {
            log::error!("Error decrypting service: {error:?}");
            ApiError::Decryption
        })?)
        .unwrap();
        let username = String::from_utf8(cipher.decrypt(&response.username).map_err(|error| {
            log::error!("Error decrypting username: {error:?}");
            ApiError::Decryption
        })?)
        .unwrap();
        let password = String::from_utf8(cipher.decrypt(&response.password).map_err(|error| {
            log::error!("Error decrypting password: {error:?}");
            ApiError::Decryption
        })?)
        .unwrap();
        let notes: Option<Vec<String>> = response.notes.map(|note| {
            note.iter()
                .filter_map(|note| Some(String::from_utf8(cipher.decrypt(note).ok()?).unwrap()))
                .collect()
        });

        Ok(NewCredential {
            service,
            username,
            password,
            notes,
        })
    }
}
