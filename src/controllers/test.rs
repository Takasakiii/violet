use actix_web::{get, HttpResponse};

use super::authentication_extractor::UserAuthentication;

#[get("")]
pub async fn index(auth: UserAuthentication) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", auth.username))
}
