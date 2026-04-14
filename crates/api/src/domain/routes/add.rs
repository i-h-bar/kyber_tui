use contracts::{GenericRequest, GenericResponse};
use crate::domain::Application;
use crate::domain::errors::routes::DomainError;
use crate::ports::services::cache::Cache;
use crate::ports::services::pw_store::PWStore;

impl<C, PW> Application<C, PW>
where
    C: Cache + Send + Sync,
    PW: PWStore + Send + Sync,
{
    pub async fn add_credential(
        &self,
        request: GenericRequest,
    ) -> Result<GenericResponse, DomainError> {
        todo!()
    }
}