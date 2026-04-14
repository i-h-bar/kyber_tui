use crate::adapters::drivers::portal::axum::response_codes::map_domain_error;
use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use axum::Json;
use contracts::{GenericRequest, GenericResponse};
use http::StatusCode;
use std::sync::Arc;
use std::time::Instant;

pub async fn run<C, PW>(
    app: Arc<Application<C, PW>>,
    Json(payload): Json<GenericRequest>,
) -> Result<Json<GenericResponse>, StatusCode>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    log::info!("New user request received");
    let start = Instant::now();
    let response = match app.create_user(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(map_domain_error(&error)),
    };
    log::info!("New user request finished took {:?}", start.elapsed());

    response
}
