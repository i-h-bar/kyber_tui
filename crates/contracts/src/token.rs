use kyber_crypto::keys::KeyError;
use kyber_crypto::keys::traits::{TryFromBytes, TryToBytes};
use uuid::Uuid;

pub struct PreAuthToken {
    pub session_id: Uuid,
    pub expiry_s: u64,
}

impl TryToBytes for PreAuthToken {
    fn to_bytes(&self) -> Result<Vec<u8>, KeyError> {
        let mut buff = Vec::new();
        buff.extend_from_slice(self.session_id.as_bytes());
        buff.extend_from_slice(self.expiry_s.to_be_bytes().as_ref());

        Ok(buff)
    }
}

impl TryFromBytes for PreAuthToken {
    fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        let session_id = Uuid::from_slice(
            &bytes
                .get(..16)
                .ok_or_else(|| KeyError::DeserialisationFailed)?,
        )
        .map_err(|_| KeyError::DeserialisationFailed)?;
        let expiry_s = u64::from_be_bytes(
            bytes
                .get(16..)
                .ok_or_else(|| KeyError::DeserialisationFailed)?
                .try_into()
                .map_err(|_| KeyError::DeserialisationFailed)?,
        );

        Ok(Self {
            session_id,
            expiry_s,
        })
    }
}
