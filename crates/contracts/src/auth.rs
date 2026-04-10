use pqcrypto::keys::CryptoError;
use pqcrypto::traits::{TryFromBytes, TryToBytes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
}

impl TryToBytes for AuthRequest {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for AuthRequest {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}

impl TryToBytes for AuthResponse {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for AuthResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}
