use kyber_crypto::keys::public::Public;
use kyber_crypto::keys::traits::TryFromBytes;
use kyber_crypto::keys::{CryptoError, KeyPair};
use kyber_crypto::message::EncryptedMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod handshake;
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
