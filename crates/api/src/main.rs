mod adapters;
mod domain;
mod ports;

use crate::adapters::drivers::create_portal;
use crate::adapters::services::cache::create_cache;
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;

#[tokio::main]
async fn main() {
    let cache = create_cache();
    let application = Application::new(cache);

    let portal = create_portal(application, Some("0.0.0.0:3000"))
        .await
        .add_health_check_route()
        .await
        .add_exchange_route()
        .await;

    portal.run().await;
}
