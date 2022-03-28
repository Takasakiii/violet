use serde::Serialize;
use sqlx::FromRow;

use super::Database;

pub struct AppsDto {
    pub name: String,
    pub owner: String,
}

#[derive(Serialize, FromRow)]
pub struct Apps {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

pub async fn create(connection: &Database, app: AppsDto) -> Result<Apps, sqlx::Error> {
    let response = sqlx::query("insert into apps (name, owner) values (?, ?)")
        .bind(&app.name)
        .bind(&app.owner)
        .execute(connection.get_pool())
        .await?;

    let id = response.last_insert_id();

    let app = Apps {
        id: id as i32,
        name: app.name,
        owner: app.owner,
    };

    Ok(app)
}

pub async fn list(connection: &Database, owner: &str) -> Result<Vec<Apps>, sqlx::Error> {
    let apps: Vec<Apps> = sqlx::query_as("select id, name, owner from apps where owner = ?")
        .bind(owner)
        .fetch_all(connection.get_pool())
        .await?;

    Ok(apps)
}

pub enum AppsUpdateError {
    NotFound,
    Generic(sqlx::Error),
}

pub async fn update(connection: &Database, app: Apps) -> Result<Apps, AppsUpdateError> {
    let response = sqlx::query("update apps set name = ? where id = ? and owner = ?")
        .bind(&app.name)
        .bind(&app.id)
        .bind(&app.owner)
        .execute(connection.get_pool())
        .await
        .map_err(AppsUpdateError::Generic)?;

    if response.rows_affected() == 0 {
        return Err(AppsUpdateError::NotFound);
    }

    Ok(app)
}
