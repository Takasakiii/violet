use std::{fmt::Display, pin::Pin};

use actix_web::{
    http::StatusCode, web::Data, FromRequest, HttpResponse, HttpResponseBuilder, ResponseError,
};
use futures::Future;
use serde::Deserialize;

use crate::{
    database::{
        app_tokens::{self, AppTokenError},
        users, Database,
    },
    jwt::{Jwt, JwtClaims},
};

#[derive(Debug)]
pub struct ExtractorError<'a> {
    status: StatusCode,
    message: &'a str,
}

impl<'a> Display for ExtractorError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> ResponseError for ExtractorError<'a> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponseBuilder::new(self.status).json(serde_json::json!({ "message": self.message }))
    }
}

impl<'a> ExtractorError<'a> {
    fn new(status: StatusCode, message: &'a str) -> Self {
        Self { status, message }
    }
}

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub username: String,
}

impl FromRequest for UserAuthentication {
    type Error = ExtractorError<'static>;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let sync_result = || {
            let header = req.headers().get("Authorization").ok_or_else(|| {
                ExtractorError::new(StatusCode::UNAUTHORIZED, "Missing authorization header")
            })?;

            let token = header
                .to_str()
                .map_err(|_| ExtractorError::new(StatusCode::UNAUTHORIZED, "Invalid token format"))?
                .replace("Bearer ", "");

            let jwt = req.app_data::<Data<Jwt>>().unwrap();

            let claims = jwt.verify_jwt(&token).map_err(|err| {
                log::error!("Jwt verification error: {}", err);
                ExtractorError::new(StatusCode::UNAUTHORIZED, "Jwt verification error")
            })?;

            Ok((token, claims)) as Result<(String, JwtClaims), ExtractorError>
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
                        ExtractorError::new(
                            StatusCode::UNAUTHORIZED,
                            "Error to get token in database",
                        )
                    })?;

            if user_token_response.username != claims.username {
                return Err(ExtractorError::new(
                    StatusCode::UNAUTHORIZED,
                    "Invalid token",
                ));
            }

            let authentication = UserAuthentication {
                username: user_token_response.username,
            };

            Ok(authentication)
        })
    }
}

pub struct ErrorAuthentication {
    pub app_id: i32,
    pub token: String,
}

impl FromRequest for ErrorAuthentication {
    type Error = ExtractorError<'static>;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = || {
            let token = req
                .headers()
                .get("Authorization")
                .ok_or_else(|| {
                    ExtractorError::new(StatusCode::UNAUTHORIZED, "Missing authorization header")
                })?
                .to_str()
                .map_err(|err| {
                    log::error!("Invalid token format, {}", err);
                    ExtractorError::new(StatusCode::UNAUTHORIZED, "Invalid token format")
                })?
                .replace("Bearer ", "");
            Ok(token) as Result<String, ExtractorError>
        };

        let cors_header = req
            .headers()
            .get("Sec-Fetch-Mode")
            .map(|header| !header.is_empty());

        let token_result = token();
        let connection = req.app_data::<Data<Database>>().unwrap().clone();

        Box::pin(async move {
            let token = token_result?;
            let token_db = app_tokens::get_by_token(&*connection, &token)
                .await
                .map_err(|err| match err {
                    AppTokenError::Unauthorized => {
                        ExtractorError::new(StatusCode::UNAUTHORIZED, "Invalid token")
                    }
                    AppTokenError::GenericError(err) => {
                        log::error!("{}", err);
                        ExtractorError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
                    }
                    AppTokenError::AppNotFound => unreachable!(),
                })?;

            if cors_header.is_some() && cors_header.unwrap() && !token_db.permit_cors {
                return Err(ExtractorError::new(
                    StatusCode::UNAUTHORIZED,
                    "Token not permit cors",
                ));
            }

            let authentication = ErrorAuthentication {
                app_id: token_db.app_id,
                token,
            };

            Ok(authentication)
        })
    }
}
