use std::usize;

use chrono::Utc;
use serde::Serialize;
use serenity::{client::Context, framework::standard::CommandError, utils::Color};

use crate::config;

pub fn reduce_to_field(el: &str, cut_len: usize) -> String {
    if el.len() > cut_len {
        format!("{}...", &el[..cut_len - 3])
    } else {
        el.to_string()
    }
}

#[derive(Serialize)]
pub struct WebhookEmbed {
    pub embeds: Vec<EmbedSerializer>
}

#[derive(Serialize)]
pub struct EmbedSerializer {
    pub author: AuthorEmbed,
    pub title: String,
    pub description: String,
    pub color: u32,
    pub fields: Option<Vec<FieldEmbed>>
}

#[derive(Serialize)]
pub struct AuthorEmbed {
    pub name: String
}

#[derive(Serialize)]
pub struct FieldEmbed {
    pub name: String,
    pub value: String,
    pub inline: bool
}

pub async fn send_err<'a>(ctx: &'a Context, error: CommandError, cmd_name: Option<&'a str>) {
    if format!("{:?}", &error).eq("Eos") {
        return;
    }

    let now = Utc::now()
        .to_string();
    println!("[{}][Erro no bot]: {}: {:?}", now, cmd_name.unwrap_or("NaC"), &error);


    let owner_channel = ctx.http.get_user(config::get_bot_owner())
        .await
        .ok()
        .map(|owner| {
            async move {
                owner.create_dm_channel(ctx)
                    .await
                    .ok()
                    .unwrap()

            }
        });

    if let Some(channel_dm_owner) = owner_channel {
        channel_dm_owner
            .await
            .send_message(ctx, |f| f
                .embed(|e| e
                    .title("Ocorreu um erro desconhecido na **Violet**")
                    .description(format!("```{}: {:?}```", cmd_name.unwrap_or("NaC"), &error))
                    .color(Color::RED)
                )
            )
            .await
            .ok();
    }
}
