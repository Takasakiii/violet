use serde::Serialize;

use super::{Database, SqlxErrorExtension};

pub enum UsersErrors {
    DuplicateUsername,
    Generic(sqlx::Error),
}

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
