use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod handshake;
pub mod token;
pub mod new_user;


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