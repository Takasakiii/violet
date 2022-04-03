use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::database::{
    app_tokens::{self, AppTokenError, AppTokens},
    Database,
};

#[derive(Clone)]
pub struct ErrorAuthenticationData {
    pub token: String,
}

pub struct ErrorsAuthentication;

impl<S, B> Transform<S, ServiceRequest> for ErrorsAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = actix_web::Error;

    type Transform = ErrorsAuthenticationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorsAuthenticationMiddleware { service })
    }
}

pub struct ErrorsAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorsAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = actix_web::Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .map(|header| {
                header.to_str().map(|token| {
                    let token = token.to_owned();
                    req.extensions_mut().insert(ErrorAuthenticationData {
                        token: token.clone(),
                    });
                    token
                })
            })
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization Header"));

        let connection = req.app_data::<Data<Database>>().unwrap().clone();

        let process = async move {
            let token = token?.map_err(|err| {
                log::error!("{}", err);
                actix_web::error::ErrorUnauthorized("Invalid Authorization Header")
            })?;

            let token_response_db = app_tokens::get_by_token(&*connection, &token)
                .await
                .map_err(|err| match err {
                    AppTokenError::Unauthorized => {
                        actix_web::error::ErrorUnauthorized("Invalid Token")
                    }
                    AppTokenError::GenericError(err) => {
                        log::error!("{}", err);
                        actix_web::error::ErrorInternalServerError("Internal Server Error")
                    }
                    AppTokenError::AppNotFound => unreachable!(),
                })?;

            Ok(token_response_db) as Result<AppTokens, actix_web::Error>
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let token = process.await?;
            let res = fut.await?;

            let cors_header = res.request().headers().get("Sec-Fetch-Mode");

            if cors_header.is_some() && !token.permit_cors {
                return Err(actix_web::error::ErrorUnauthorized(
                    "CORS is not permitted for this token".to_owned(),
                ));
            }
            Ok(res)
        })
    }
}
