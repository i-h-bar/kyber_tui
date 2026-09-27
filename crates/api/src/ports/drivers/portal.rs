use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use async_trait::async_trait;

#[async_trait]
pub trait Portal<'b>
{
    async fn new(bind_addr: Option<&str>) -> Self;
    fn add_health_check_route<'a: 'b, C, PW>(self, application: &'a Application<C, PW>) -> Self 
    where
        C: Cache + Send + Sync,
        PW: PWStore + Send + Sync ;
    fn add_handshake_route(self) -> Self;
    fn add_new_user_route(self) -> Self;
    fn add_authentication_route(self) -> Self;
    fn add_new_credential_route(self) -> Self;
    fn add_get_credential_route(self) -> Self;
    async fn run(self);
}
