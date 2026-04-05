use crate::ports::services::cache::{Cache, CacheError, CachedSession};
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
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
        self.client.get_multiplexed_async_connection().await.unwrap().set::<String, String, ()>(session.id.into(), session_str).await.unwrap();

        Ok(())
    }

    async fn load_session(&self) -> Result<CachedSession, CacheError> {
        todo!()
    }
}
