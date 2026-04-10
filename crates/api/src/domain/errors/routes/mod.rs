use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Deserialisation error: {0}")]
    DeserialisationError(String),

    #[error("Serialization error: {0}")]
    SerialisationError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Error hashing password: {0}")]
    HashingError(String),

    #[error("Permission error: {0}")]
    PermissionError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("{0}")]
    GenericError(String),
}
