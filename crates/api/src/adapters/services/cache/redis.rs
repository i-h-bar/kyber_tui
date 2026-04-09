use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client, RedisError, SetExpiry, SetOptions};
use sqlx::postgres::PgTypeKind::Domain;
use std::env;
use uuid::Uuid;
use crate::domain::session::Session;

pub struct RedisCache {
    connection: MultiplexedConnection,
}

impl RedisCache {}

#[async_trait]
impl Cache for RedisCache {
    async fn create() -> Self {
        let url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let client = Client::open(url).expect("Failed to open redis client");
        let connection = client
            .get_multiplexed_async_connection()
            .await
            .expect("Failed to connect to Redis");
        Self { connection }
    }
    async fn save_session(&self, session: &Session) -> Result<(), DomainError> {
        let session_str = serde_json::to_string(&session).map_err(|error| {
            log::warn!("Session serialisation error: {}", error);
            DomainError::SerialisationError("Failed to serialise session".to_string())
        })?;
        self.get_connection()
            .await
            .set_options::<String, String, ()>(
                session.id.to_string(),
                session_str,
                SetOptions::default().with_expiration(SetExpiry::EX(60)),
            )
            .await
            .map_err(|error| {
                log::warn!("Session save error: {}", error);
                DomainError::SessionError("Failed to save session".to_string())
            })?;

        Ok(())
    }

    async fn load_session(&self, session_id: &Uuid) -> Result<Session, DomainError> {
        Ok(serde_json::from_str(
            &self
                .get_connection()
                .await
                .get::<String, String>((*session_id).into())
                .await
                .map_err(|error| {
                    log::warn!("Session load error: {}", error);
                    DomainError::PermissionError("Failed to load session".to_string())
                })?,
        )
        .map_err(|error| {
            log::warn!("Session load deserialisation error: {}", error);
            DomainError::DeserialisationError("Failed to deserialise session".to_string())
        })?)
    }
}

impl RedisCache {
    async fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}
