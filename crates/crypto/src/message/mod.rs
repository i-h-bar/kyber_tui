use crate::keys::KeyError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use pqcrypto_traits::sign::SignedMessage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptedMessageError {
    #[error("Deserialisation failed")]
    DeserialisationError,
}

pub struct EncryptedMessage {
    pub kyber_ciphertext: [u8; 768],
    pub nonce: [u8; 12],
    pub signed_ciphertext: Vec<u8>,
}

impl EncryptedMessage {
    /// Serialize to bytes: [kyber: 768][nonce: 12][`signed_ciphertext`: rest]
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(780 + self.signed_ciphertext.len());
        buf.extend_from_slice(&self.kyber_ciphertext);
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
            kyber_ciphertext: bytes[..768]
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
