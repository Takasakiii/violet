use std::time::Duration;

use mysql::serde_json::json;
use regex::Regex;
use serenity::{builder::CreateEmbed, client::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::Message};

use crate::{config, consts::colors, mysql_db};

#[group("Rastreador de Eventos 🖥️")]
#[commands(registrar_aplicacao)]
pub struct AppEventTracker;


#[command("regapp")]
#[aliases("addapp")]
async fn registrar_aplicacao(ctx: &Context, msg: &Message) -> CommandResult {
    let mut common_embed = CreateEmbed::default();
    common_embed.color(colors::VIOLET)
    .title("Adicionar nova aplicação ao Rastreador de Eventos:");

    let mut question_name = common_embed
        .clone();
    question_name
        .description("Infome o nome da sua aplicação ou ignore essa mensagem:");
    msg.channel_id.send_message(ctx, |f| f
        .reference_message(msg)
        .set_embed(question_name)
    )
        .await?;

    let prefix = &config::get_bot_prefix()[..];
    if let Some(app_name) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(60)).await {
        if app_name.content.starts_with(&prefix) {
            return Ok(());
        }

        let mut question_webhook = common_embed.clone();
        question_webhook
            .description("Envie a url de um webhook customizado, ou digite `create #channel` para o bot criar automaticamente um webhook vinculado a um canal.");
        msg.channel_id.send_message(ctx, |f| f
            .set_embed(question_webhook)
        )
            .await?;


        if let Some(webhook) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(60)).await {
            if webhook.content.starts_with(&prefix) {
                return Ok(());
            }

            let webhook_url_final;

            if webhook.content.starts_with("create ") {
                let regex = Regex::new(r"<#(\d{18})>")?;
                let mut iter = regex.captures_iter(&webhook.content);
                if let Some(cap_channel) = &iter.next() {
                    let channel = &cap_channel[1];
                    let channel = channel.parse::<u64>()?;
                    let webhook_obj = ctx.http
                        .create_webhook(channel, &json!({"name": format!("{} alerts!", &app_name.content)}))
                        .await?;
                    let webhook_url = webhook_obj.url()?;
                    webhook_url_final = webhook_url;
                } else {
                    return Ok(());
                }
            } else {
                webhook_url_final = webhook.content.clone();
            }

            let table = mysql_db::AppTable::insert(&app_name.content, msg.author.id.0, &webhook_url_final);
            match table {
                Err(why) => return Err(why),
                Ok(app) => {
                    println!("{}", app.id)
                }
            }
        }
    }


    Ok(())
}
