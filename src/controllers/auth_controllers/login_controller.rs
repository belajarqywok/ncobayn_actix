use actix_web::{
    HttpResponse,
    web::{Data, Json}
};

use crate::{
    services::auth_services::login_service::login_service,
    schemas::{
        requests::auth_requests::LoginRequest,
        responses::auth_responses::{
            LoginResponse,
            LoginResponseFailed
        }
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};


/**
 *  Login Controller
 */
pub async fn login_controller(
    repo: Data<AuthRepositories>,
    request_body: Json<LoginRequest>
) -> HttpResponse {

    let service: LoginResponse = login_service(
        repo, request_body
    ).await;

    if service.http_code != 200 {
        return HttpResponse::Unauthorized().json(
            LoginResponseFailed {
                message: service.message,
                http_code: service.http_code
            }
        )
    }

    return HttpResponse::Ok().json(service);
}
