use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    #[allow(clippy::unused_async)]
    pub async fn health(&self) -> &str {
        "Ready"
    }
}
