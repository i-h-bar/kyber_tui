use crate::adapters::drivers::portal::axum::response_codes::map_domain_error;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use axum::Json;
use contracts::{GenericRequest, GenericResponse};
use http::StatusCode;
use std::time::Instant;
use axum::extract::State;
use crate::adapters::drivers::portal::axum::AxumAppState;

pub async fn run<C, PW>(
    State(app): AxumAppState<C, PW>,
    Json(payload): Json<GenericRequest>,
) -> Result<Json<GenericResponse>, StatusCode>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    log::info!("Get credential request received");
    let start = Instant::now();
    let response = match app.get_credential(payload).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(map_domain_error(&error)),
    };
    log::info!("Get credential finished. It took {:?}", start.elapsed());

    response
}
