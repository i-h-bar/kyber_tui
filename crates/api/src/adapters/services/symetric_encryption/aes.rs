use crate::domain::errors::routes::DomainError;
use crate::ports::services::encryption::{SymmetricCipher, SymmetricCipherFactory};
use pqcrypto::sym::Symmetric;
use pqcrypto::sym::aes::AesCipher;
use crate::domain::Application;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

pub struct SymmetricEncryption<'a> {
    cipher: AesCipher<'a>,
}

impl<'a> SymmetricCipher<'a> for SymmetricEncryption<'a> {
    fn new(secret: &'a [u8; 32]) -> Self {
        Self {
            cipher: AesCipher::new(secret),
        }
    }

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, DomainError> {
        self.cipher.encrypt(data).map_err(|error| {
            log::error!("Symmetric encryption failed with error: {error}");
            DomainError::Encryption("Error encrypting data".to_string())
        })
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, DomainError> {
        self.cipher.decrypt(data).map_err(|error| {
            log::error!("Symmetric encryption failed with error: {error}");
            DomainError::Decryption("Error decrypting data".to_string())
        })
    }
}

impl<'a, PW, C> SymmetricCipherFactory<'a> for Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    type Output = SymmetricEncryption<'a>;

    fn generate(secret: &'a [u8; 32]) -> Self::Output {
        SymmetricEncryption::new(secret)
    }
}
