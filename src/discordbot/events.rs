use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}};
use crate::{config, consts::colors};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("Violet está conectada ao discord como: {}", data_about_bot.user.tag());

        tokio::spawn(async {
            // GerChannels::get(|g| {
            //     g.get_channel("send_app_event", |c| {
            //         println!("Debug: {:?}", c.get_data::<EventTrackerReceive>())
            //     }).unwrap();
            // });
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let id_bot = ctx.cache.current_user()
            .await
            .id
            .0;

        let template_1 = format!("<@!{}>", id_bot);
        let template_2 = format!("<@{}>", id_bot);
        if msg.content.len() <= 22 && (msg.content.starts_with(&template_1) || msg.content.starts_with(&template_2)) {
            msg.channel_id.send_message(ctx, |f| f
                .reference_message(&msg)
                .embed(|e| e
                    .color(colors::VIOLET)
                    .title("Ola eu sou a Violet 👋")
                    .description(format!("Meus comandos podem ser vistos usando `{}help`.", config::get_bot_prefix()))
                )
            )
                .await
                .ok();
        }
    }
}
