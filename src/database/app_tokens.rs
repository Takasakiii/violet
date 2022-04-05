use serde::Serialize;
use sqlx::FromRow;

use super::{
    apps::{self, AppGetError},
    Database,
};

#[derive(FromRow, Serialize, Clone)]
pub struct AppTokens {
    pub token: String,
    pub app_id: i32,
    pub permit_cors: bool,
    pub subapp_name: Option<String>,
}

pub enum AppTokenError {
    AppNotFound,
    Unauthorized,
    GenericError(sqlx::Error),
}

impl From<AppGetError> for AppTokenError {
    fn from(err: AppGetError) -> Self {
        match err {
            AppGetError::NotFound => AppTokenError::AppNotFound,
            AppGetError::Generic(err) => AppTokenError::GenericError(err),
        }
    }
}

impl From<sqlx::Error> for AppTokenError {
    fn from(err: sqlx::Error) -> Self {
        AppTokenError::GenericError(err)
    }
}

pub async fn create(
    connection: &Database,
    token: AppTokens,
    owner: &str,
) -> Result<AppTokens, AppTokenError> {
    apps::get(connection, token.app_id, owner).await?;

    sqlx::query(
        "insert into app_tokens (token, app_id, permit_cors, subapp_name) values (?, ?, ?, ?)",
    )
    .bind(&token.token)
    .bind(&token.app_id)
    .bind(&token.permit_cors)
    .bind(&token.subapp_name)
    .execute(connection.get_pool())
    .await
    .map_err(AppTokenError::GenericError)?;

    Ok(token)
}

fn cut_token(token: &str) -> String {
    format!("{}...", token.split_at(5).0)
}

pub async fn list(
    connection: &Database,
    app: i32,
    owner: &str,
    subapp_name: Option<&String>,
) -> Result<Vec<AppTokens>, AppTokenError> {
    apps::get(connection, app, owner).await?;

    let mut tokens: Vec<AppTokens> = if let Some(subapp_name) = subapp_name {
        sqlx::query_as("select t.* from app_tokens t join apps a on t.app_id = a.id where t.app_id = ? and a.owner = ? and t.subapp_name like ?")
        .bind(app)    
        .bind(owner)
            .bind(format!("%{}%", subapp_name))
            .fetch_all(connection.get_pool())
            .await?
    } else {
        sqlx::query_as(
            "select t.* from app_tokens t join apps a on a.id = t.app_id where a.owner = ? and t.app_id = ?",
        )
        .bind(owner)
        .bind(app)
        .fetch_all(connection.get_pool())
        .await?
    };

    tokens
        .iter_mut()
        .for_each(|t| t.token = cut_token(&t.token));

    Ok(tokens)
}

pub async fn check_app_token(database: &Database, token: &str) -> Result<i32, AppTokenError> {
    let token_result: (i32,) = sqlx::query_as("select  app_id from app_tokens where token = ?")
        .bind(token)
        .fetch_optional(database.get_pool())
        .await?
        .ok_or(AppTokenError::Unauthorized)?;

    Ok(token_result.0)
}

pub async fn get_by_token(database: &Database, token: &str) -> Result<AppTokens, AppTokenError> {
    let token = sqlx::query_as("select * from app_tokens where token = ?")
        .bind(token)
        .fetch_optional(database.get_pool())
        .await?
        .ok_or(AppTokenError::Unauthorized)?;
    Ok(token)
}
