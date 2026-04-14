use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use crate::asym::CryptoError;
use crate::sym::Symmetric;

pub struct AesCipher<'a> {
    secret: &'a [u8; 32],
}

impl<'a> Symmetric<'a> for AesCipher<'a> {
    fn new(secret: &'a [u8; 32]) -> Self {
        Self { secret }
    }
    
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new_from_slice(self.secret).map_err(|_| CryptoError::GenerationFailed)?;
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let mut stored = nonce_bytes.to_vec();
        stored.extend_from_slice(&ciphertext);
        Ok(stored)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new_from_slice(self.secret).map_err(|_| CryptoError::GenerationFailed)?;
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}