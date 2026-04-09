use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PreAuthToken {
    pub session_id: Uuid,
    pub expiry_s: u64,
}

impl PreAuthToken {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(self.session_id.as_bytes());
        buff.extend_from_slice(self.expiry_s.to_be_bytes().as_ref());

        buff
    }

    pub fn to_b64(&self) -> String {
        STANDARD.encode(self.to_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let session_id = Uuid::from_slice(&bytes[..16]).unwrap();
        let expiry_s = u64::from_be_bytes(bytes[16..].try_into().unwrap());

        Self {
            session_id,
            expiry_s,
        }
    }

    pub fn from_b64(base64: &str) -> Self {
        Self::from_bytes(&STANDARD.decode(base64).unwrap())
    }
}
