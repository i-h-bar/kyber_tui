use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use contracts::get_credential::{GetCredentialRequest, GetCredentialResponse};
use contracts::{GenericRequest, GenericResponse};
use pqcrypto::sym::Symmetric;
use pqcrypto::sym::aes::AesCipher;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn get_credential(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let (session, _) = self.verify_auth_token(&request).await?;
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

        let service_index = self.construct_hash(user_id, &creds_request.service)?;

        let credential = self
            .pw_store
            .get_credential(&service_index, user_id)
            .await?;
        let secret = self.construct_secret(user_id, &credential.id)?;
        let cipher = AesCipher::new(&secret);

        let service = cipher.decrypt(&credential.service).map_err(|error| {
            log::warn!("Error decrypting message {error}");
            DomainError::Decryption("Error decrypting message".to_string())
        })?;

        let username = cipher.decrypt(&credential.username).map_err(|error| {
            log::warn!("Error decrypting message {error}");
            DomainError::Decryption("Error decrypting message".to_string())
        })?;

        let password = cipher.decrypt(&credential.password).map_err(|error| {
            log::warn!("Error decrypting message {error}");
            DomainError::Decryption("Error decrypting message".to_string())
        })?;

        let notes: Option<Vec<Vec<u8>>> = credential.notes.map(|notes| {
            notes
                .iter()
                .filter_map(|note| cipher.decrypt(note).ok())
                .collect()
        });

        let response = GetCredentialResponse {
            service,
            username,
            password,
            notes,
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
