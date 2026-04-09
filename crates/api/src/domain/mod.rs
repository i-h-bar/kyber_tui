use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use std::env;

pub mod auth;
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
}

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub(crate) fn new(cache: C, pw_store: PW) -> Self {
        Self { cache, pw_store }
    }
}
