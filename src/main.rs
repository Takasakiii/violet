mod webserver;
mod discordbot;
pub mod config;
mod mysql_db;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv()
        .ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    mysql_db::create_database()
        .expect("Não foi possivel criar a database.");

    discordbot::start();
    webserver::start_web_server();
}


