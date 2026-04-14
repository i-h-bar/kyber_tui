use crate::asym::CryptoError;

pub mod aes;


pub trait Symmetric<'a> {
    fn new(secret: &'a [u8; 32]) -> Self;
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
}
