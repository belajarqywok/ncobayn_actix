use serde::Deserialize;

/**
 *  Register Request
 */
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nickname: String,
    pub email:    String,
    pub password: String,
}

/**
 *  Login Request
 */
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/**
 *  Refresh Token Request
 */
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}