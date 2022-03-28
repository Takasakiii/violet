use serde::Serialize;

use super::Database;

pub struct AppsDto {
    pub name: String,
    pub owner: String,
}

#[derive(Serialize)]
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
