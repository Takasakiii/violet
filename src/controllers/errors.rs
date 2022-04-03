use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
    HttpResponse,
};
use serde::Deserialize;

use validator::Validate;

use crate::database::{
    errors::{self, ErrorsCreateDto, ErrorsErros},
    Database,
};

use super::errors_authentication_middleware::ErrorAuthenticationData;

#[derive(Validate, Deserialize)]
pub struct ErrorsDto {
    #[validate(length(min = 1, max = 20))]
    error_level: String,
    #[validate(length(min = 1, max = 65535))]
    message: String,
    #[validate(length(max = 65535))]
    stack_trace: Option<String>,
}

// #[post("")]
pub async fn create(
    database: Data<Database>,
    token: ReqData<ErrorAuthenticationData>,
    data: Json<ErrorsDto>,
) -> HttpResponse {
    let errors_dto = data.into_inner();
    if let Err(validation_errors) = errors_dto.validate() {
        return HttpResponse::UnprocessableEntity().json(validation_errors);
    }

    let app_token = token.into_inner().token;

    let data = ErrorsCreateDto {
        message: errors_dto.message,
        error_level: errors_dto.error_level,
        stack_trace: errors_dto.stack_trace,
        token: app_token,
    };

    let error_response = errors::create(&*database, data).await;

    match error_response {
        Ok(error) => HttpResponse::Created().json(error),
        Err(ErrorsErros::Generic(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
        Err(ErrorsErros::Unauthorized) => unreachable!(),
        Err(ErrorsErros::NotFound) => unreachable!(),
    }
}

// #[get("")]
pub async fn list() -> HttpResponse {
    HttpResponse::Ok().finish()
}
