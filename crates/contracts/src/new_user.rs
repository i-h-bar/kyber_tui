use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use kyber_crypto::keys::traits::{B64Serialisation, DeserialisationError};

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserResponse {
    pub success: bool,
}

impl B64Serialisation for NewUserResponse {
    fn to_b64(&self) -> String {
        STANDARD.encode(serde_json::to_vec(&self).unwrap().as_slice())
    }

    fn from_b64(b64: &str) -> Result<Self, DeserialisationError> {
        serde_json::from_slice(STANDARD.decode(b64).unwrap().as_slice()).map_err(|_| DeserialisationError)
    }
}

impl B64Serialisation for NewUserRequest {
    fn to_b64(&self) -> String {
        STANDARD.encode(serde_json::to_vec(&self).unwrap().as_slice())
    }

    fn from_b64(b64: &str) -> Result<Self, DeserialisationError> {
        serde_json::from_slice(STANDARD.decode(b64).unwrap().as_slice()).map_err(|_| DeserialisationError)
    }
}
