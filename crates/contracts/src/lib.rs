use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod handshake;
pub mod new_user;
pub mod token;

#[derive(Serialize, Deserialize)]
pub struct GenericRequest {
    pub session_id: Uuid,
    pub body: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub session_id: Uuid,
    pub body: String,
    pub token: String,
}
