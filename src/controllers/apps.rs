use actix_web::{
    post,
    web::{Data, Json, ReqData},
    HttpResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        apps::{self, AppsDto},
        Database,
    },
    jwt::JwtClaims,
};

#[derive(Deserialize, Validate)]
pub struct AppDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
}

#[post("")]
pub async fn create(
    user: ReqData<JwtClaims>,
    database: Data<Database>,
    app: Json<AppDto>,
) -> HttpResponse {
    let app = app.into_inner();

    if let Err(e) = app.validate() {
        return HttpResponse::UnprocessableEntity().json(e);
    }

    let database_dto = AppsDto {
        name: app.name,
        owner: user.username.clone(),
    };

    match apps::create(&*database, database_dto).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
