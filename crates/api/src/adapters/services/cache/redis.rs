use crate::ports::services::cache::{Cache, CacheError, CachedSession};
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client, SetExpiry, SetOptions};
use std::env;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {}

#[async_trait]
impl Cache for RedisCache {
    fn create() -> Self {
        let url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        Self {
            client: Client::open(url).unwrap(),
        }
    }
    async fn save_session(&self, session: &CachedSession) -> Result<(), CacheError> {
        let session_str = serde_json::to_string(&session).map_err(|_| CacheError::SaveError)?;
        self.get_connection()
            .await?
            .set_options::<String, String, ()>(
                session.id.into(),
                session_str,
                SetOptions::default().with_expiration(SetExpiry::EX(60)),
            )
            .await
            .map_err(|_| CacheError::SaveError)?;

        Ok(())
    }

    async fn load_session(&self, session_id: String) -> Result<CachedSession, CacheError> {
        Ok(serde_json::from_str(
            &self
                .get_connection()
                .await?
                .get::<String, String>(session_id)
                .await
                .map_err(|_| CacheError::LoadError("Error fetching session".to_string()))?,
        )
        .map_err(|_| CacheError::LoadError("Error serialising session".to_string()))?)
    }
}

impl RedisCache {
    async fn get_connection(&self) -> Result<MultiplexedConnection, CacheError> {
        Ok(self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| CacheError::ConnectionError)?)
    }
}
