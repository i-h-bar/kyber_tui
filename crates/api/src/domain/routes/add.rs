use pqcrypto::sym::Symmetric;
use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::{Credential, Note, PWStore};
use aes_gcm::{Aes256Gcm, KeyInit};
use contracts::new_credential::{NewCredentialRequest, NewCredentialResponse};
use contracts::{GenericRequest, GenericResponse};
use uuid::Uuid;
use pqcrypto::sym::aes::AesCipher;

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
        let cipher = AesCipher::new(&secret);
        let service = cipher.encrypt(&new_credential.service).map_err(| error | {
            log::info!("Failed to encrypt service new user credential: {error:?}");
            DomainError::Encryption("Failed to encrypt new user credential".into())
        })?;
        let username = cipher.encrypt(&new_credential.username).map_err(| error | {
            log::info!("Failed to encrypt username new user credential: {error:?}");
            DomainError::Encryption("Failed to encrypt new user credential".into())
        })?;
        let password = cipher.encrypt(&new_credential.password).map_err(| error | {
            log::info!("Failed to encrypt new password user credential: {error:?}");
            DomainError::Encryption("Failed to encrypt new user credential".into())
        })?;
        let service_index = self.construct_hash(
            &token.user_id, &credential_id, &new_credential.service_name
        )?;

        let notes: Option<Vec<Note>> = new_credential.notes.map(|notes| {
            notes
                .iter()
                .filter_map(|note| {
                    let content = cipher.encrypt(note).ok()?;

                    Some(Note {
                        id: Uuid::new_v4(),
                        content,
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
            service_index,
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
