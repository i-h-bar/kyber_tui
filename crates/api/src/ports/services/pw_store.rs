use crate::domain::errors::routes::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

pub struct CreateUser {
    pub id: Uuid,
    pub username: String,
    pub hashed_pw: String,
}

pub struct AuthCredentials {
    pub id: Uuid,
    pub pw_hash: String,
}

pub struct Note {
    pub id: Uuid,
    pub content: Vec<u8>,
}

pub struct CredentialIn {
    pub id: Uuid,
    pub service: Vec<u8>,
    pub username: Vec<u8>,
    pub password: Vec<u8>,
    pub service_index: [u8; 32],
    pub user_id: Uuid,
    pub notes: Option<Vec<Note>>,
}

pub struct CredentialOut {
    pub id: Uuid,
    pub service: Vec<u8>,
    pub username: Vec<u8>,
    pub password: Vec<u8>,
    pub notes: Option<Vec<Vec<u8>>>,
}

#[async_trait]
pub trait PWStore {
    async fn create() -> Self;
    async fn create_user(&self, user_info: CreateUser) -> Result<Uuid, DomainError>;
    async fn get_auth_credentials(&self, username: &str) -> Result<AuthCredentials, DomainError>;
    async fn upsert_credential(&self, credential: &CredentialIn) -> Result<(), DomainError>;
    async fn get_credential(
        &self,
        service_index: &[u8; 32],
        user_id: &Uuid,
    ) -> Result<CredentialOut, DomainError>;
}
