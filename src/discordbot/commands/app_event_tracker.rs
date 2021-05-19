use std::time::Duration;


use mysql::serde_json::json;
use regex::Regex;
use serenity::{builder::CreateEmbed, client::Context, framework::standard::{Args, CommandResult, macros::{command, group}}, model::channel::Message, utils::Color};

use crate::{config, consts::colors, discordbot::helpers::SmallerString, mysql_db::{self, ReportsTable}, webserver::dtos::Severity};

#[group("Rastreador de Eventos 🖥️")]
#[commands(registrar_aplicacao, list_events, event_detail)]
pub struct AppEventTracker;

#[command("regapp")]
#[aliases("addapp")]
#[description("Registra uma nova aplicação para receber o rastreamento de eventos.")]
#[usage("")]
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
                    let mut token_embed = common_embed;
                    token_embed
                        .description("Sua aplicação foi cadastrada com sucesso, abaixo as informações sobre seu cadastro:")
                        .field("Identificador:", app.id, true)
                        .field("Nome da Aplicação:", &app.name, true)
                        .field("Token da Aplicação:", format!("||{}||", &app.token_app), false);


                    let result = loop_send_dm(&token_embed, ctx, msg)
                        .await;

                    match result {
                        Ok(_) => {
                            msg.channel_id.send_message(ctx, |f| f
                                .embed(|e| e
                                    .color(Color::DARK_GREEN)
                                    .description("Aplicação cadastrada com sucesso.\n\nDados especiais foram mandado para seu DM.")
                                )
                            )
                                .await?;
                        },
                        Err(why) => return Err(why)
                    }
                }
            }
        }
    }

    Ok(())
}


async fn loop_send_dm(token_embed: &CreateEmbed, ctx: &Context, msg: &Message) -> CommandResult {
    loop {
        if send_dm_message_done(token_embed, ctx, msg).await.is_ok() {
            return Ok(());
        }

        let msg_err = msg.channel_id.send_message(ctx, |f| f
            .embed(|e| e
                .color(Color::RED)
                .description("Sua dm esta bloqueada, favor libere sua dm e pressione o ✅ abaixo.")
            )
        )
            .await?;
        msg_err.react(ctx, '✅')
            .await?;
        msg.author.await_reaction(ctx)
            .await;
    }
}

async fn send_dm_message_done(token_embed: &CreateEmbed, ctx: &Context, msg: &Message) -> CommandResult {
    msg.author
        .create_dm_channel(ctx)
        .await?
        .send_message(ctx, |f| f
            .set_embed(token_embed.clone())
        )
        .await?;
    Ok(())
}

#[command("evlist")]
#[aliases("listev", "lsev")]
#[description("Lista os ultimos 25 eventos de uma aplicação.")]
#[usage("{id da aplicação}")]
async fn list_events(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let id_app = args.single::<u64>()?;
    let elements = ReportsTable::get_last_25(id_app, msg.author.id.0)?;
    let mut events_format = String::from("Não existem eventos para esse app.");
    if !elements.is_empty() {
        events_format = elements
            .iter()
            .fold("".to_string(), |old, new|  format!("{}\n{}: ({}): {}", old, new.id, String::from(Severity::from(new.severity)), &new.title));
    }
    msg.channel_id.send_message(ctx, |f| f
        .embed(|e| e
            .title("Ultimos 25 eventos:")
            .description(format!("```{}```", events_format))
            .footer(|f| f
                .text(format!("Use {}evdet {{:id}} para ver informações sobre o evento.", config::get_bot_prefix()))
            )
            .color(colors::VIOLET)
        )
    )
        .await?;
    Ok(())
}

#[command("evdet")]
#[aliases("detev", "eventdetail")]
#[description("Envia os detalhes sobre um evento em especifico.")]
#[usage("{id do evento}")]
async fn event_detail(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let id_ev = args.single::<u64>()?;
    let element = ReportsTable::get(id_ev, msg.author.id.0);
    match element {
        None => {
            msg.channel_id
                .send_message(ctx, |f| f
                    .embed(|e| e
                        .color(Color::RED)
                        .description("Informação não encontrada.")
                    )
                )
                .await?;
        },
        Some(report) => {
            let severity = Severity::from(report.severity);
            msg.channel_id
                .send_message(ctx, |f| f
                    .embed(|e| e
                        .color(Color::from(severity))
                        .title(format!("{}: {}", String::from(severity), &report.title))
                        .description(format!("```{}```", &report.message))
                        .field("Stacktrace:", String::from(SmallerString::from(report.stacktrace.as_ref().unwrap())), true)
                    )
                )
                .await?;
        }
    }
    Ok(())
}

