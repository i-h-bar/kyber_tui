use crate::domain::Application;
use crate::ports::services::cache::Cache;
use axum::Json;
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use http::StatusCode;
use std::sync::Arc;

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
