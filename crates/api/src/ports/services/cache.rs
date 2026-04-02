use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedSession {
    pub id: Uuid,
    pub client_public_key: String,
}

#[async_trait]
pub trait Cache {
    fn create() -> Self;
    async fn save_session(&self, session: &CachedSession);
    async fn load_session(&self) -> CachedSession;
}
