use actix_web::web;
use actix_web::{middleware::from_fn, web::service};

use super::{handlers, middlewares};

// This is in charge of every path in /user path
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            // this wrap sets middleware for user authentication. the .service() after this line will be affected by this middleware
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(handlers::user_handlers::user), // .service(handlers::user_handlers::update_user),
    );
}
