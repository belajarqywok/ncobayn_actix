use actix_web::{
    web::{Data, Json},
    HttpResponse
};

use crate::{
    services::auth_services::refresh_token_service::refresh_token_service,
    schemas::{
        requests::auth_requests::RefreshTokenRequest,
        responses::auth_responses::{
            LoginResponse,
            LoginResponseFailed
        }
    },
    validations::auth_validation::{
        TokenValidationStruct,
        refresh_token_validation
    },
    repositories::auth_repositories::auth_repos::AuthRepositories
};


/**
 *  Refresh Token Controller
 */
pub async fn refresh_token_controller(
    repo: Data<AuthRepositories>,
    request_body: Json<RefreshTokenRequest>
) -> HttpResponse {

    let token_is_valid: TokenValidationStruct = refresh_token_validation(
        request_body.refresh_token.as_str()
    ).await;

    if !token_is_valid.response {
        return HttpResponse::Unauthorized().json(
            LoginResponseFailed {
                message: "Unauthorized".to_string(),
                http_code: 401
            }
        )
    }

    let service: LoginResponse = refresh_token_service(
        repo, token_is_valid.user_id
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