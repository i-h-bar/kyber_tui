use crate::ports::services::cache::{Cache, CachedSession};
use async_trait::async_trait;
use redis::Client;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {}

#[async_trait]
impl Cache for RedisCache {
    fn create() -> Self {
        Self {
            client: Client::open("redis://127.0.0.1/").unwrap(),
        }
    }
    async fn save_session(&self, session: &CachedSession) {
        todo!()
    }

    async fn load_session(&self) -> CachedSession {
        todo!()
    }
}
