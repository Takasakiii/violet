mod apps;
mod auth;
mod errors;
mod test;

use actix_web::{dev::HttpServiceFactory, web, Scope};

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(auth::sing_up)
        .service(auth::login)
}

pub fn apps_routes() -> impl HttpServiceFactory {
    web::scope("/apps")
        .service(apps::create)
        .service(apps::list)
        .service(apps::update)
        .service(apps::tokens::create)
        .service(apps::tokens::list)
        .service(apps::errors::list)
}

pub fn errors_routes() -> impl HttpServiceFactory {
    web::scope("/errors").service(errors::create)
}

pub fn test_routes() -> impl HttpServiceFactory {
    web::scope("/test").service(test::index)
}
