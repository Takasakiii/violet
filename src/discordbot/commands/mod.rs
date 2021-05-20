mod app_event_tracker;

use std::collections::HashSet;
use serenity::{builder::CreateEmbed, client::Context, framework::{StandardFramework, standard::{Args, CommandGroup, CommandResult, HelpOptions, macros::{hook, help}}}, model::{channel::Message, id::UserId}, utils::Color};
use crate::{config, consts::colors};

use super::helpers;

pub fn get_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix(&config::get_bot_prefix()[..]))
        .after(after_hook)
        .help(&HELP)
        .group(&app_event_tracker::APPEVENTTRACKER_GROUP)
}


#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, cmd_result: CommandResult) {
    if let Err(why) = cmd_result {
        msg.react(ctx, '❌')
            .await
            .ok();

        helpers::send_err(ctx, why, Some(cmd_name))
            .await;
    }
}

#[help]
async fn help(ctx: &Context, msg: &Message, mut args: Args, _help_options: &'static HelpOptions, groups: &[&'static CommandGroup], _owners: HashSet<UserId>) -> CommandResult{
    let prefix = config::get_bot_prefix();

    if args.is_empty() {
        let mut embed_send = CreateEmbed::default();
        embed_send.color(colors::VIOLET);
        if args.is_empty() {
            embed_send.title("**Meus comandos:**")
                .description(format!("Sou Violet estou aqui para ajudar meus criadores e até mesmo você com logs e reports de bugs nas aplicações.\n\nMeu prefixo é: `{}`, e abaixo você pode encontrar a lista de meus comandos:", config::get_bot_prefix()))
                .field("Ajuda ⁉:", format!("`{}help`", &prefix), false)
                .footer(|f| f
                    .text(format!("Use {}help {{nome do comando}} para mais ajuda sobre o mesmo", &prefix))
                );

            for group in groups.iter() {
                let name_module = format!("{}:", group.name);
                let cmds = group.options.commands
                    .iter()
                    .map(|el| {
                        el.options.names
                            .first()
                            .map(|el| el.to_string())
                            .unwrap()
                    })
                    .fold("".to_string(), |ini, new| format!("{} `{}`", ini, new));
                embed_send.field(name_module, cmds, false);
            }
        }

        msg.channel_id.send_message(ctx, |f| f
            .reference_message(msg)
            .set_embed(embed_send)
        )
            .await?;
    } else {
        let cmd_name = args.single::<String>()?;

        if cmd_name.eq("help") {
            msg.channel_id.send_message(ctx, |f| f
                .embed(|e| e
                    .color(colors::VIOLET)
                    .title("Help")
                    .description("Esse é o comando que possui a listagem de informações de todos os outros comandos 👍")
                )
            )
            .await?;
            return Ok(());
        }

        let mut cmd = None;
        'command_finder: for gp in groups.iter() {
            cmd = gp.options.commands
                .iter()
                .find(|c| c.options.names
                    .iter()
                    .any(|n| n.eq(&&cmd_name))
                );

            if cmd.is_some() {
                break 'command_finder;
            }
        }

        match cmd {
            None => {
                msg.channel_id
                .send_message(ctx, |f| f
                    .embed(|e| e
                        .color(Color::RED)
                        .description("Comando não existe.")
                    )
                )
                .await?;
            },
            Some(cmd) => {
                let oficinal_cmd_name = cmd.options.names
                    .first()
                    .unwrap();
                msg.channel_id
                    .send_message(ctx, |f| f
                        .embed(|e| e
                            .color(colors::VIOLET)
                            .title(oficinal_cmd_name)
                            .description(&cmd.options.desc
                                .unwrap_or("Nenhuma descrição informada")
                            )
                            .field("Aliases:", cmd.options.names
                                .iter()
                                .skip(1)
                                .fold(String::new(), |old, new| format!("{}\n`{}`", old, new))
                            , true)
                            .field("Uso:", format!("`{}{} {}`", &prefix, oficinal_cmd_name,cmd.options.usage
                                    .unwrap_or("Sem uso informado")
                                )
                            , true)
                        )
                    )
                    .await?;
            }
        }
    }
    Ok(())
}
