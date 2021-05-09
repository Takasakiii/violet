use serenity::{async_trait, client::{Context, EventHandler}, model::prelude::Ready};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("Violet está conectada ao discord como: {}", data_about_bot.user.tag())
    }
}
