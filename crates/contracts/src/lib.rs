use pqcrypto::EncryptedMessage;
use pqcrypto::keys::public::Public;
use pqcrypto::keys::{CryptoError, KeyPair};
use pqcrypto::traits::TryFromBytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod auth;
pub mod handshake;
pub mod new_credential;
pub mod new_user;
pub mod token;

#[derive(Serialize, Deserialize)]
pub struct GenericRequest {
    pub session_id: Uuid,
    pub body: EncryptedMessage,
    pub token: EncryptedMessage,
}

impl GenericRequest {
    pub fn get_message<T: TryFromBytes>(
        &self,
        key_pair: &KeyPair,
        public: &Public,
    ) -> Result<T, CryptoError> {
        key_pair.decrypt(&self.body, public)
    }

    pub fn get_token<T: TryFromBytes>(
        &self,
        key_pair: &KeyPair,
        public: &Public,
    ) -> Result<T, CryptoError> {
        key_pair.decrypt(&self.token, public)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub body: EncryptedMessage,
    pub token: EncryptedMessage,
}

impl GenericResponse {
    pub fn get_message<T: TryFromBytes>(
        &self,
        key_pair: &KeyPair,
        public: &Public,
    ) -> Result<T, CryptoError> {
        key_pair.decrypt(&self.body, public)
    }

    pub fn get_token<T: TryFromBytes>(
        &self,
        key_pair: &KeyPair,
        public: &Public,
    ) -> Result<T, CryptoError> {
        key_pair.decrypt(&self.token, public)
    }
}
