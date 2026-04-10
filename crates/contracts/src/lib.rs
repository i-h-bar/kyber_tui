use serde::{Deserialize, Serialize};
use uuid::Uuid;
use kyber_crypto::message::EncryptedMessage;

pub mod handshake;
pub mod new_user;
pub mod token;

#[derive(Serialize, Deserialize)]
pub struct GenericRequest {
    pub session_id: Uuid,
    pub body: EncryptedMessage,
    pub token: EncryptedMessage,
}

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub session_id: Uuid,
    pub body: EncryptedMessage,
    pub token: EncryptedMessage,
}
