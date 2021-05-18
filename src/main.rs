mod webserver;
mod discordbot;
pub mod config;
pub mod consts;
pub mod tokens;
mod mysql_db;
pub mod channels;

use dotenv::dotenv;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenv()
        .ok();

    channels::GerChannels::get(|g| {
        g.create_channel("send_app_event");
        Ok(())
    })
        .expect("Falha ao criar os canais");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    mysql_db::create_database()
        .expect("Não foi possivel criar a database.");

    discordbot::start();
    webserver::start_web_server();
}


