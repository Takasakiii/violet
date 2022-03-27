mod config;

use std::io;

use actix_web::{middleware::Logger, App, HttpServer};
use config::Config;
use env_logger::Env;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let config = Config::get_config();

    HttpServer::new(|| App::new().wrap(Logger::default()))
        .bind(("0.0.0.0", config.server_port))?
        .run()
        .await
}
