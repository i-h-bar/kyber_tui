use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Cache save error")]
    CacheError,
}
