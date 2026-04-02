use crate::ports::drivers::portal::Portal;
use crate::ports::services::cache::Cache;

pub mod routes;

pub struct Application<C>
where
    C: Cache + Send + Sync,
{
    cache: C,
}

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub(crate) fn new(cache: C) -> Self {
        Self { cache }
    }
}
