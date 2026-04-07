use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandshakeError {
    #[error("Cache save error")]
    CacheError,

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Key generation error")]
    KeyGenError,

    #[error("Token creation error")]
    TokenCreationError,

    #[error("Token encryption error")]
    TokenEncryptionError,
}
