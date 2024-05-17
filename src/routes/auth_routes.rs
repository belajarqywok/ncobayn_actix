use actix_web::{
    web::{post, resource, ServiceConfig}
};

use crate::controllers::auth_controllers::{
    login_controller::login_controller,
    register_controller::register_controller,
    refresh_token_controller::refresh_token_controller
};

/**
 *  Auth Routes Scope
 */
pub fn auth_routes_scope(service_config_routes: &mut ServiceConfig) {
    service_config_routes
        // Register Route
        .service(
            resource("/register")
                .route(post().to(register_controller))
        )

        // Login Route
        .service(
            resource("/login")
                .route(post().to(login_controller))
        )

        // Refresh Token Route
        .service(
            resource("/refresh")
                .route(post().to(refresh_token_controller))
        );
}
