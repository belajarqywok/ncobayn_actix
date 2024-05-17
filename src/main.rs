mod api;
mod request;
mod response;
mod repository;


mod models;
mod routes;
mod schemas;
mod services;
mod validations;
mod controllers;
mod repositories;
mod configurations;

use actix_web::{
    web, 
    App,
    HttpServer,
    web::Data,
    middleware::Logger
};

use api::user_api::{
    create_user, 
    delete_user,
    get_all_users,
    get_user,
    update_user,

    // login,
    // refresh_token,
    restricted_area
};


use repository::mongodb_repo::MongoRepo;
use repositories::{
    auth_repositories::auth_repos::AuthRepositories,
    profile_repositories::profile_repos::ProfileRepositories
};
use env_logger::Env;


use routes::auth_routes::auth_routes_scope;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let db: MongoRepo = MongoRepo::init().await;
    // let db_data: Data<MongoRepo> = Data::new(db);

    let mongo_repo: Data<MongoRepo> = Data::new(
        MongoRepo::init().await
    );

    // Auth Repository
    let auth_repo: Data<AuthRepositories> = Data::new(
        AuthRepositories::init().await
    );

    // Profile Repository
    let profile_repo: Data<ProfileRepositories> = Data::new(
        ProfileRepositories::init().await
    );

    let env: Env<'_> = Env::default().filter_or("MY_APP_LOG_LEVEL", "info");
    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    HttpServer::new(move || {
        // Auth Routes Scope
        // let auth_scope: Scope = actix_web::web::scope("/v1")
        //     // Login Route
        //     .service(
        //         web::resource("/login")
        //             .route(web::post().to(login))
        //     )

        //     // Refresh Token Route
        //     .service(
        //         web::resource("/refresh")
        //             .route(web::post().to(refresh_token))
        //     )

        // Auth Route Scope
        // let auth_route_scope: Scope = auth_routes_scope();

        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Logger::default())

            // Repositories
            .app_data(mongo_repo.clone())
            .app_data(auth_repo.clone())
            .app_data(profile_repo.clone())

            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)

            .service(
                web::resource("/restricted")
                    .route(web::get().to(restricted_area))
            )

            .service(
                web::scope("/v1")
                    .configure(auth_routes_scope)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


// use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use serde::{Deserialize, Serialize};
// use std::time::{Duration, SystemTime};


// // Secret key for JWT
// const SECRET_KEY: &[u8] = b"secret_key";

// // User credentials hardcoded for simplicity (in a real application, these should be stored securely)
// const USERNAME: &str = "admin";
// const PASSWORD: &str = "password";

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }

// #[derive(Debug, Deserialize)]
// struct LoginRequest {
//     username: String,
//     password: String,
// }

// #[derive(Debug, Serialize)]
// struct LoginResponse {
//     token: String,
// }

// fn generate_token(username: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let expiration = SystemTime::now() + Duration::from_secs(3600); // Token expires in 1 hour

//     let claims = Claims {
//         sub: username.to_owned(),
//         exp: expiration.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize,
//     };

//     let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))?;
//     Ok(token)
// }

// fn decode_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
//     let decoding_key = DecodingKey::from_secret(SECRET_KEY);
//     let validation = Validation::default();

//     let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
//     Ok(decoded_token.claims)
// }

// async fn login(request: web::Json<LoginRequest>) -> impl Responder {
//     if request.username == USERNAME && request.password == PASSWORD {
//         match generate_token(&request.username) {
//             Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
//             Err(_) => HttpResponse::InternalServerError().finish(),
//         }
//     } else {
//         HttpResponse::Unauthorized().finish()
//     }
// }

// async fn restricted_area(request: HttpRequest) -> impl Responder {
//     let authorization_header = request.headers().get("Authorization");

//     match authorization_header {
//         Some(header_value) => {
//             let token = header_value.to_str().unwrap().trim_start_matches("Bearer ");
//             match decode_token(token) {
//                 Ok(claims) => HttpResponse::Ok().body(format!("Welcome, {}", claims.sub)),
//                 Err(_) => HttpResponse::Unauthorized().finish(),
//             }
//         }
//         None => HttpResponse::Unauthorized().finish(),
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(web::resource("/login").route(web::post().to(login)))
//             .service(web::resource("/restricted").to(restricted_area))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }