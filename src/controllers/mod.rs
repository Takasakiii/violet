mod apps;
mod auth;
mod authentication_middleware;
mod errors;
mod errors_authentication_middleware;

use actix_web::{dev::HttpServiceFactory, guard, web, Scope};

use authentication_middleware::Authentication;

use self::errors_authentication_middleware::ErrorsAuthentication;

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
        .service(apps::tokens::create)
        .service(apps::tokens::list)
        .service(apps::errors::list)
}

pub fn errors_routes() -> impl HttpServiceFactory {
    web::scope("/errors").service(
        web::resource("")
            .guard(guard::Post())
            .wrap(ErrorsAuthentication)
            .route(web::post().to(errors::create)),
    )
    // .service(
    //     web::resource("")
    //         .wrap(Authentication)
    //         .route(web::get().to(errors::list)),
    // )
}
