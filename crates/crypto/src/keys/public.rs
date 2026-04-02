use crate::keys::KeyError;
use base64::{Engine, engine::general_purpose::STANDARD};
use pqcrypto_sphincsplus::sphincsshake128fsimple::PublicKey as SignPublicKey;
use pqcrypto_traits::sign::PublicKey;

pub struct Public {
    kem: [u8; 800],
    signing: SignPublicKey,
}

impl Public {
    pub fn new(kem: [u8; 800], signing: SignPublicKey) -> Public {
        Self { kem, signing }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Public, KeyError> {
        let kem: [u8; 800] = bytes[..800]
            .try_into()
            .map_err(|_| KeyError::DeserializationFailed)?;
        let signing = SignPublicKey::from_bytes(&bytes[800..])
            .map_err(|_| KeyError::DeserializationFailed)?;

        Ok(Public { kem, signing })
    }

    pub fn from_b64(base64: &str) -> Result<Public, KeyError> {
        Ok(Self::from_bytes(
            &STANDARD
                .decode(base64)
                .map_err(|_| KeyError::DeserializationFailed)?,
        )?)
    }

    #[must_use]
    pub fn kem(&self) -> &[u8; 800] {
        &self.kem
    }

    #[must_use]
    pub fn signing(&self) -> &SignPublicKey {
        &self.signing
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(832);
        buff.extend_from_slice(&self.kem);
        buff.extend_from_slice(self.signing.as_bytes());

        buff
    }

    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }
}
