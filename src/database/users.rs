use serde::Serialize;
use sqlx::FromRow;

use super::{Database, SqlxErrorExtension};

pub enum UsersErrors {
    DuplicateUsername,
    Generic(sqlx::Error),
}

#[derive(FromRow)]
pub struct Users {
    pub username: String,
    pub password_hash: String,
    pub last_token: Option<String>,
}

#[derive(Serialize)]
pub struct UsersDtoResult {
    pub username: String,
}

pub async fn create(connection: &Database, user: Users) -> Result<UsersDtoResult, UsersErrors> {
    let result = sqlx::query("insert into users (username, password, last_token) values (?, ?, ?)")
        .bind(user.username.clone())
        .bind(user.password_hash)
        .bind(user.last_token)
        .execute(connection.get_pool())
        .await;

    match result {
        Ok(_) => Ok(UsersDtoResult {
            username: user.username,
        }),
        Err(err @ sqlx::Error::Database(_)) if err.get_mysql().number() == 1062 => {
            Err(UsersErrors::DuplicateUsername)
        }
        Err(err) => Err(UsersErrors::Generic(err)),
    }
}

pub enum UsersGetByUsernameErrors {
    NotFound,
    Generic(sqlx::Error),
}

pub async fn get_by_username(
    connection: &Database,
    username: String,
) -> Result<Users, UsersGetByUsernameErrors> {
    let result: Users = sqlx::query_as("select * from users where username = ?")
        .bind(username)
        .fetch_one(connection.get_pool())
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UsersGetByUsernameErrors::NotFound,
            err => UsersGetByUsernameErrors::Generic(err),
        })?;

    Ok(result)
}

pub async fn add_last_token(
    connection: &Database,
    last_token: String,
    username: String,
) -> Result<(), sqlx::Error> {
    sqlx::query("update users set last_token = ? where username = ?")
        .bind(last_token)
        .bind(username)
        .execute(connection.get_pool())
        .await?;

    Ok(())
}

pub async fn get_by_token(connection: &Database, token: String) -> Result<Users, sqlx::Error> {
    let user: Users = sqlx::query_as("select * from users where last_token = ?")
        .bind(token)
        .fetch_one(connection.get_pool())
        .await?;

    Ok(user)
}
