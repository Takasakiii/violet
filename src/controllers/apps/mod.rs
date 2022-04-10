pub mod errors;
pub mod tokens;

use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        apps::{self, Apps, AppsDto, AppsUpdateError},
        Database,
    },
    extractors::UserAuthentication,
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
    user: UserAuthentication,
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

#[get("")]
pub async fn list(user: UserAuthentication, database: Data<Database>) -> HttpResponse {
    match apps::list(&*database, &user.username).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/{id}")]
pub async fn update(
    user: UserAuthentication,
    id: Path<(i32,)>,
    database: Data<Database>,
    app: Json<AppDto>,
) -> HttpResponse {
    let id = id.into_inner().0;
    let app = app.into_inner();

    if let Err(e) = app.validate() {
        return HttpResponse::UnprocessableEntity().json(e);
    }

    let app_dto = Apps {
        id,
        name: app.name,
        owner: user.username.clone(),
    };

    match apps::update(&*database, app_dto).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(AppsUpdateError::NotFound) => HttpResponse::NotFound().finish(),
        Err(AppsUpdateError::Generic(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
