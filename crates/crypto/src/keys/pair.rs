use crate::keys::public::Public;
use crate::keys::secret::Secret;
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use pqc_kyber::{decapsulate, encapsulate, keypair};
use pqcrypto_sphincsplus::sphincsshake128fsimple::{self, SignedMessage};
use pqcrypto_traits::sign::SignedMessage as SignedMessageTrait;
use rand::RngCore;
use rand::rngs::OsRng;
use thiserror::Error;

/// All data transmitted from sender to recipient.
pub struct EncryptedMessage {
    /// Kyber KEM ciphertext — lets the recipient recover the shared secret.
    pub kyber_ciphertext: [u8; 768],
    /// AES-GCM nonce.
    pub nonce: [u8; 12],
    /// SPHINCS+ signed message containing the AES ciphertext; verifies authenticity.
    pub signed_ciphertext: Vec<u8>,
}

impl EncryptedMessage {
    /// Serialize to bytes: [kyber: 768][nonce: 12][`signed_ciphertext`: rest]
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(768 + 12 + self.signed_ciphertext.len());
        buf.extend_from_slice(&self.kyber_ciphertext);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.signed_ciphertext);
        buf
    }

    /// Deserialize from bytes produced by `to_bytes`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() < 768 + 12 {
            return Err(KeyError::DeserializationFailed);
        }
        Ok(EncryptedMessage {
            kyber_ciphertext: bytes[..768].try_into().unwrap(),
            nonce: bytes[768..780].try_into().unwrap(),
            signed_ciphertext: bytes[780..].to_vec(),
        })
    }

    #[must_use]
    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }

    pub fn from_b64(s: &str) -> Result<Self, KeyError> {
        let bytes = STANDARD
            .decode(s)
            .map_err(|_| KeyError::DeserializationFailed)?;
        Self::from_bytes(&bytes)
    }
}

pub struct KeyPair {
    pub public: Public,
    secret: Secret,
}

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
    DeserializationFailed,
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

        // Key exchange: encapsulate a shared secret with the recipient's KEM public key.
        let (kyber_ciphertext, shared_secret) =
            encapsulate(recipient.kem(), &mut rng).map_err(|_| KeyError::EncapsulationFailed)?;

        // Symmetric encryption with the shared secret.
        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| KeyError::EncryptionFailed)?;
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);
        let aes_ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| KeyError::EncryptionFailed)?;

        // Sign the ciphertext and store as raw bytes.
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
        String::from_utf8(self.decrypt(&EncryptedMessage::from_b64(msg)?, sender)?)
            .map_err(|_| KeyError::DeserializationFailed)
    }

    /// Decrypt an `EncryptedMessage`, verifying it was signed by `sender`.
    pub fn decrypt(&self, msg: &EncryptedMessage, sender: &Public) -> Result<Vec<u8>, KeyError> {
        // Reconstruct SignedMessage from bytes; open() verifies and returns the inner AES ciphertext.
        let signed_message = SignedMessage::from_bytes(&msg.signed_ciphertext)
            .map_err(|_| KeyError::DeserializationFailed)?;
        let aes_ciphertext = sphincsshake128fsimple::open(&signed_message, sender.signing())
            .map_err(|_| KeyError::VerificationFailed)?;

        // Recover the shared secret with our KEM private key.
        let shared_secret = decapsulate(&msg.kyber_ciphertext, self.secret.kem())
            .map_err(|_| KeyError::DecapsulationFailed)?;

        // Decrypt.
        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| KeyError::DecryptionFailed)?;
        cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| KeyError::DecryptionFailed)
    }
}
