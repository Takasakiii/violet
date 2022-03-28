use actix_web::{get, web::ReqData, Responder};

use crate::jwt::JwtClaims;

#[get("")]
async fn index(user: ReqData<JwtClaims>) -> impl Responder {
    user.username.clone()
}
