use async_trait::async_trait;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedSession {
    pub session_id: Uuid,
    pub client_public_key: String,
}

#[async_trait]
pub trait Cache {
    async fn save_session(self, session: CachedSession);
    async fn load_session(self) -> CachedSession;
}