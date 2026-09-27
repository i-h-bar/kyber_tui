mod adapters;
mod domain;
mod ports;

use std::cell::LazyCell;
use std::sync::OnceLock;
use crate::adapters::drivers::create_portal;
use crate::adapters::services::cache::create_cache;
use crate::adapters::services::pw_store::create_pw_store;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use dotenv::dotenv;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cache = create_cache().await;
    let pw_store = create_pw_store().await;
    let application = Application::new(cache, pw_store);

    let url = std::env::var("URL").ok();

    {
        let portal = create_portal(url.as_deref())
            .await
            .add_health_check_route(&application)
            .add_handshake_route()
            .add_new_user_route()
            .add_authentication_route()
            .add_new_credential_route()
            .add_get_credential_route();

        log::info!("Initialised");

        portal.run().await;
    }
}
