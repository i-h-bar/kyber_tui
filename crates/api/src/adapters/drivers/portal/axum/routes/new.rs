use std::sync::Arc;
use axum::Json;
use http::StatusCode;
use contracts::{GenericRequest, GenericResponse};
use crate::domain::Application;
use crate::ports::services::cache::Cache;

pub async fn run<C>(
    app: Arc<Application<C>>,
    Json(payload): Json<GenericRequest>,
) -> Result<Json<GenericResponse>, StatusCode>
where
    C: Cache + Send + Sync,
{
    match app.create_user(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
