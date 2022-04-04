use chrono::Utc;
use serde::Serialize;
use sql_builder::{bind::Bind, SqlBuilder};
use sqlx::FromRow;

use super::{
    app_tokens::{self, AppTokenError},
    apps::{self, AppGetError},
    Database,
};

pub enum ErrorsErros {
    Unauthorized,
    NotFound,
    QueryError,
    Generic(sqlx::Error),
}

impl From<sqlx::Error> for ErrorsErros {
    fn from(error: sqlx::Error) -> Self {
        ErrorsErros::Generic(error)
    }
}

impl From<AppGetError> for ErrorsErros {
    fn from(_: AppGetError) -> Self {
        ErrorsErros::NotFound
    }
}

impl From<AppTokenError> for ErrorsErros {
    fn from(e: AppTokenError) -> Self {
        match e {
            AppTokenError::AppNotFound => ErrorsErros::NotFound,
            AppTokenError::Unauthorized => ErrorsErros::Unauthorized,
            AppTokenError::GenericError(err) => ErrorsErros::Generic(err),
        }
    }
}

#[derive(FromRow, Serialize)]
pub struct Errors {
    pub id: i32,
    pub app_id: i32,
    pub error_level: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub created_at: u64,
}

#[derive(Serialize, FromRow)]
pub struct ErrorsWithSubAppName {
    pub id: i32,
    pub app_id: i32,
    pub error_level: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub created_at: u64,
    pub subapp_name: String,
}

pub struct ErrorsCreateDto {
    pub error_level: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub token: String,
}

pub async fn create(database: &Database, data: ErrorsCreateDto) -> Result<Errors, ErrorsErros> {
    let now = Utc::now().timestamp();

    let app_id = app_tokens::check_app_token(database, &data.token).await?;

    let result = sqlx::query("insert into errors (app_id, error_level, message, stack_trace, token, created_at) values (?, ?, ?, ?, ?, ?)")
        .bind(app_id)
        .bind(&data.error_level)
        .bind(&data.message)
        .bind(&data.stack_trace)
        .bind(&data.token)
        .bind(&now)
        .execute(database.get_pool())
        .await?;

    let error = Errors {
        id: result.last_insert_id() as i32,
        app_id,
        created_at: now as u64,
        error_level: data.error_level,
        message: data.message,
        stack_trace: data.stack_trace,
    };

    Ok(error)
}

#[derive(Default)]
pub struct ErrorsSearch<'a> {
    pub app_id: i32,
    pub owner: &'a str,
    pub subapp: Option<String>,
    pub error_level: Option<String>,
    pub message: Option<String>,
    pub created_at: Option<u64>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

pub async fn list<'a>(
    database: &Database,
    search: ErrorsSearch<'a>,
) -> Result<Vec<ErrorsWithSubAppName>, ErrorsErros> {
    apps::get(database, search.app_id, search.owner).await?;

    let limit = if let Some(per_page) = search.per_page {
        per_page
    } else {
        10
    };

    let mut query_builder = SqlBuilder::select_from("errors e");
    query_builder
        .field("e.id as id")
        .field("e.app_id as app_id")
        .field("e.error_level as error_level")
        .field("e.message as message")
        .field("e.stack_trace as stack_trace")
        .field("e.created_at as created_at")
        .field("t.subapp_name as subapp_name")
        .join("app_tokens t")
        .on("t.token = e.token")
        .and_where("e.app_id = ?".bind(&search.app_id))
        .limit(limit);

    if let Some(subapp) = search.subapp {
        query_builder.and_where("t.subapp_name = ?".bind(&subapp));
    }

    if let Some(error_level) = search.error_level {
        query_builder.and_where("e.error_level = ?".bind(&error_level));
    }

    if let Some(message) = search.message {
        query_builder.and_where("e.message like ?".bind(&format!("%{}%", message)));
    }

    if let Some(created_at) = search.created_at {
        query_builder.and_where("e.created_at >= ?".bind(&created_at));
    }

    if let Some(page) = search.page {
        query_builder.offset(page * limit);
    }

    let final_query = query_builder.sql().map_err(|err| {
        log::error!("{}", err);
        ErrorsErros::QueryError
    })?;

    let result = sqlx::query_as(&final_query)
        .fetch_all(database.get_pool())
        .await?;

    Ok(result)
}
