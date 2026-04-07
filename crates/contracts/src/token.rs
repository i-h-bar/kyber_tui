use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use kyber_crypto::keys::public::Public;
use uuid::Uuid;

#[derive(Debug)]
pub struct Token {
    pub session_id: Uuid,
    pub pub_key: Public,
    pub expiry_s: u64,
}

impl Token {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(self.session_id.as_bytes());
        buff.extend_from_slice(&self.pub_key.to_bytes());
        buff.extend_from_slice(self.expiry_s.to_be_bytes().as_ref());

        buff
    }

    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let session_id = Uuid::from_slice(&bytes[..16]).unwrap();
        let pub_key = Public::from_bytes(&bytes[16..848]).unwrap();
        let expiry_s = u64::from_be_bytes(bytes[848..].try_into().unwrap());

        Self {
            session_id,
            pub_key,
            expiry_s,
        }
    }

    pub fn from_b64(base64: &str) -> Self {
        Self::from_bytes(&STANDARD.decode(base64).unwrap())
    }
}
