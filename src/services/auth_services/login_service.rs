use sha2::{Digest, Sha512};

use actix_web::{
    web::{Data, Json}
};

use crate::{
    configurations::auth::{
        generate_access_token,
        generate_refresh_token
    },
    schemas::{
        requests::auth_requests::LoginRequest,
        responses::auth_responses::{
            LoginResponse,
            LoginResponseData
        }
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};


/**
 *  Login Service
 */
pub async fn login_service(
    repo: Data<AuthRepositories>,
    user_data: Json<LoginRequest>
) -> LoginResponse {

    let password = user_data.password
        .to_owned();

    let mut hasher = Sha512::new();
    hasher.update(password);

    let hashed_password = format!("{:x}", hasher.finalize());

    // User Data
    let data = LoginRequest {
        // Email
        email: user_data.email
            .to_owned(),

        // Password
        password: hashed_password
            .to_owned()
    };

    // Login Repository
    let login_repository = repo
        .login_repository(data).await;


    match login_repository {
        Ok(repository_message) => {
            if repository_message == "0".to_string() {
                return LoginResponse {
                    message: "Email or Password is wrong".to_string(),
                    http_code: 401,
                    data: LoginResponseData { 
                        access_token: "".to_string(),
                        refresh_token: "".to_string() 
                    }
                };
            }

            let gen_access_token = generate_access_token(
                &repository_message.as_str()
            ).await.unwrap();

            let gen_refresh_token = generate_refresh_token(
                &repository_message.as_str()
            ).await.unwrap();

            return LoginResponse {
                message: "Login Success".to_string(),
                http_code: 200,
                data: LoginResponseData { 
                    access_token: gen_access_token,
                    refresh_token: gen_refresh_token
                }
            };
        }
        Err(err) => {
            println!("{:?}", err);
            return LoginResponse {
                message: "Internal Server Error".to_string(),
                http_code: 500,
                data: LoginResponseData { 
                    access_token: "".to_string(),
                    refresh_token: "".to_string() 
                }
            };
        }
    };
}
