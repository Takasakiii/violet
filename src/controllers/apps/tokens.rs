use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        app_tokens::{self, AppTokenError, AppTokens},
        Database,
    },
    extractors::UserAuthentication,
    tokens,
};

#[derive(Deserialize, Validate)]
pub struct AppTokensDto {
    permit_cors: bool,
    #[validate(length(max = 255, message = "Subapp name must be less than 255 characters"))]
    subapp_name: Option<String>,
}

#[derive(Deserialize)]
pub struct AppTokensListQuery {
    subapp: Option<String>,
}

#[post("/{id}/tokens")]
pub async fn create(
    path: Path<(i32,)>,
    owner: UserAuthentication,
    form: Json<AppTokensDto>,
    database: Data<Database>,
) -> HttpResponse {
    let form = form.into_inner();

    if let Err(err) = form.validate() {
        return HttpResponse::BadRequest().json(err);
    }

    let token = match tokens::create_token() {
        Ok(token) => token,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let app_token_db_dto = AppTokens {
        token,
        app_id: path.0,
        permit_cors: form.permit_cors,
        subapp_name: form.subapp_name,
    };

    match app_tokens::create(&*database, app_token_db_dto, &owner.username).await {
        Ok(token) => HttpResponse::Created().json(token),
        Err(AppTokenError::AppNotFound) => HttpResponse::Unauthorized().finish(),
        Err(AppTokenError::GenericError(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
        Err(_) => unreachable!(),
    }
}

#[get("/{id}/tokens")]
pub async fn list(
    path: Path<(i32,)>,
    auth: UserAuthentication,
    database: Data<Database>,
    query: Query<AppTokensListQuery>,
) -> HttpResponse {
    let app_id = path.0;

    match app_tokens::list(&*database, app_id, &auth.username, query.subapp.as_ref()).await {
        Ok(tokens) => HttpResponse::Ok().json(tokens),
        Err(AppTokenError::AppNotFound) => HttpResponse::NotFound().finish(),
        Err(AppTokenError::GenericError(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
        Err(_) => unreachable!(),
    }
}
