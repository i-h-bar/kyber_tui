use pqcrypto::keys::CryptoError;
use pqcrypto::traits::{TryFromBytes, TryToBytes};
use uuid::Uuid;

pub struct PreAuthToken {
    pub session_id: Uuid,
    pub expiry_s: u64,
}

impl TryToBytes for PreAuthToken {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        let mut buff = Vec::new();
        buff.extend_from_slice(self.session_id.as_bytes());
        buff.extend_from_slice(self.expiry_s.to_be_bytes().as_ref());

        Ok(buff)
    }
}

impl TryFromBytes for PreAuthToken {
    fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let session_id = Uuid::from_slice(
            bytes
                .get(..16)
                .ok_or(CryptoError::DeserialisationFailed)?,
        )
        .map_err(|_| CryptoError::DeserialisationFailed)?;
        let expiry_s = u64::from_be_bytes(
            bytes
                .get(16..)
                .ok_or(CryptoError::DeserialisationFailed)?
                .try_into()
                .map_err(|_| CryptoError::DeserialisationFailed)?,
        );

        Ok(Self {
            session_id,
            expiry_s,
        })
    }
}
