use contracts::{GenericRequest, GenericResponse};
use contracts::new_credential::{NewCredentialRequest, NewCredentialResponse};
use contracts::token::AuthToken;
use crate::api::{ApiError, ApiSession};


struct NewCredential {
    service: String,
    username: String,
    password: String,
    notes: Option<Vec<String>>,
}

impl ApiSession {
    pub async fn add_credential(&mut self, credential: NewCredential) -> Result<u16, ApiError> {
        let Some(token) = std::mem::take(&mut self.encrypted_auth_token) else {
            log::error!("Not authenticated");
            return Err(ApiError::Authenticate)
        };
        
        let credential = NewCredentialRequest {
            service: vec![],
            username: vec![],
            password: vec![],
            notes: None,
        };
        
        let request = GenericRequest {
            session_id: self.session_id,
            token,
            body: self.key_pair.encrypt(&credential, &self.server_public_key).unwrap(),
        };
        
        let response: GenericResponse = self.client.post("http://localhost:3000/add")
            .json(&request)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        
        if let Err(why) = self.key_pair.decrypt::<AuthToken>(&response.token, &self.server_public_key) {
            log::error!("Error decrypting auth token: {why}");
            return Err(ApiError::Authenticate)
        }
        
        let response: NewCredentialResponse = self.key_pair.decrypt(&response.body, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error decrypting New credential response: {error:?}");
                ApiError::Authenticate
            })?;
        
        Ok(response.added)
    }
}