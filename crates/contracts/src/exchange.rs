use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct ExchangeRequest {
    pub pub_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExchangeResponse {
    pub session_id: Uuid,
    pub pub_key: String,
}
