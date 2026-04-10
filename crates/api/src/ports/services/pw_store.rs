use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::errors::routes::DomainError;


pub struct CreateUser {
    pub id: Uuid,
    pub username: String,
    pub hashed_pw: String,
}

pub struct AuthCredentials {
    pub id: Uuid,
    pub username: String,
    pub pw_hash: String,
}

#[async_trait]
pub trait PWStore {
    async fn create() -> Self;
    async fn create_user(&self, user_info: CreateUser) -> Result<Uuid, DomainError>;
    async fn get_auth_credentials(&self, username: &str) -> Result<AuthCredentials, DomainError>;
}
