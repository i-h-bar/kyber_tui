use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[must_use]
pub fn hash(payload: &String) -> Option<[u8; 32]> {
    let hash = HmacSha256::new_from_slice(payload.as_bytes()).ok()?;

    Some(hash.finalize().into_bytes().into())
}
