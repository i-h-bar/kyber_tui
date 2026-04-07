use std::env;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use crate::ports::services::pw_store::{PWStore, PWStoreError};

struct Postgres {
    pool: Pool<sqlx::Postgres>,
}

#[async_trait]
impl PWStore for Postgres {
    async fn create() -> Self {
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER wasn't in env vars");
        let password = env::var("POSTGRES_PW").expect("POSTGRES_PW wasn't in env vars");
        let db = env::var("POSTGRES_DB").expect("POSTGRES_DB wasn't in env vars");
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost:5432".to_string());
        let uri = format!("postgresql://{user}:{password}@{host}/{db}");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .expect("Failed Postgres connection");

        Self { pool }
    }

    async fn create_user(&self) -> Result<(), PWStoreError> {
        todo!()
    }
}