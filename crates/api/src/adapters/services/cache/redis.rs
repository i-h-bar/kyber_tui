use async_trait::async_trait;
use redis::Client;
use crate::ports::services::cache::{Cache, CachedSession};

struct RedisCache {
    client: Client,
}

impl RedisCache {
    fn init() -> Self {
        Self {
            client: Client::open("redis://127.0.0.1/").unwrap(),
        }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn save_session(self, session: CachedSession) {
        todo!()
    }

    async fn load_session(self) -> CachedSession {
        todo!()
    }
}