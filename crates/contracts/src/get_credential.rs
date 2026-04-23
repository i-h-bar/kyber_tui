use pqcrypto::asym::CryptoError;
use pqcrypto::traits::{TryFromBytes, TryToBytes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetCredentialRequest {
    pub service: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCredentialResponse {
    pub service: Vec<u8>,
    pub username: Vec<u8>,
    pub password: Vec<u8>,
    pub notes: Option<Vec<Vec<u8>>>,
}

impl TryToBytes for GetCredentialRequest {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for GetCredentialRequest {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryToBytes for GetCredentialResponse {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        serde_json::to_vec(self).map_err(|_| CryptoError::SerializationFailed)
    }
}

impl TryFromBytes for GetCredentialResponse {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        serde_json::from_slice(bytes).map_err(|_| CryptoError::SerializationFailed)
    }
}
