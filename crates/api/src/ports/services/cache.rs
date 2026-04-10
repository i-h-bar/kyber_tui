use crate::domain::errors::routes::DomainError;
use crate::domain::session::Session;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Cache {
    async fn create() -> Self;
    async fn save_session(&self, session: &Session) -> Result<(), DomainError>;
    async fn load_session(&self, session_id: &Uuid) -> Result<Session, DomainError>;
}
