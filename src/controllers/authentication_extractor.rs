use std::pin::Pin;

use actix_web::{web::Data, FromRequest};
use futures::Future;
use serde::Deserialize;

use crate::{
    database::{users, Database},
    jwt::{Jwt, JwtClaims},
};

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub username: String,
}

impl FromRequest for UserAuthentication {
    type Error = actix_web::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let sync_result = || {
            let header = req.headers().get("Authorization").ok_or_else(|| {
                actix_web::error::ErrorUnauthorized("Missing Authorization header")
            })?;

            let token = header
                .to_str()
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?
                .replace("Bearer ", "");

            let jwt = req.app_data::<Data<Jwt>>().unwrap();

            let claims = jwt.verify_jwt(&token).map_err(|err| {
                log::error!("Jwt verification error: {}", err);
                actix_web::error::ErrorUnauthorized("Invalid token")
            })?;

            Ok((token, claims)) as Result<(String, JwtClaims), actix_web::Error>
        };

        let token_result = sync_result();
        let connection = req.app_data::<Data<Database>>().unwrap().clone();

        Box::pin(async move {
            let (token, claims) = token_result?;
            let user_token_response =
                users::get_by_token(&*connection, token)
                    .await
                    .map_err(|err| {
                        log::error!("Error to get token in database, {}", err);
                        actix_web::error::ErrorUnauthorized("Invalid token")
                    })?;

            if user_token_response.username != claims.username {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
            }

            let authentication = UserAuthentication {
                username: user_token_response.username,
            };

            Ok(authentication)
        })
    }
}
