use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PWStoreError {
    #[error("Failed to create user")]
    UserCreationError,
}


#[async_trait]
pub trait PWStore {
    async fn create() -> Self;
    async fn create_user(&self) -> Result<(), PWStoreError>;
}