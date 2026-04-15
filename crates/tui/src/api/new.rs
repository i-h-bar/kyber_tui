use crate::api::{ApiError, ApiSession};
use crate::ports::services::request::RequestClient;
use contracts::new_user::{NewUserRequest, NewUserResponse};
use contracts::{GenericRequest, GenericResponse};

impl ApiSession {
    pub async fn create_new_user(
        &self,
        username: String,
        password: String,
    ) -> Result<bool, ApiError> {
        let request = NewUserRequest { username, password };
        let Some(token) = self.encrypted_pre_auth_token.as_ref() else {
            return Ok(false);
        };

        let request = GenericRequest {
            session_id: self.session_id,
            body: self
                .key_pair
                .encrypt(&request, &self.server_public_key)
                .map_err(|error| {
                    log::error!("Error encrypting create user request {error}");
                    ApiError::Decryption
                })?,
            token: token.clone(),
        };

        let response: GenericResponse = self
            .client
            .post("http://localhost:3000/new", &request)
            .await?;

        let response: NewUserResponse = response
            .get_message(&self.key_pair, &self.server_public_key)
            .unwrap();

        Ok(response.success)
    }
}
