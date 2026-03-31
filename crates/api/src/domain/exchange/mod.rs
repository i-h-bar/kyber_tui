use axum::Json;
use http::StatusCode;
use kyber_crypto::keys::{public::Public, pair::KeyPair};
use contracts::exchange::{ExchangeRequest, ExchangeResponse};


pub async fn exchange(Json(payload): Json<ExchangeRequest>) -> (StatusCode, Json<ExchangeResponse>) {
    let pub_key = Public::from_b64(&payload.pub_key).unwrap();
    let key_pair = KeyPair::generate().unwrap();
    let response = ExchangeResponse{
        session_id: payload.session_id,
        pub_key: key_pair.public.to_b64()
    };

    (StatusCode::OK, Json(response))
}