use crate::domain::Application;
use crate::ports::services::cache::Cache;
use async_trait::async_trait;

#[async_trait]
pub trait Portal<C>
where
    C: Cache + Send + Sync,
{
    async fn new(application: Application<C>, bind_addr: Option<&str>) -> Self;
    fn add_health_check_route(self) -> Self;
    fn add_handshake_route(self) -> Self;
    fn add_new_user_route(self) -> Self;
    async fn run(self);
}
