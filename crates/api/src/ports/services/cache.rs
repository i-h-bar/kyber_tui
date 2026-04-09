use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
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

#[async_trait]
pub trait Cache {
    async fn create() -> Self;
    async fn save_session(&self, session: &Session) -> Result<(), DomainError>;
    async fn load_session(&self, session_id: &Uuid) -> Result<Session, DomainError>;
}
