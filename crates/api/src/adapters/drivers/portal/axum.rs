use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use async_trait::async_trait;
use axum::routing::{get, post};
use axum::{Json, Router};
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use http::StatusCode;
use std::sync::Arc;
use tokio::net::TcpListener;

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

async fn exchange<C>(
    app: Arc<Application<C>>,
    Json(payload): Json<ExchangeRequest>,
) -> Result<Json<ExchangeResponse>, StatusCode>
where
    C: Cache + Send + Sync,
{
    match app.exchange(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[async_trait]
impl<C> Portal<C> for AxumPortal<C>
where
    C: Cache + Send + Sync
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

    fn add_exchange_route(mut self) -> Self {
        self.router = self.router.route(
            "/exchange",
            post({
                let app = Arc::clone(&self.application);
                move | payload | exchange(app, payload)
            }),
        );

        self
    }

    async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap();
    }
}
