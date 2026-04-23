use crate::domain::errors::routes::DomainError;

pub trait SymEncryptor<'a> {
    fn new_sym(secret: &'a [u8; 32]) -> Self;

    fn encrypt(data: &[u8]) -> Result<Vec<u8>, DomainError>;

    fn decrypt(data: &[u8]) -> Result<Vec<u8>, DomainError>;
}


pub trait SymEncryptorFactory<'a, T: SymEncryptor<'a>> {
    fn generate() -> T;
}