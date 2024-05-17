use crate::{
    models::user_model::User,

    response::{ 
        error_response::Error, 
        login_response::LoginResponse,
    },

    repository::mongodb_repo::MongoRepo,

    request::auth_request::{
        LoginRequest,
        RefreshTokenRequest
    },

    configurations::auth::{
        generate_access_token,
        generate_refresh_token,

        decode_refresh_token,
        decode_access_token,
    },

    validations::auth_validation::{
        TokenValidationStruct,
        access_token_validation
    }
};


use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpRequest,
    HttpResponse,
    http::header::HeaderValue
};

use mongodb::bson::oid::ObjectId;



#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user_detail = db.create_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().json(Error{ 
            message: "ID not found".to_string()
        });
    }

    let get_user = db.get_user(&id).await;

    match get_user {

        Ok(user) => {
            if user.is_none() {
                return HttpResponse::NotFound().json(
                    Error {
                        message: "ID not found".to_string()
                    }
                );

            } else {
                return HttpResponse::Ok()
                    .json(
                        user.unwrap()
                    );
            }
        }

        Err(_err) => {
            HttpResponse::InternalServerError().json(
                Error {
                    message: "Internal Server Error".to_string()
                }
            )
        }
    }
}


#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {

    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().json(
            Error { message: "ID not found".to_string() }
        );
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;

                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {

    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    // Get All User
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}


// Login Controller
pub async fn login(request: Json<LoginRequest>) -> HttpResponse {

    if request.username == "admin".to_string() && request.password == "admin".to_string() {

        let access:  String  = generate_access_token(&request.username).await.unwrap();
        let refresh: String  = generate_refresh_token(&request.username).await.unwrap();

        HttpResponse::Ok().json(
            LoginResponse {
                access_token: access,
                refresh_token: refresh
            }
        )

    } else {
        HttpResponse::Unauthorized().finish()
    }
}



// Refresh Token Controller
pub async fn refresh_token(request: Json<RefreshTokenRequest>) -> HttpResponse {

    let refresh_token: &String = &request.refresh_token;

    match decode_refresh_token(refresh_token).await {
        Ok(claims) => {

            let access:  String  = generate_access_token(&claims.sub).await.unwrap();
            let refresh: String  = generate_refresh_token(&claims.sub).await.unwrap();

            HttpResponse::Ok().json(
                LoginResponse {
                    access_token: access,
                    refresh_token: refresh
                }
            )
        },

        Err(_) => HttpResponse::Unauthorized().json(
            Error {
                message: "Unauthorized".to_string()
            }
        ),
    }
}



// #[get("/restricted")]
pub async fn restricted_area(request: HttpRequest) -> HttpResponse {

    let authorization_header: Option<&HeaderValue> = request.headers().get("Authorization");

    let token_is_valid: TokenValidationStruct = access_token_validation(authorization_header).await;

    if !token_is_valid.response {
        return HttpResponse::Unauthorized().json(
            Error {
                message: "Unauthorized".to_string()
            }
        );
    }

    return HttpResponse::Unauthorized().json(
        Error {
            message: format!("Selamat Datang {}", token_is_valid.user_id)
        }
    );

    // match authorization_header {
    //     Some(header_value) => {
    //         let token = header_value.to_str().unwrap().trim_start_matches("Bearer ");

    //         match decode_access_token(token).await {
    //             Ok(claims) => HttpResponse::Ok().body(format!("Welcome, {}", claims.sub)),

    //             Err(_) => HttpResponse::Unauthorized().json(
    //                 Error {
    //                     message: "Unauthorized".to_string()
    //                 }
    //             ),
    //         }
    //     }

    //     None => HttpResponse::Unauthorized().json(
    //         Error {
    //             message: "Unauthorized".to_string()
    //         }
    //     ),
    // }
}
