use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Deserialisation error: {0}")]
    Deserialisation(String),

    #[error("Serialization error: {0}")]
    Serialisation(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Error hashing password: {0}")]
    Hashing(String),

    #[error("Permission error: {0}")]
    Permission(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("{0}")]
    Generic(String),
}
