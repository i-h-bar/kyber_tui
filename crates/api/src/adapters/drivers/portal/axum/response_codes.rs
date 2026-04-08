use crate::domain::errors::routes::DomainError;
use http::StatusCode;

pub fn map_domain_error(error: DomainError) -> StatusCode {
    match error {
        DomainError::EncryptionError(_) | DomainError::PermissionError(_) => {
            StatusCode::UNAUTHORIZED
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
