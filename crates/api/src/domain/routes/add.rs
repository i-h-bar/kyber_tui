use crate::domain::Application;
use crate::domain::encryption::encrypt_field;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::{Credential, Note, PWStore};
use aes_gcm::{Aes256Gcm, KeyInit};
use contracts::new_credential::{NewCredentialRequest, NewCredentialResponse};
use contracts::{GenericRequest, GenericResponse};
use uuid::Uuid;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn add_credential(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        let (session, token) = self.verify_auth_token(&request).await?;

        let new_credential: NewCredentialRequest = request
            .get_message(&session.server_key_pair, &session.client_public_key)
            .map_err(|error| {
                log::info!("Failed to decrypt new user credential: {error:?}");
                DomainError::Decryption("Failed to decrypt credential".into())
            })?;

        let credential_id = Uuid::new_v4();
        let secret = self.construct_secret(&token.user_id, &credential_id)?;
        let aes = Aes256Gcm::new_from_slice(&secret).unwrap();
        let service = encrypt_field(&aes, &new_credential.service)?;
        let username = encrypt_field(&aes, &new_credential.username)?;
        let password = encrypt_field(&aes, &new_credential.password)?;

        let notes: Option<Vec<Note>> = new_credential.notes.map(|notes| {
            notes
                .iter()
                .filter_map(|note| {
                    Some(Note {
                        id: Uuid::new_v4(),
                        content: encrypt_field(&aes, note).ok()?,
                    })
                })
                .collect()
        });

        let payload = Credential {
            id: credential_id,
            user_id: token.user_id,
            service,
            username,
            password,
            notes,
        };

        self.pw_store.upsert_credential(&payload).await?;

        let response = NewCredentialResponse { added: 1 };

        let response = GenericResponse {
            body: session
                .server_key_pair
                .encrypt(&response, &session.client_public_key)
                .unwrap(),
            token: request.token,
        };

        Ok(response)
    }
}
