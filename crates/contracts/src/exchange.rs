use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExchangeRequest {
    pub pub_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExchangeResponse {
    pub pub_key: String,
}
