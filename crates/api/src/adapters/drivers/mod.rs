use crate::adapters::drivers::portal::axum::AxumPortal;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;

pub mod portal;

pub async fn create_portal<C>(
    application: Application<C>,
    bind_addr: Option<&str>,
) -> impl Portal<C>
where
    C: Cache + Send + Sync + 'static,
{
    AxumPortal::new(application, bind_addr).await
}
