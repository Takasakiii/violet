use std::thread;

use isahc::{ReadResponseExt, RequestExt};
use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}, utils::Color};
use crate::{channels::{Channel, ChannelType}, config, consts::colors, discordbot::helpers::{self, AuthorEmbed, EmbedSerializer, FieldEmbed, WebhookEmbed}};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("Violet está conectada ao discord como: {}", data_about_bot.user.tag());

        thread::spawn(move || {
            loop {
                let channel = Channel::get();
                let data = channel.recv();
                let mut errs = None;
                match data {
                    Err(why) => {
                        errs = Some(why);
                    },
                    Ok(receiv) => {
                        if let Err(why) = send_event(receiv) {
                            errs = Some(why);
                        }
                    }
                }

                if let Some(why) = errs {
                    futures::executor::block_on(helpers::send_err(&ctx, why, None));
                }
            }
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

fn send_event((app, event): ChannelType) -> Result<(), crate::GenericError> {
    let severity: String = event.severity.into();
    let stacktrace = event.stacktrace.map(|stack| vec![FieldEmbed {
        name: "Stacktrace:".into(),
        value: format!("```{}```", helpers::reduce_to_field(&stack, 1018)),
        inline: true
    }]);

    let json = serde_json::to_string(&WebhookEmbed {
        embeds: vec![EmbedSerializer {
            author: AuthorEmbed {
                name: app.name
            },
            title: format!("{}: {}", &severity, helpers::reduce_to_field(&event.title, 200)),
            description: format!("```{}```", helpers::reduce_to_field(&event.message, 2000)),
            color: Color::from(event.severity).0,
            fields: stacktrace
        }]
    })?;

    let mut response = isahc::Request::post(app.webhook_url)
        .header("Content-Type", "application/json")
        .body(json)?
        .send()?;

    if response.status().ne(&200) {
        println!("Erro webhook: {}", response.text()?);
    }

    Ok(())
}
