use chrono;

use sha2::{Digest, Sha512};

use actix_web::{
    web::{Data, Json}
};
use mongodb::bson::extjson::de::Error;

use crate::{
    models::user::User,
    schemas::{
        requests::auth_requests::RegisterRequest
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};

// Register Service
pub async fn register_service(
    repo: Data<AuthRepositories>,
    user_data: Json<RegisterRequest>
) -> Result<bool, Error> {

    let password = user_data.password
        .to_owned();

    let mut hasher = Sha512::new();
    hasher.update(password);

    let hashed_password = format!("{:x}", hasher.finalize());

    // User Data
    let data = User {
        id: None,

        // Nickname
        nickname: user_data.nickname
            .to_owned(),

        // Email
        email: user_data.email
            .to_owned(),

        // Password
        password: hashed_password
            .to_owned(),

        // Created At
        created_at: chrono::offset::Utc::now()
            .to_string()
            .to_owned(),
            
        // Updated At
        updated_at: chrono::offset::Utc::now()
            .to_string()
            .to_owned()
    };

    // Register Repository
    let register_repository = repo
        .register_repository(data).await;

    return register_repository;
}
