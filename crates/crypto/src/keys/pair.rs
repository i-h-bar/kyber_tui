use crate::keys::public::{Public, PUBLIC_KEY_SIZE};
use crate::keys::secret::{Secret, SECRET_KEY_SIZE};
use crate::message::EncryptedMessage;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use pqcrypto_traits::kem::{Ciphertext as KemCiphertext, SharedSecret as KemSharedSecret};
use pqcrypto_traits::sign::SignedMessage;
use thiserror::Error;

pub const KEY_PAIR_SIZE: usize = SECRET_KEY_SIZE + PUBLIC_KEY_SIZE;

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
        let (kem_public, kem_secret) = kyber512::keypair();
        let (sign_public, sign_secret) = dilithium3::keypair();

        Ok(KeyPair {
            public: Public::new(kem_public, sign_public),
            secret: Secret::new(kem_secret, sign_secret),
        })
    }

    /// Encrypt `plaintext` for `recipient`, signing it with this keypair. Returning the encrypted message & shared secret
    pub fn encrypt_with_secret(
        &self,
        plaintext: &[u8],
        recipient: &Public,
    ) -> Result<(EncryptedMessage, [u8; 32]), KeyError> {
        let mut rng = OsRng;

        let (shared_secret, kem_ciphertext) = kyber512::encapsulate(recipient.kem());
        let shared_secret: [u8; 32] = shared_secret.as_bytes().try_into().map_err(|_| KeyError::EncapsulationFailed)?;
        let kem_ciphertext: [u8; 768] = kem_ciphertext.as_bytes().try_into().map_err(|_| KeyError::EncapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| KeyError::EncryptionFailed)?;
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);
        let aes_ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| KeyError::EncryptionFailed)?;

        let signed_ciphertext =
            dilithium3::sign(&aes_ciphertext, self.secret.signing())
                .as_bytes()
                .to_vec();

        Ok((
            EncryptedMessage {
                kyber_ciphertext: kem_ciphertext,
                nonce,
                signed_ciphertext,
            },
            shared_secret,
        ))
    }

    /// Encrypt `plaintext` for `recipient`, signing it with this keypair.
    pub fn encrypt(
        &self,
        plaintext: &[u8],
        recipient: &Public,
    ) -> Result<EncryptedMessage, KeyError> {
        self.encrypt_with_secret(plaintext, recipient)
            .map(|(encrypted_message, _)| encrypted_message)
    }

    pub fn encrypt_bytes_to_b64(&self, msg: &[u8], recipient: &Public) -> Result<String, KeyError> {
        Ok(self.encrypt(msg, recipient)?.to_b64())
    }

    pub fn encrypt_to_b64(&self, plaintext: &str, recipient: &Public) -> Result<String, KeyError> {
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
        let aes_ciphertext = dilithium3::open(&signed_message, sender.signing())
            .map_err(|_| KeyError::VerificationFailed)?;

        let ct = kyber512::Ciphertext::from_bytes(&msg.kyber_ciphertext)
            .map_err(|_| KeyError::DecapsulationFailed)?;
        let shared_secret = kyber512::decapsulate(&ct, self.secret.kem());
        let ss_bytes: [u8; 32] = shared_secret.as_bytes().try_into().map_err(|_| KeyError::DecapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&ss_bytes).map_err(|_| KeyError::DecryptionFailed)?;
        cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| KeyError::DecryptionFailed)
    }

    pub fn decrypt_with_secret_from_b64(
        &self,
        msg: &str,
        secret: &[u8; 32],
        sender: &Public,
    ) -> Result<Vec<u8>, KeyError> {
        let encrypted_message =
            EncryptedMessage::from_b64(msg).map_err(|_| KeyError::EncryptionFailed)?;
        self.decrypt_with_secret(&encrypted_message, secret, sender)
    }

    pub fn decrypt_with_secret(
        &self,
        msg: &EncryptedMessage,
        secret: &[u8; 32],
        sender: &Public,
    ) -> Result<Vec<u8>, KeyError> {
        let signed_message = SignedMessage::from_bytes(&msg.signed_ciphertext)
            .map_err(|_| KeyError::DeserialisationFailed)?;
        let aes_ciphertext = dilithium3::open(&signed_message, sender.signing())
            .map_err(|_| KeyError::VerificationFailed)?;

        let cipher = Aes256Gcm::new_from_slice(secret).map_err(|_| KeyError::DecryptionFailed)?;
        cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| KeyError::DecryptionFailed)
    }

    /// Verify this key pair signed the message
    pub fn verify(&self, msg: &EncryptedMessage) -> bool {
        let signed_message = match SignedMessage::from_bytes(&msg.signed_ciphertext) {
            Ok(signed_message) => signed_message,
            Err(_) => return false,
        };
        match dilithium3::open(&signed_message, self.public.signing()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn verify_b64(&self, msg: &str) -> bool {
        let message = match EncryptedMessage::from_b64(msg) {
            Ok(message) => message,
            Err(_) => return false,
        };
        self.verify(&message)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(&self.public.to_bytes());
        buff.extend_from_slice(&self.secret.to_bytes());

        buff
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() != KEY_PAIR_SIZE {
            return Err(KeyError::DeserialisationFailed);
        }

        Ok(Self {
            public: Public::from_bytes(&bytes[..PUBLIC_KEY_SIZE])?,
            secret: Secret::from_bytes(&bytes[PUBLIC_KEY_SIZE..])?,
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
