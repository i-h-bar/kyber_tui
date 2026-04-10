use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString, rand_core::OsRng}, PasswordHash, PasswordVerifier};
use std::time::Instant;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn create_pw_hash(&self, password_bytes: Vec<u8>) -> Result<String, DomainError> {
        let start = Instant::now();
        let hash = tokio::task::spawn_blocking(move || -> Result<String, DomainError> {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            Ok(argon2
                .hash_password(&password_bytes, &salt)
                .map_err(|error| {
                    log::warn!("Error hashing password {error}");
                    DomainError::Hashing("Error hashing password".to_string())
                })?
                .to_string())
        })
        .await
        .map_err(|error| {
            log::error!("Tokio error {error}");
            DomainError::Hashing("Tokio error".to_string())
        })?;
        log::info!("Hashing password took {:?}", start.elapsed());

        hash
    }

    pub async fn verify_pw_hash(&self, password: String, hash: String) -> Result<bool, DomainError> {
        let start = Instant::now();
        let result = tokio::task::spawn_blocking(move || -> Result<bool, DomainError> {
            Ok(Argon2::default().verify_password(
                &password.as_bytes(),
                &PasswordHash::new(&hash).map_err(|error| {
                    log::error!("Error creating password hash {error}");
                    DomainError::Hashing("Error creating password hash".to_string())
                })?
            ).is_ok())
        }).await.map_err(| error | {
            log::error!("Tokio error {error}");
            DomainError::Hashing("Error verifying password".to_string())
        })?;

        log::info!("Verifying password took {:?}", start.elapsed());

        result
    }
}
