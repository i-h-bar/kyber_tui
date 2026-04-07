use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct HandshakeRequest {
    pub pub_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HandshakeResponse {
    pub public_key: String,
    pub token: String,
}
