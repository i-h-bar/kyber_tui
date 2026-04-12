use crate::api::{ApiError, ApiSession};

impl ApiSession {
    pub async fn authenticate(&mut self, username: String, password: String) -> Result<bool, ApiError> {
        todo!()
    }
}