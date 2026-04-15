use crate::api::ApiError;
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait RequestClient {
    fn new() -> Self;

    async fn post<T: Serialize + ?Sized + Send + Sync, R: DeserializeOwned + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<R, ApiError>;
}
