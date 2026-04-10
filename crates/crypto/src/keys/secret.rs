use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use serde::{Deserialize, Serialize};

pub const SECRET_KEY_SIZE: usize = 5664;

#[derive(Serialize, Deserialize)]
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
}
