use crate::domain::errors::routes::DomainError;
use http::StatusCode;

pub fn map_domain_error(error: &DomainError) -> StatusCode {
    match error {
        DomainError::Decryption(_) | DomainError::Permission(_) => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
