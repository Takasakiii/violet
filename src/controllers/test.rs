use actix_web::{get, HttpResponse};

use crate::extractors::UserAuthentication;

#[get("")]
pub async fn index(auth: UserAuthentication) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", auth.username))
}
