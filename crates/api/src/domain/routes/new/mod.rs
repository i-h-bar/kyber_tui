use contracts::{GenericRequest, GenericResponse};
use crate::domain::Application;
use crate::domain::errors::routes::new::NewUserError;
use crate::ports::services::cache::Cache;

impl<C> Application<C>
where
    C: Cache + Send + Sync,
{
    pub async fn create_user(&self, request: GenericRequest) -> Result<GenericResponse, NewUserError> {
        todo!()
    }
}
    
