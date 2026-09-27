use crate::adapters::drivers::portal::axum::AxumPortal;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

pub mod portal;

pub async fn create_portal(
    bind_addr: Option<&str>,
) -> impl Portal
{
    AxumPortal::new(bind_addr).await
}
