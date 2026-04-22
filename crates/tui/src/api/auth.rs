use crate::api::{ApiError, ApiSession};
use crate::ports::services::request::RequestClient;
use crate::utils::hashing;
use contracts::auth::{AuthRequest, AuthResponse};
use contracts::token::AuthToken;
use contracts::{GenericRequest, GenericResponse};

impl ApiSession {
    pub async fn authenticate(
        &mut self,
        username: String,
        password: String,
    ) -> Result<bool, ApiError> {
        let Some(token) = std::mem::take(&mut self.encrypted_pre_auth_token) else {
            log::error!("Not able to take pre_auth_token from ApiSession");
            return Err(ApiError::Authenticate);
        };

        let auth_request = AuthRequest { username, password };

        let body = self
            .key_pair
            .encrypt(&auth_request, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error encrypting auth request: {error:?}");
                ApiError::Authenticate
            })?;

        let request = GenericRequest {
            session_id: self.session_id,
            body,
            token,
        };

        let response: GenericResponse = self
            .client
            .post("http://localhost:3000/auth", &request)
            .await?;

        let auth_token: AuthToken = response
            .get_token(&self.key_pair, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error decrypting token: {error:?}");
                ApiError::Authenticate
            })?;

        if auth_token.session_id != self.session_id {
            log::error!("Session id mismatch");
            return Err(ApiError::Authenticate);
        }

        let auth_response: AuthResponse = response
            .get_message(&self.key_pair, &self.server_public_key)
            .map_err(|error| {
                log::error!("Error decrypting auth response: {error:?}");
                ApiError::Authenticate
            })?;

        self.auth_token = Some(auth_token);
        self.encrypted_auth_token = Some(response.token);

        let Some(hash) = hashing::hash(&auth_request.password) else {
            return Err(ApiError::Authenticate);
        };
        self.secret = Some(hash);

        Ok(auth_response.success)
    }
}
