use std::ops::Add;
use std::time::{Duration, SystemTime};
use crate::domain::{Application, errors::routes::exchange::ExchangeError};
use crate::ports::services::cache::{Cache, CachedSession};
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use kyber_crypto::keys::{pair::KeyPair, public::Public};
use uuid::Uuid;

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub async fn exchange(
        &self,
        payload: ExchangeRequest,
    ) -> Result<ExchangeResponse, ExchangeError> {
        if let Err(_) = Public::from_b64(&payload.pub_key) {
            return Err(ExchangeError::InvalidPublicKey);
        };

        let key_pair = KeyPair::generate().map_err(|_| ExchangeError::KeyGenError)?;

        let session = CachedSession {
            id: Uuid::new_v4(),
            client_public_key: payload.pub_key,
            expiry: SystemTime::now().add(Duration::from_secs(30))
        };

        self.cache.save_session(&session).await.map_err(|_| ExchangeError::CacheError)?;

        Ok(ExchangeResponse {
            session_id: session.id,
            pub_key: key_pair.public.to_b64(),
        })
    }
}
