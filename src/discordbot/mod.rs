mod events;

use serenity::{Client, framework::StandardFramework};
use crate::config;

pub fn start() {
    tokio::spawn(async {
        start_bot()
            .await
            .expect("Não foi possivel iniciar o bot.");
    });
}

async fn start_bot() -> Result<(), serenity::Error> {
    let mut client = Client::builder(config::get_discord_token())
        .event_handler(events::Handler)
        .framework(StandardFramework::new())
        .await?;

    client
        .start()
        .await?;
    Ok(())
}
