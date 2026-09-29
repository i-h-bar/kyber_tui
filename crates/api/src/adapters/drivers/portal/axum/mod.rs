use crate::adapters::drivers::portal::axum::routes::{add, authentication, get, handshake, new};
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use async_trait::async_trait;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use axum::extract::State;
use tokio::net::TcpListener;

pub mod response_codes;
pub mod routes;

type AxumAppState<C, PW> = State<Arc<Application<C, PW>>>;

pub struct AxumPortal<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    application: Arc<Application<C, PW>>,
    router: Router<Arc<Application<C, PW>>>,
    listener: TcpListener,
}

async fn health<C, PW>(State(app): AxumAppState<C, PW>)
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    app.health().await;
}

#[async_trait]
impl<C, PW> Portal<C, PW> for AxumPortal<C, PW>
where
    C: Cache + Send + Sync + 'static,
    PW: PWStore + Send + Sync + 'static,
{
    async fn new(bind_addr: Option<&str>, application: Application<C, PW>) -> Self {
        Self {
            application: Arc::new(application),
            router: Router::new(),
            listener: TcpListener::bind(bind_addr.unwrap_or("0.0.0.0:3000"))
                .await
                .expect("Unable to bind axum server"),
        }
    }

    fn add_health_check_route(mut self) -> Self
    {
        self.router = self.router.route(
            "/ready",
            get(health),
        );

        self
    }

    fn add_handshake_route(mut self) -> Self {
        self.router = self.router.route(
            "/handshake",
            post(handshake::run),
        );

        self
    }

    fn add_new_user_route(mut self) -> Self {
        self.router = self.router.route(
            "/new",
            post(new::run),
        );

        self
    }

    fn add_authentication_route(mut self) -> Self {
        self.router = self.router.route(
            "/auth",
            post(authentication::run),
        );

        self
    }

    fn add_new_credential_route(mut self) -> Self {
        self.router = self.router.route(
            "/add",
            post(add::run)
        );

        self
    }

    fn add_get_credential_route(mut self) -> Self {
        self.router = self.router.route(
            "/get",
            post(get::run),
        );

        self
    }

    async fn run(self) {
        axum::serve(self.listener, self.router.with_state(self.application))
            .await
            .expect("Axum server error");
    }
}
