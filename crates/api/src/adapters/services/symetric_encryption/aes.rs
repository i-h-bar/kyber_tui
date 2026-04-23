use pqcrypto::sym::aes::AesCipher;
use pqcrypto::sym::Symmetric;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::encryption::SymEncryptor;


struct SymmetricEncryption {
    
}

impl<'a> SymEncryptor<'a> for AesCipher<'a> {
    fn new_sym(secret: &'a [u8; 32]) -> Self {
        AesCipher::new(secret)
    }

    fn encrypt(data: &[u8]) -> Result<Vec<u8>, DomainError> {
        todo!()
    }

    fn decrypt(data: &[u8]) -> Result<Vec<u8>, DomainError> {
        todo!()
    }
}