use crate::domain::Application;
use crate::ports::services::cache::Cache;

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    #[allow(clippy::unused_async)]
    pub async fn health(&self) -> &str {
        "Ready"
    }
}
