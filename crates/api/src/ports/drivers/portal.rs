use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use async_trait::async_trait;

#[async_trait]
pub trait Portal<C, PW>
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    async fn new(bind_addr: Option<&str>, application: Application<C, PW>) -> Self;
    fn add_health_check_route(self) -> Self;
    fn add_handshake_route(self) -> Self;
    fn add_new_user_route(self) -> Self;
    fn add_authentication_route(self) -> Self;
    fn add_new_credential_route(self) -> Self;
    fn add_get_credential_route(self) -> Self;
    async fn run(self);
}
