use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum PWStoreError {
    #[error("Failed to create user")]
    UserCreationError,
}

pub struct CreateUser {
    pub id: Uuid,
    pub username: String,
    pub hashed_pw: String,
}

#[async_trait]
pub trait PWStore {
    async fn create() -> Self;
    async fn create_user(&self, user_info: CreateUser) -> Result<Uuid, PWStoreError>;
}
