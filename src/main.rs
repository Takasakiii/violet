mod config;
mod controllers;
mod database;
mod jwt;
mod tokens;

use std::io;

use actix_cors::Cors;
use actix_web::{
    error::InternalError,
    middleware::Logger,
    web::{Data, JsonConfig},
    App, HttpResponse, HttpServer,
};

use config::Config;
use database::Database;
use env_logger::Env;
use jwt::Jwt;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let config = Config::get_config();

    let database = Database::new(&config).await;
    database.migrate().await;

    let database_data = Data::new(database);
    let jwt = Data::new(Jwt::new(&config));

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(serde_json::json!({
                            "error": err.to_string()
                        })),
                )
                .into()
            }))
            .app_data(database_data.clone())
            .app_data(jwt.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(controllers::auth_routes())
            .service(controllers::apps_routes())
            .service(controllers::errors_routes())
            .service(controllers::test_routes())
    })
    .bind(("0.0.0.0", config.server_port))?
    .run()
    .await
}
