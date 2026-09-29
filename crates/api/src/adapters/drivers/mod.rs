use crate::adapters::drivers::portal::axum::AxumPortal;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

pub mod portal;

pub async fn create_portal<C, PW>(
    bind_addr: Option<&str>,
    application: Application<C, PW>
) -> impl Portal<C, PW>
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    AxumPortal::new(bind_addr, application).await
}
