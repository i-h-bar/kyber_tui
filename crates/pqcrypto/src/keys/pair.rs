use crate::EncryptedMessage;
use crate::keys::public::{PUBLIC_KEY_SIZE, Public};
use crate::keys::secret::{SECRET_KEY_SIZE, Secret};
use crate::traits::{TryFromBytes, TryToBytes};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use pqcrypto_dilithium::dilithium3;
use pqcrypto_kyber::kyber512;
use pqcrypto_traits::kem::{Ciphertext as KemCiphertext, SharedSecret as KemSharedSecret};
use pqcrypto_traits::sign::SignedMessage;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const KEY_PAIR_SIZE: usize = SECRET_KEY_SIZE + PUBLIC_KEY_SIZE;

#[derive(Error, Debug)]
pub enum CryptoError {
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
    #[error("serialization failed")]
    SerializationFailed,
}

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub public: Public,
    secret: Secret,
}

impl KeyPair {
    /// Generate a fresh Kyber KEM + Dilithium keypair.
    pub fn generate() -> Result<Self, CryptoError> {
        let (kem_public, kem_secret) = kyber512::keypair();
        let (sign_public, sign_secret) = dilithium3::keypair();

        Ok(KeyPair {
            public: Public::new(kem_public, sign_public),
            secret: Secret::new(kem_secret, sign_secret),
        })
    }

    /// Encrypt `plaintext` for `recipient`, signing it with this keypair. Returning the encrypted message & shared secret
    pub fn encrypt_with_secret<T: TryToBytes>(
        &self,
        plaintext: &T,
        recipient: &Public,
    ) -> Result<(EncryptedMessage, [u8; 32]), CryptoError> {
        self.encrypt_raw(plaintext.to_bytes()?.as_slice(), recipient)
    }

    /// Encrypt `plaintext` for `recipient`, signing it with this keypair.
    pub fn encrypt_raw(
        &self,
        plaintext: &[u8],
        recipient: &Public,
    ) -> Result<(EncryptedMessage, [u8; 32]), CryptoError> {
        let mut rng = OsRng;

        let (shared_secret, kem_ciphertext) = kyber512::encapsulate(recipient.kem());
        let shared_secret: [u8; 32] = shared_secret
            .as_bytes()
            .try_into()
            .map_err(|_| CryptoError::EncapsulationFailed)?;
        let kem_ciphertext: [u8; 768] = kem_ciphertext
            .as_bytes()
            .try_into()
            .map_err(|_| CryptoError::EncapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&shared_secret).map_err(|_| CryptoError::EncryptionFailed)?;
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);
        let aes_ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let signed_ciphertext = dilithium3::sign(&aes_ciphertext, self.secret.signing())
            .as_bytes()
            .to_vec();

        Ok((
            EncryptedMessage {
                kem_ciphertext,
                nonce,
                signed_ciphertext,
            },
            shared_secret,
        ))
    }

    pub fn encrypt<T: TryToBytes>(
        &self,
        obj: &T,
        recipient: &Public,
    ) -> Result<EncryptedMessage, CryptoError> {
        self.encrypt_raw(obj.to_bytes()?.as_slice(), recipient)
            .map(|(encrypted_message, _)| encrypted_message)
    }

    /// Decrypt an `EncryptedMessage`, verifying it was signed by `sender`.
    pub fn decrypt_raw(
        &self,
        msg: &EncryptedMessage,
        sender: &Public,
    ) -> Result<Vec<u8>, CryptoError> {
        let signed_message = SignedMessage::from_bytes(&msg.signed_ciphertext)
            .map_err(|_| CryptoError::DeserialisationFailed)?;
        let aes_ciphertext = dilithium3::open(&signed_message, sender.signing())
            .map_err(|_| CryptoError::VerificationFailed)?;

        let ct = kyber512::Ciphertext::from_bytes(&msg.kem_ciphertext)
            .map_err(|_| CryptoError::DecapsulationFailed)?;
        let shared_secret = kyber512::decapsulate(&ct, self.secret.kem());
        let ss_bytes: [u8; 32] = shared_secret
            .as_bytes()
            .try_into()
            .map_err(|_| CryptoError::DecapsulationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(&ss_bytes).map_err(|_| CryptoError::DecryptionFailed)?;
        cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)
    }

    pub fn decrypt<T: TryFromBytes>(
        &self,
        msg: &EncryptedMessage,
        sender: &Public,
    ) -> Result<T, CryptoError> {
        let bytes = self.decrypt_raw(msg, sender)?;

        T::from_bytes(&bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }

    pub fn decrypt_with_secret<T: TryFromBytes>(
        &self,
        msg: &EncryptedMessage,
        secret: &[u8; 32],
        sender: &Public,
    ) -> Result<T, CryptoError> {
        let signed_message = SignedMessage::from_bytes(&msg.signed_ciphertext)
            .map_err(|_| CryptoError::DeserialisationFailed)?;
        let aes_ciphertext = dilithium3::open(&signed_message, sender.signing())
            .map_err(|_| CryptoError::VerificationFailed)?;

        let cipher =
            Aes256Gcm::new_from_slice(secret).map_err(|_| CryptoError::DecryptionFailed)?;
        let bytes = cipher
            .decrypt(Nonce::from_slice(&msg.nonce), aes_ciphertext.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)?;

        T::from_bytes(&bytes).map_err(|_| CryptoError::DeserialisationFailed)
    }
}
