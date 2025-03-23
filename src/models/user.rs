use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub is_active: bool,
}
