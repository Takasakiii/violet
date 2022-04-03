use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, LocalBoxFuture, Ready};

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
        todo!()
    }
}
