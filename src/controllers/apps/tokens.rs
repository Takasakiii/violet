use actix_web::{
    post,
    web::{Data, Json, Path, ReqData},
    HttpResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        app_tokens::{self, AppCreateError, AppTokens},
        Database,
    },
    jwt::JwtClaims,
    tokens,
};

#[derive(Deserialize, Validate)]
pub struct AppTokensDto {
    permit_cors: bool,
    #[validate(length(max = 255, message = "Subapp name must be less than 255 characters"))]
    subapp_name: Option<String>,
}

#[post("/{id}/tokens")]
pub async fn create(
    path: Path<(i32,)>,
    owner: ReqData<JwtClaims>,
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
        Err(AppCreateError::AppNotFound) => HttpResponse::Unauthorized().finish(),
        Err(AppCreateError::GenericError(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
