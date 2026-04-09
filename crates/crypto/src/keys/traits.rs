use thiserror::Error;

#[derive(Error, Debug)]
#[error("Deserialisation failed")]
pub struct DeserialisationError;

pub trait B64Serialisation: Sized {
    fn to_b64(&self) -> String;

    fn from_b64(b64: &str) -> Result<Self, DeserialisationError>;
}