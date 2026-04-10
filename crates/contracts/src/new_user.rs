use kyber_crypto::keys::CryptoError;
use kyber_crypto::keys::traits::{TryFromBytes, TryToBytes};
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

impl TryToBytes for NewUserResponse {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for NewUserResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}

impl TryToBytes for NewUserRequest {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for NewUserRequest {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}
