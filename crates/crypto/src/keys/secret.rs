use crate::keys::KeyError;
use aes_gcm::aead::Buffer;
use pqcrypto_sphincsplus::sphincsshake128fsimple::SecretKey;
use pqcrypto_traits::sign::SecretKey as SecretKeyTrait;

pub struct Secret {
    kem: [u8; 1632],
    signing: SecretKey,
}

impl Secret {
    pub fn new(kem: [u8; 1632], signing: SecretKey) -> Secret {
        Self { kem, signing }
    }

    #[must_use]
    pub fn signing(&self) -> &SecretKey {
        &self.signing
    }

    #[must_use]
    pub fn kem(&self) -> &[u8; 1632] {
        &self.kem
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(1696);
        buff.extend_from_slice(&self.kem);
        buff.extend_from_slice(&self.signing.as_bytes());

        buff
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() != 1696 {
            return Err(KeyError::GenerationFailed);
        }

        Ok(Self {
            kem: bytes[0..1632].try_into().unwrap(),
            signing: SecretKey::from_bytes(&bytes[1632..1696]).unwrap(),
        })
    }
}
