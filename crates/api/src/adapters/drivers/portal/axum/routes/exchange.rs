use std::sync::Arc;
use axum::Json;
use crate::domain::Application;
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use http::StatusCode;
use crate::ports::services::cache::Cache;

pub async fn run<C>(
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