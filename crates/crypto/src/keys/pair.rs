use crate::keys::public::Public;
use crate::keys::secret::Secret;
use crate::message::EncryptedMessage;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use pqc_kyber::{decapsulate, encapsulate, keypair};
use pqcrypto_sphincsplus::sphincsshake128fsimple;
use pqcrypto_traits::sign::SignedMessage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("keypair generation failed")]
    GenerationFailed,
    #[error("encapsulation failed")]
    EncapsulationFailed,
    #[error("decapsulation failed")]
    DecapsulationFailed,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("signature verification failed")]
    VerificationFailed,
    #[error("deserialization failed")]
    DeserialisationFailed,
}

pub struct KeyPair {
    pub public: Public,
    secret: Secret,
}

impl KeyPair {
    /// Generate a fresh Kyber KEM + SPHINCS+ keypair.
    pub fn generate() -> Result<Self, KeyError> {
        let mut rng = OsRng;
        let kem = keypair(&mut rng).map_err(|_| KeyError::GenerationFailed)?;
        let (sign_public, sign_secret) = sphincsshake128fsimple::keypair();

        Ok(KeyPair {
            public: Public::new(kem.public, sign_public),
            secret: Secret::new(kem.secret, sign_secret),
        })
    }

    /// Encrypt `plaintext` for `recipient`, signing it with this keypair.
    pub fn encrypt(
        &self,
        plaintext: &[u8],
        recipient: &Public,
    ) -> Result<EncryptedMessage, KeyError> {
        let mut rng = OsRng;

        let (kyber_ciphertext, shared_secret) =
            encapsulate(recipient.kem(), &mut rng).map_err(|_| KeyError::EncapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| KeyError::EncryptionFailed)?;
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);
        let aes_ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| KeyError::EncryptionFailed)?;

        let signed_ciphertext =
            sphincsshake128fsimple::sign(&aes_ciphertext, self.secret.signing())
                .as_bytes()
                .to_vec();

        Ok(EncryptedMessage {
            kyber_ciphertext,
            nonce,
            signed_ciphertext,
        })
    }

    pub fn encrypt_b64(&self, plaintext: &str, recipient: &Public) -> Result<String, KeyError> {
        Ok(self.encrypt(plaintext.as_bytes(), recipient)?.to_b64())
    }

    pub fn decrypt_b64(&self, msg: &str, sender: &Public) -> Result<String, KeyError> {
        String::from_utf8(self.decrypt(
            &EncryptedMessage::from_b64(msg).map_err(|_| KeyError::DeserialisationFailed)?,
            sender,
        )?)
        .map_err(|_| KeyError::DeserialisationFailed)
    }

    /// Decrypt an `EncryptedMessage`, verifying it was signed by `sender`.
    pub fn decrypt(&self, msg: &EncryptedMessage, sender: &Public) -> Result<Vec<u8>, KeyError> {
        let signed_message = SignedMessage::from_bytes(&msg.signed_ciphertext)
            .map_err(|_| KeyError::DeserialisationFailed)?;
        let aes_ciphertext = sphincsshake128fsimple::open(&signed_message, sender.signing())
            .map_err(|_| KeyError::VerificationFailed)?;

        let shared_secret = decapsulate(&msg.kyber_ciphertext, self.secret.kem())
            .map_err(|_| KeyError::DecapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| KeyError::DecryptionFailed)?;
        cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| KeyError::DecryptionFailed)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(&self.public.to_bytes());
        buff.extend_from_slice(&self.secret.to_bytes());

        buff
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() != 2528 {
            return Err(KeyError::DeserialisationFailed);
        }

        Ok(Self {
            public: Public::from_bytes(&bytes[..832])?,
            secret: Secret::from_bytes(&bytes[832..])?,
        })
    }

    pub fn to_b64(&self) -> String {
        STANDARD.encode(&self.to_bytes())
    }

    pub fn from_b64(base64: &str) -> Result<Self, KeyError> {
        Ok(Self::from_bytes(
            &STANDARD
                .decode(base64)
                .map_err(|_| KeyError::DeserialisationFailed)?,
        )?)
    }
}
