use crate::asym::CryptoError;

pub trait TryToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError>;
}

pub trait TryFromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError>;
}
