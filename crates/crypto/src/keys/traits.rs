use crate::keys::KeyError;

pub trait TryToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, KeyError>;
}

pub trait TryFromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError>;
}
