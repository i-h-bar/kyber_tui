use serde::{Deserialize, Serialize};
use kyber_crypto::keys::traits::{DeserialisationError, FromBytes, ToBytes};

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserResponse {
    pub success: bool,
}


impl ToBytes for NewUserResponse {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

impl FromBytes for NewUserResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserialisationError> {
        serde_json::from_slice(bytes).map_err(|_| DeserialisationError)
    }
}

impl ToBytes for NewUserRequest {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

impl FromBytes for NewUserRequest {
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserialisationError> {
        serde_json::from_slice(bytes).map_err(|_| DeserialisationError)
    }
}

