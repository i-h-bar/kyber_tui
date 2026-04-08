use crate::adapters::services::cache::redis::RedisCache;
use crate::ports::services::cache::Cache;

pub mod redis;

pub async fn create_cache() -> impl Cache {
    RedisCache::create().await
}
