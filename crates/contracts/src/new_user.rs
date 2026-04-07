use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub hashed_pw: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserResponse {
    pub success: bool,
}