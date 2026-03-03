use aes_gcm::aead::OsRng;
use pqc_kyber::keypair;
use pqcrypto_sphincsplus::sphincsshake128fsimple;
use pqcrypto_sphincsplus::sphincsshake128fsimple::{SecretKey, PublicKey};
use thiserror::Error;
use crate::keys::pair::KeyError::GenerationError;

struct Public {
    kem: [u8; 800],
    signing: PublicKey,
}

struct Secret {
    kem: [u8; 1632],
    signing: SecretKey,
}

pub struct KeyPair {
    public: Public,
    secret: Secret,
}

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("Error generating keypair")]
    GenerationError,
}

impl KeyPair {
    pub fn generate() -> Result<KeyPair, KeyError> {
        let mut rng = OsRng;
        let kem_pair = keypair(&mut rng)
            .map_err(|e| GenerationError)?;

        // Alice generates a SPHINCS+ keypair for signing
        let (public_signing_key, secret_signing_key) = sphincsshake128fsimple::keypair();
        let secret = Secret {
            kem: kem_pair.secret,
            signing: secret_signing_key,
        };
        let public = Public {
            kem: kem_pair.public,
            signing: public_signing_key,
        };

        Ok(
            Self {
                secret,
                public
            }
        )
    }
}