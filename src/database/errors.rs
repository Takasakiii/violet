use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;

use super::{
    app_tokens::{self, AppTokenError},
    apps::AppGetError,
    Database,
};

pub enum ErrorsErros {
    Unauthorized,
    NotFound,
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
    pub stack_trace: String,
    pub created_at: u64,
}

pub struct ErrorsCreateDto {
    app_id: i32,
    error_level: String,
    message: String,
    stack_trace: String,
    token: String,
}

pub async fn create(database: &Database, data: ErrorsCreateDto) -> Result<Errors, ErrorsErros> {
    let now = Utc::now().timestamp();

    app_tokens::check_app_token(database, &data.token, data.app_id).await?;

    let result = sqlx::query("insert into errors (app_id, error_level, message, stack_trace, token, created_at) values (?, ?, ?, ?, ?, ?)")
        .bind(&data.app_id)
        .bind(&data.error_level)
        .bind(&data.message)
        .bind(&data.stack_trace)
        .bind(&data.token)
        .bind(&now)
        .execute(database.get_pool())
        .await?;

    let error = Errors {
        id: result.last_insert_id() as i32,
        app_id: data.app_id,
        created_at: now as u64,
        error_level: data.error_level,
        message: data.message,
        stack_trace: data.stack_trace,
    };

    Ok(error)
}
