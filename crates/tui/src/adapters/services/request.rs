use crate::api::ApiError;
use crate::ports::services::request::RequestClient;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Client {
    client: reqwest::Client,
}

#[async_trait]
impl RequestClient for Client {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn post<T: Serialize + ?Sized + Send + Sync, R: DeserializeOwned + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<R, ApiError> {
        Ok(self
            .client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|error| {
                log::error!("Error sending request {error}");
                ApiError::Send
            })?
            .error_for_status()
            .map_err(|error| {
                let status_code = error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                log::error!("Error creating response: {error} - {status_code}");
                ApiError::ApiResponse(format!("Error from the API - {status_code}"))
            })?
            .json()
            .await
            .map_err(|error| {
                log::error!("Error deserialising response {error}");
                ApiError::Deserialisation
            })?)
    }
}
