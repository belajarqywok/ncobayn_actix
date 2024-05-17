use serde::Serialize;

// Register Response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub http_code: i32
}


// Login Response
#[derive(Debug, Serialize)]
pub struct LoginResponseData {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub http_code: i32,
    pub data: LoginResponseData
}

#[derive(Debug, Serialize)]
pub struct LoginResponseFailed{
    pub message: String,
    pub http_code: i32
}