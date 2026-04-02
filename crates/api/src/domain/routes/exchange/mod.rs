use crate::domain::Application;
use crate::ports::services::cache::Cache;
use axum::Json;
use contracts::exchange::{ExchangeRequest, ExchangeResponse};
use http::StatusCode;
use kyber_crypto::keys::{pair::KeyPair, public::Public};

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub async fn exchange(&self, payload: ExchangeRequest) -> ExchangeResponse {
        let pub_key = Public::from_b64(&payload.pub_key).unwrap();
        let key_pair = KeyPair::generate().unwrap();

        ExchangeResponse {
            session_id: payload.session_id,
            pub_key: key_pair.public.to_b64(),
        }
    }
}
