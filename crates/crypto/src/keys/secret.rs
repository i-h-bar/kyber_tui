use crate::keys::KeyError;
use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use pqcrypto_traits::kem::SecretKey as KemSecretKey;
use pqcrypto_traits::sign::SecretKey as SecretKeyTrait;

pub const SECRET_KEY_SIZE: usize = 5664;

pub struct Secret {
    kem: kyber512::SecretKey,
    signing: dilithium3::SecretKey,
}

impl Secret {
    pub fn new(kem: kyber512::SecretKey, signing: dilithium3::SecretKey) -> Secret {
        Self { kem, signing }
    }

    #[must_use]
    pub fn signing(&self) -> &dilithium3::SecretKey {
        &self.signing
    }

    #[must_use]
    pub fn kem(&self) -> &kyber512::SecretKey {
        &self.kem
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(SECRET_KEY_SIZE);
        buff.extend_from_slice(self.kem.as_bytes());
        buff.extend_from_slice(self.signing.as_bytes());

        buff
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() != SECRET_KEY_SIZE {
            return Err(KeyError::DeserialisationFailed);
        }

        Ok(Self {
            kem: kyber512::SecretKey::from_bytes(&bytes[0..1632])
                .map_err(|_| KeyError::DeserialisationFailed)?,
            signing: dilithium3::SecretKey::from_bytes(&bytes[1632..])
                .map_err(|_| KeyError::DeserialisationFailed)?,
        })
    }
}