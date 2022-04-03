pub mod app_tokens;
pub mod apps;
pub mod errors;
pub mod users;

use std::time::Duration;

use actix_web::rt::time;
use sqlx::{
    mysql::{MySql, MySqlDatabaseError, MySqlPoolOptions},
    Pool,
};

use crate::config::Config;

pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new(config: &Config) -> Self {
        log::info!("Connecting to database");

        let mysql_pool = loop {
            let connection = MySqlPoolOptions::new()
                .max_connections(10)
                .connect(&config.database_url)
                .await;

            if let Ok(connection) = connection {
                break connection;
            } else {
                log::error!("Failed to connect to database, retrying in 5 seconds");
                time::sleep(Duration::from_secs(5)).await;
            }
        };

        log::info!("Connected to database");

        Self { pool: mysql_pool }
    }

    pub async fn migrate(&self) {
        log::info!("Migrating database");
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .expect("Failed to migrate database");
        log::info!("Migrated database");
    }

    pub(self) fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}

pub(crate) trait SqlxErrorExtension {
    fn get_mysql(&self) -> &MySqlDatabaseError;
}

impl SqlxErrorExtension for sqlx::Error {
    fn get_mysql(&self) -> &MySqlDatabaseError {
        match self {
            sqlx::Error::Database(err) => err.downcast_ref::<MySqlDatabaseError>(),
            _ => panic!("Unexpected error type"),
        }
    }
}
