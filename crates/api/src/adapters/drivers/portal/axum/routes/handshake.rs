use crate::domain::Application;
use crate::ports::services::cache::Cache;
use axum::Json;
use contracts::handshake::{HandshakeRequest, HandshakeResponse};
use http::StatusCode;
use std::sync::Arc;

pub async fn run<C>(
    app: Arc<Application<C>>,
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, StatusCode>
where
    C: Cache + Send + Sync,
{
    match app.handshake(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
