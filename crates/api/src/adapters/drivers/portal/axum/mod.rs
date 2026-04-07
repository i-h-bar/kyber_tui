use crate::adapters::drivers::portal::axum::routes::{handshake, new};
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use async_trait::async_trait;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod routes;

pub struct AxumPortal<C>
where
    C: Cache + Send + Sync + 'static,
{
    application: Arc<Application<C>>,
    router: Router,
    listener: TcpListener,
}

async fn health<C>(app: Arc<Application<C>>)
where
    C: Cache + Send + Sync,
{
    app.health().await;
}

#[async_trait]
impl<C> Portal<C> for AxumPortal<C>
where
    C: Cache + Send + Sync,
{
    async fn new(application: Application<C>, bind_addr: Option<&str>) -> Self {
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
