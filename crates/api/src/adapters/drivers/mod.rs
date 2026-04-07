use crate::adapters::drivers::portal::axum::AxumPortal;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

pub mod portal;

pub async fn create_portal<C, PW>(
    application: Application<C, PW>,
    bind_addr: Option<&str>,
) -> impl Portal<C, PW>
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    AxumPortal::new(application, bind_addr).await
}
