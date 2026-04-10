use thiserror::Error;

#[derive(Error, Debug)]
#[error("Deserialisation failed")]
pub struct DeserialisationError;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserialisationError>;
}