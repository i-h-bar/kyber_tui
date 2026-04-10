use pqcrypto::keys::KeyPair;
use pqcrypto::keys::public::Public;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub client_public_key: Public,
    pub server_key_pair: KeyPair,
    pub expiry: SystemTime,
    pub token_key: [u8; 32],
    pub user_id: Option<Uuid>,
}
