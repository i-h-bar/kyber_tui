use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use serde::{Deserialize, Serialize};

pub const PUBLIC_KEY_SIZE: usize = 2752;

#[derive(Serialize, Deserialize, Clone)]
pub struct Public {
    kem: kyber512::PublicKey,
    signing: dilithium3::PublicKey,
}

impl Public {
    #[must_use]
    pub fn new(kem: kyber512::PublicKey, signing: dilithium3::PublicKey) -> Public {
        Self { kem, signing }
    }

    #[must_use]
    pub fn kem(&self) -> &kyber512::PublicKey {
        &self.kem
    }

    #[must_use]
    pub fn signing(&self) -> &dilithium3::PublicKey {
        &self.signing
    }
}
