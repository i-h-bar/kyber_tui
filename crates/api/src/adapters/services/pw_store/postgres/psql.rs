use crate::adapters::services::pw_store::postgres::queries::auth::GET_AUTH_CREDENTIALS;
use crate::adapters::services::pw_store::postgres::queries::create_user::CREATE_USER;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::pw_store::{AuthCredentials, CreateUser, PWStore};
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Row};
use std::env;
use std::time::Instant;
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

    async fn create_user(&self, user_info: CreateUser) -> Result<Uuid, DomainError> {
        match sqlx::query(CREATE_USER)
            .bind(user_info.id)
            .bind(user_info.username)
            .bind(user_info.hashed_pw)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row.get("id")),
            Err(_) => Err(DomainError::Generic("Unable to create user".to_string())),
        }
    }

    async fn get_auth_credentials(&self, username: &str) -> Result<AuthCredentials, DomainError> {
        let start = Instant::now();
        let result = sqlx::query_as::<_, AuthCredentials>(GET_AUTH_CREDENTIALS)
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| {
                log::warn!("Failed to fetch auth credentials {error}");
                DomainError::Generic("Unable to fetch auth credentials".to_string())
            });

        log::info!("Fetching auth credentials took {:?}", start.elapsed());
        result
    }
}
