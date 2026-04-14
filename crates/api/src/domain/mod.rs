use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

pub mod auth;
pub mod encryption;
pub mod errors;
pub mod routes;
pub mod session;

pub struct Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    cache: C,
    pw_store: PW,
    root_secret: Vec<u8>,
}

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub(crate) fn new(cache: C, pw_store: PW) -> Self {
        let root_secret = std::env::var("ROOT_SECRET")
            .expect("Root secret not found")
            .as_bytes()
            .to_vec();

        Self {
            cache,
            pw_store,
            root_secret,
        }
    }
}
