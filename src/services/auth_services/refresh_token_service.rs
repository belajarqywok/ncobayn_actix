use actix_web::web::Data;

use crate::{
    configurations::auth::{
        generate_access_token,
        generate_refresh_token
    },
    schemas::responses::auth_responses::{
        LoginResponse,
        LoginResponseData
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};


/**
 *  Refresh Token Service
 */
pub async fn refresh_token_service(
    repo: Data<AuthRepositories>,
    user_id: String
) -> LoginResponse {
    // Refresh Token Repository
    let refresh_repository = repo
        .refresh_token_repository(&user_id).await;

    match refresh_repository {
        Ok(repository_message) => {
            if !repository_message {
                return LoginResponse {
                    message: "Token Invalid".to_string(),
                    http_code: 401,
                    data: LoginResponseData { 
                        access_token: "".to_string(),
                        refresh_token: "".to_string() 
                    }
                };
            }

            let gen_access_token = generate_access_token(
                &user_id.as_str()
            ).await.unwrap();

            let gen_refresh_token = generate_refresh_token(
                &user_id.as_str()
            ).await.unwrap();

            return LoginResponse {
                message: "Refresh Token Success".to_string(),
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