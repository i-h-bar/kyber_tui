use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Error Saving the Session")]
    SaveError,
    
    #[error("Error Retrieving the Session")]
    LoadError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedSession {
    pub id: Uuid,
    pub client_public_key: String,
}

#[async_trait]
pub trait Cache {
    fn create() -> Self;
    async fn save_session(&self, session: &CachedSession) -> Result<(), CacheError>;
    async fn load_session(&self) -> Result<CachedSession, CacheError>;
}
