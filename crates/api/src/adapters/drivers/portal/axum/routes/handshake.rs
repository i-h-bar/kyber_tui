use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use axum::Json;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use http::StatusCode;
use std::sync::Arc;

pub async fn run<C, PW>(
    app: Arc<Application<C, PW>>,
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, StatusCode>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    match app.handshake(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
