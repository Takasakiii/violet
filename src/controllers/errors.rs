use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;

use validator::Validate;

use crate::{
    database::{
        errors::{self, ErrorsCreateDto, ErrorsErros},
        Database,
    },
    extractors::ErrorAuthentication,
};

#[derive(Validate, Deserialize)]
pub struct ErrorsDto {
    #[validate(length(min = 1, max = 20))]
    error_level: String,
    #[validate(length(min = 1, max = 65535))]
    message: String,
    #[validate(length(max = 65535))]
    stack_trace: Option<String>,
}

#[post("")]
pub async fn create(
    authentication: ErrorAuthentication,
    database: Data<Database>,
    data: Json<ErrorsDto>,
) -> HttpResponse {
    let errors_dto = data.into_inner();
    if let Err(validation_errors) = errors_dto.validate() {
        return HttpResponse::UnprocessableEntity().json(validation_errors);
    }

    let data = ErrorsCreateDto {
        message: errors_dto.message,
        error_level: errors_dto.error_level,
        stack_trace: errors_dto.stack_trace,
        app_id: authentication.app_id,
        token: authentication.token,
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
        Err(ErrorsErros::QueryError) => unreachable!(),
    }
}
