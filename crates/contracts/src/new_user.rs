use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserResponse {
    pub success: bool,
}

impl NewUserResponse {
    pub fn to_b64(&self) -> String {
        STANDARD.encode(serde_json::to_vec(&self).unwrap().as_slice())
    }

    pub fn from_b64(b64: &str) -> Self {
        serde_json::from_slice(STANDARD.decode(b64).unwrap().as_slice()).unwrap()
    }
}


impl NewUserRequest {
    pub fn to_b64(&self) -> String {
        STANDARD.encode(serde_json::to_vec(&self).unwrap().as_slice())
    }

    pub fn from_b64(b64: &str) -> Self {
        serde_json::from_slice(STANDARD.decode(b64).unwrap().as_slice()).unwrap()
    }
}