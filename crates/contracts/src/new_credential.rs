use pqcrypto::keys::CryptoError;
use pqcrypto::traits::{TryFromBytes, TryToBytes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewCredentialRequest {
    pub service: Vec<u8>,
    pub username: Vec<u8>,
    pub password: Vec<u8>,
    pub notes: Option<Vec<Vec<u8>>>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCredentialResponse {
    pub added: u16,
}

impl TryToBytes for NewCredentialRequest {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for NewCredentialRequest {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}

impl TryToBytes for NewCredentialResponse {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for NewCredentialResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}
