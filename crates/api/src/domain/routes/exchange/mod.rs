use crate::domain::Application;
use crate::ports::services::cache::{Cache, CachedSession};
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use uuid::Uuid;
use kyber_crypto::keys::{pair::KeyPair, public::Public};

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub async fn exchange(&self, payload: ExchangeRequest) -> ExchangeResponse {
        let _pub_key = Public::from_b64(&payload.pub_key).unwrap();
        let key_pair = KeyPair::generate().unwrap();

        let session = CachedSession{
            id: Uuid::new_v4(),
            client_public_key: payload.pub_key
        };

        self.cache.save_session(&session).await;

        ExchangeResponse {
            session_id: session.id,
            pub_key: key_pair.public.to_b64(),
        }
    }
}
