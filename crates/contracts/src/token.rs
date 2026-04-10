use uuid::Uuid;
use kyber_crypto::keys::traits::{DeserialisationError, FromBytes, ToBytes};

pub struct PreAuthToken {
    pub session_id: Uuid,
    pub expiry_s: u64,
}

impl ToBytes for PreAuthToken {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(self.session_id.as_bytes());
        buff.extend_from_slice(self.expiry_s.to_be_bytes().as_ref());

        buff
    }
}

impl FromBytes for PreAuthToken {
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserialisationError> {
        let session_id = Uuid::from_slice(&bytes[..16]).unwrap();
        let expiry_s = u64::from_be_bytes(bytes[16..].try_into().unwrap());

        Ok(Self {
            session_id,
            expiry_s,
        })
    }
}
