use pqcrypto::EncryptedMessage;
use pqcrypto::asym::public::Public;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HandshakeRequest {
    pub pub_key: Public,
}

#[derive(Deserialize, Serialize)]
pub struct HandshakeResponse {
    pub public_key: Public,
    pub token: EncryptedMessage,
}
