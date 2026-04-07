use crate::adapters::services::pw_store::postgres::queries::CREATE_USER;
use crate::ports::services::pw_store::{CreateUser, PWStore, PWStoreError};
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Row};
use std::env;
use uuid::Uuid;

pub struct Postgres {
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

    async fn create_user(&self, user_info: CreateUser) -> Result<Uuid, PWStoreError> {
        match sqlx::query(CREATE_USER)
            .bind(user_info.id)
            .bind(user_info.username)
            .bind(user_info.hashed_pw)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row.get::<Uuid, &str>("id")),
            Err(_) => Err(PWStoreError::UserCreationError),
        }
    }
}
