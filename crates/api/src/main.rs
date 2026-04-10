mod adapters;
mod domain;
mod ports;

use crate::adapters::drivers::create_portal;
use crate::adapters::services::cache::create_cache;
use crate::adapters::services::pw_store::create_pw_store;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cache = create_cache().await;
    let pw_store = create_pw_store().await;
    let application = Application::new(cache, pw_store);

    let portal = create_portal(application, Some("0.0.0.0:3000"))
        .await
        .add_health_check_route()
        .add_handshake_route()
        .add_new_user_route()
        .add_authentication_route();

    log::info!("Initialised");

    portal.run().await;
}
