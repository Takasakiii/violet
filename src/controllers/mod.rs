use actix_web::{dev::HttpServiceFactory, web, Scope};

use crate::authentication_middleware::Authentication;

mod apps;
mod auth;

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(auth::sing_up)
        .service(auth::login)
}

pub fn apps_routes() -> impl HttpServiceFactory {
    web::scope("/apps")
        .wrap(Authentication)
        .service(apps::create)
        .service(apps::list)
        .service(apps::update)
}
