use crate::domain::errors::routes::DomainError;

pub trait SymEncryptor<'a> {
    fn new(secret: &'a [u8; 32]) -> Self;

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, DomainError>;

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, DomainError>;
}

pub trait SymEncryptorFactory<'a> {
    type Output: SymEncryptor<'a>;

    fn generate(secret: &'a [u8; 32]) -> Self::Output;
}
