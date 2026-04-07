use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Error Creating Connection")]
    ConnectionError,

    #[error("Error Saving the Session")]
    SaveError,

    #[error("{0}")]
    LoadError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedSession {
    pub id: Uuid,
    pub client_public_key: String,
    pub server_key_pair: String,
    pub expiry: SystemTime,
    pub user_id: Option<Uuid>,
}

#[async_trait]
pub trait Cache {
    fn create() -> Self;
    async fn save_session(&self, session: &CachedSession) -> Result<(), CacheError>;
    async fn load_session(&self, session_id: &Uuid) -> Result<CachedSession, CacheError>;
}
