use crate::adapters::services::pw_store::postgres::psql::Postgres;
use crate::ports::services::pw_store::PWStore;

mod postgres;

pub async fn create_pw_store() -> impl PWStore {
    Postgres::create().await
}
