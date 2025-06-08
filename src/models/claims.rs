use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenClaims {
    pub username: String,
    pub user_id: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub user_id: String,
    pub exp: usize,
    pub is_remember: bool,
}