use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserializer, Serializer, de};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptedMessageError {
    #[error("Deserialisation failed")]
    DeserialisationError,
}

pub struct EncryptedMessage {
    pub kem_ciphertext: [u8; 768],
    pub nonce: [u8; 12],
    pub signed_ciphertext: Vec<u8>,
}

impl EncryptedMessage {
    /// Serialize to bytes: [kyber: 768][nonce: 12][`signed_ciphertext`: rest]
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(780 + self.signed_ciphertext.len());
        buf.extend_from_slice(&self.kem_ciphertext);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.signed_ciphertext);
        buf
    }

    /// Deserialize from bytes produced by `to_bytes`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EncryptedMessageError> {
        if bytes.len() < 780 {
            return Err(EncryptedMessageError::DeserialisationError);
        }
        Ok(EncryptedMessage {
            kem_ciphertext: bytes[..768]
                .try_into()
                .map_err(|_| EncryptedMessageError::DeserialisationError)?,
            nonce: bytes[768..780]
                .try_into()
                .map_err(|_| EncryptedMessageError::DeserialisationError)?,
            signed_ciphertext: bytes[780..].to_vec(),
        })
    }

    #[must_use]
    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }

    pub fn from_b64(base64: &str) -> Result<Self, EncryptedMessageError> {
        let bytes = STANDARD
            .decode(base64)
            .map_err(|_| EncryptedMessageError::DeserialisationError)?;

        Self::from_bytes(&bytes)
    }
}

impl serde::Serialize for EncryptedMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.to_bytes())
    }
}

impl<'de> serde::Deserialize<'de> for EncryptedMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(de::Error::custom)
    }
}
