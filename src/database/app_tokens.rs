use serde::Serialize;
use sqlx::FromRow;

use super::{
    apps::{self, AppGetError},
    Database,
};

#[derive(FromRow, Serialize)]
pub struct AppTokens {
    pub token: String,
    pub app_id: i32,
    pub permit_cors: bool,
    pub subapp_name: Option<String>,
}

pub enum AppCreateError {
    AppNotFound,
    GenericError(sqlx::Error),
}

impl From<AppGetError> for AppCreateError {
    fn from(err: AppGetError) -> Self {
        match err {
            AppGetError::NotFound => AppCreateError::AppNotFound,
            AppGetError::Generic(err) => AppCreateError::GenericError(err),
        }
    }
}

pub async fn create(
    connection: &Database,
    token: AppTokens,
    owner: &str,
) -> Result<AppTokens, AppCreateError> {
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
    .map_err(AppCreateError::GenericError)?;

    Ok(token)
}
