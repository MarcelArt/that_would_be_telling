use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::models::role::Role;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub username: String,
    pub email: String,
    // pub password: String,
    pub roles: Option<Vec<RecordId>>,
    pub roles_detail: Option<Vec<Role>>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub roles: Option<Vec<RecordId>>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
    pub is_remember: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshInput {
    pub refresh_token: String,
}