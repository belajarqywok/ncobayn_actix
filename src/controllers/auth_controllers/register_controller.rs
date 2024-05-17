use actix_web::{
    HttpResponse,
    web::{Data, Json}
};

use mongodb::bson::extjson::de::Error;

use crate::{
    services::auth_services::register_service::register_service,
    schemas::{
        requests::auth_requests::RegisterRequest,
        responses::auth_responses::RegisterResponse
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};

// Register Controller
pub async fn register_controller(
    repo: Data<AuthRepositories>,
    user_data: Json<RegisterRequest>
) -> HttpResponse {

    let service: Result<bool, Error> = register_service(
        repo, user_data
    ).await;


    match service {
        Ok(message) => {
            if !message {
                return HttpResponse::BadRequest().json(RegisterResponse{
                    message: "Email Already".to_string(),
                    http_code: 400
                })
            }

            return HttpResponse::BadRequest().json(RegisterResponse{
                message: "Register Success".to_string(),
                http_code: 200
            })
        }

        Err(error) => {
            println!("{:}", error);
            return HttpResponse::BadRequest().json(RegisterResponse{
                message: "Internal Server Error".to_string(),
                http_code: 500
            })
        }
    }

}
