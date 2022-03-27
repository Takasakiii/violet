use std::time::Duration;

use actix_web::rt::time;
use sqlx::{
    mysql::{MySql, MySqlPoolOptions},
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
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .expect("Failed to migrate database");
    }

    pub(self) async fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}
