use crate::keys::CryptoError;
use crate::traits::{TryFromBytes, TryToBytes};
use serde::{Deserializer, Serializer, de, ser};

pub mod keys;
pub mod traits;

pub struct EncryptedMessage {
    pub kem_ciphertext: [u8; 768],
    pub nonce: [u8; 12],
    pub signed_ciphertext: Vec<u8>,
}

impl TryToBytes for EncryptedMessage {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        let mut buf = Vec::with_capacity(780 + self.signed_ciphertext.len());
        buf.extend_from_slice(&self.kem_ciphertext);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.signed_ciphertext);

        Ok(buf)
    }
}

impl TryFromBytes for EncryptedMessage {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() < 780 {
            return Err(CryptoError::DeserialisationFailed);
        }
        Ok(EncryptedMessage {
            kem_ciphertext: bytes
                .get(..768)
                .ok_or(CryptoError::DeserialisationFailed)?
                .try_into()
                .map_err(|_| CryptoError::DeserialisationFailed)?,
            nonce: bytes
                .get(768..780)
                .ok_or(CryptoError::DeserialisationFailed)?
                .try_into()
                .map_err(|_| CryptoError::DeserialisationFailed)?,
            signed_ciphertext: bytes
                .get(780..)
                .ok_or(CryptoError::DeserialisationFailed)?
                .to_vec(),
        })
    }
}

impl serde::Serialize for EncryptedMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.to_bytes().map_err(ser::Error::custom)?)
    }
}

impl<'de> serde::Deserialize<'de> for EncryptedMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(de::Error::custom)
    }
}
