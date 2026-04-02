use crate::ports::services::cache::{Cache, CachedSession};
use async_trait::async_trait;
use redis::Client;
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
    async fn save_session(&self, session: &CachedSession) {
        todo!()
    }

    async fn load_session(&self) -> CachedSession {
        todo!()
    }
}
