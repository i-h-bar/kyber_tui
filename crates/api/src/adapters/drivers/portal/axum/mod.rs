use crate::adapters::drivers::portal::axum::routes::{add, authentication, get, handshake, new};
use crate::domain::Application;
use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use async_trait::async_trait;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod response_codes;
pub mod routes;

pub struct AxumPortal
{
    router: Router,
    listener: TcpListener,
}

async fn health<C, PW>(app: &Application<C, PW>)
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    app.health().await;
}

#[async_trait]
impl<'b> Portal<'b> for AxumPortal
{
    async fn new(bind_addr: Option<&str>) -> Self {
        Self {
            router: Router::new(),
            listener: TcpListener::bind(bind_addr.unwrap_or("0.0.0.0:3000"))
                .await
                .expect("Unable to bind axum server"),
        }
    }

    fn add_health_check_route<'a: 'b, C, PW>(mut self, application: &'a Application<C, PW>) -> Self
    where
        C: Cache + Send + Sync,
        PW: PWStore + Send + Sync
    {
        self.router = self.router.route(
            "/ready",
            get({
                || application.health()
            }),
        );

        self
    }

    fn add_handshake_route(mut self) -> Self {
        // self.router = self.router.route(
        //     "/handshake",
        //     post({
        //         let app = Arc::clone(&self.application);
        //         move |payload| handshake::run(app, payload)
        //     }),
        // );

        self
    }

    fn add_new_user_route(mut self) -> Self {
        // self.router = self.router.route(
        //     "/new",
        //     post({
        //         let app = Arc::clone(&self.application);
        //         move |payload| new::run(app, payload)
        //     }),
        // );

        self
    }

    fn add_authentication_route(mut self) -> Self {
        // self.router = self.router.route(
        //     "/auth",
        //     post({
        //         let app = Arc::clone(&self.application);
        //         move |payload| authentication::run(app, payload)
        //     }),
        // );

        self
    }

    fn add_new_credential_route(mut self) -> Self {
        // self.router = self.router.route(
        //     "/add",
        //     post({
        //         let app = Arc::clone(&self.application);
        //         move |payload| add::run(app, payload)
        //     }),
        // );

        self
    }

    fn add_get_credential_route(mut self) -> Self {
        // self.router = self.router.route(
        //     "/get",
        //     post({
        //         let app = Arc::clone(&self.application);
        //         move |payload| get::run(app, payload)
        //     }),
        // );

        self
    }

    async fn run(self) {
        axum::serve(self.listener, self.router)
            .await
            .expect("Axum server error");
    }
}
