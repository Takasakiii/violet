use std::time::Duration;

use serenity::{builder::CreateEmbed, client::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::Message};

use crate::{config, consts::colors};

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

            if webhook.content.starts_with("create ") {
                let slited_webhook = webhook.content
                    .split(" ")
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" ");
            }
        }
    }


    Ok(())
}
