use futures::future::{ok, LocalBoxFuture, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    HttpMessage,
};

use crate::{
    database::{users, Database},
    jwt::Jwt,
};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        let mut authentication_claims = None;
        let mut auth_token = None;
        let authorization_header = req.headers().get("Authorization");

        if let Some(header) = authorization_header {
            if let Ok(header) = header.to_str() {
                if header.starts_with("Bearer ") {
                    let jwt = req.app_data::<Data<Jwt>>().unwrap();
                    let token = header.replace("Bearer ", "");
                    match jwt.verify_jwt(&token) {
                        Ok(claims) => {
                            req.extensions_mut().insert(claims.clone());
                            authentication_claims = Some(claims);
                            auth_token = Some(token);
                        }
                        Err(err) => {
                            log::error!("{}", err);
                        }
                    }
                }
            }
        }

        let db = req.app_data::<Data<Database>>().unwrap().clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(token) = auth_token {
                let claims = authentication_claims.unwrap();
                match users::get_by_token(&*db, token).await {
                    Ok(user) => {
                        if user.username != claims.username {
                            return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                        }
                    }
                    Err(_) => {
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                    }
                }
            } else {
                return Err(actix_web::error::ErrorUnauthorized("No token provided"));
            }

            let res = fut.await?;

            Ok(res)
        })
    }
}
