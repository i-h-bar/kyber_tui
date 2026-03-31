use pqcrypto_sphincsplus::sphincsshake128fsimple::SecretKey;

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
}