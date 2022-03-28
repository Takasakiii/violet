use actix_web::{web, Scope};

mod auth;

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(auth::sing_up)
        .service(auth::login)
}
