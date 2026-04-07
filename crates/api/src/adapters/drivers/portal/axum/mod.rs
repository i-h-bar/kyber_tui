use crate::adapters::drivers::portal::axum::routes::{handshake, new};
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use async_trait::async_trait;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod routes;

pub struct AxumPortal<C, PW>
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    application: Arc<Application<C, PW>>,
    router: Router,
    listener: TcpListener,
}

async fn health<C, PW>(app: Arc<Application<C, PW>>)
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    app.health().await;
}

#[async_trait]
impl<C, PW> Portal<C, PW> for AxumPortal<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    async fn new(application: Application<C, PW>, bind_addr: Option<&str>) -> Self {
        Self {
            application: Arc::new(application),
            router: Router::new(),
            listener: TcpListener::bind(bind_addr.unwrap_or("0.0.0.0:3000"))
                .await
                .unwrap(),
        }
    }

    fn add_health_check_route(mut self) -> Self {
        self.router = self.router.route(
            "/ready",
            get({
                let app = Arc::clone(&self.application);
                move || health(app)
            }),
        );

        self
    }

    fn add_handshake_route(mut self) -> Self {
        self.router = self.router.route(
            "/handshake",
            post({
                let app = Arc::clone(&self.application);
                move |payload| handshake::run(app, payload)
            }),
        );

        self
    }

    fn add_new_user_route(mut self) -> Self {
        self.router = self.router.route(
            "/new",
            post({
                let app = Arc::clone(&self.application);
                move |payload| new::run(app, payload)
            }),
        );

        self
    }

    async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap();
    }
}
