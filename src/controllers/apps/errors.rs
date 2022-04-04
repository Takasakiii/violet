use actix_web::{
    get,
    web::{Data, Path, Query, ReqData},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    database::{
        errors::{self, ErrorsErros, ErrorsSearch},
        Database,
    },
    jwt::JwtClaims,
};

#[derive(Clone, Deserialize)]
pub struct ErrorsListQuery {
    pub subapp: Option<String>,
    pub error_level: Option<String>,
    pub message: Option<String>,
    pub created_at: Option<u64>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[get("/{id}/errors")]
pub async fn list(
    path: Path<(i32,)>,
    database: Data<Database>,
    user: ReqData<JwtClaims>,
    query: Query<ErrorsListQuery>,
) -> HttpResponse {
    let app_id = path.into_inner().0;
    let query = query.into_inner();

    let database_query = ErrorsSearch {
        app_id,
        owner: &user.username,
        created_at: query.created_at,
        error_level: query.error_level,
        message: query.message,
        page: query.page,
        per_page: query.per_page,
        subapp: query.subapp,
    };

    let errors = errors::list(&*database, database_query).await;

    match errors {
        Ok(errors) => HttpResponse::Ok().json(errors),
        Err(ErrorsErros::Generic(err)) => {
            log::error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
        Err(ErrorsErros::NotFound) => HttpResponse::NotFound().finish(),
        Err(ErrorsErros::Unauthorized) => HttpResponse::Unauthorized().finish(),
        Err(ErrorsErros::QueryError) => HttpResponse::NotAcceptable().finish(),
    }
}
