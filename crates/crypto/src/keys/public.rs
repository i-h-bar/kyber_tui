use crate::keys::KeyError;
use base64::{Engine, engine::general_purpose::STANDARD};
use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use pqcrypto_traits::kem::PublicKey as KemPublicKey;
use pqcrypto_traits::sign::PublicKey;
use serde::{Deserialize, Serialize};

pub const PUBLIC_KEY_SIZE: usize = 2752;

#[derive(Serialize, Deserialize)]
pub struct Public {
    kem: kyber512::PublicKey,
    signing: dilithium3::PublicKey,
}

impl Public {
    #[must_use]
    pub fn new(kem: kyber512::PublicKey, signing: dilithium3::PublicKey) -> Public {
        Self { kem, signing }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Public, KeyError> {
        if bytes.len() != PUBLIC_KEY_SIZE {
            return Err(KeyError::DeserialisationFailed);
        }

        let kem = kyber512::PublicKey::from_bytes(&bytes[..800])
            .map_err(|_| KeyError::DeserialisationFailed)?;
        let signing = dilithium3::PublicKey::from_bytes(&bytes[800..])
            .map_err(|_| KeyError::DeserialisationFailed)?;

        Ok(Public { kem, signing })
    }

    pub fn from_b64(base64: &str) -> Result<Public, KeyError> {
        Self::from_bytes(
            &STANDARD
                .decode(base64)
                .map_err(|_| KeyError::DeserialisationFailed)?,
        )
    }

    #[must_use]
    pub fn kem(&self) -> &kyber512::PublicKey {
        &self.kem
    }

    #[must_use]
    pub fn signing(&self) -> &dilithium3::PublicKey {
        &self.signing
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(PUBLIC_KEY_SIZE);
        buff.extend_from_slice(self.kem.as_bytes());
        buff.extend_from_slice(self.signing.as_bytes());

        buff
    }

    #[must_use]
    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }
}
