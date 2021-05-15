mod app_event_tracker;

use std::collections::HashSet;
use serenity::{builder::CreateEmbed, client::Context, framework::{StandardFramework, standard::{Args, CommandGroup, CommandResult, HelpOptions, macros::{hook, help}}}, model::{channel::Message, id::UserId}};
use crate::{config, consts::colors};

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
        println!("Ocorreu um erro no {}: {:?}", &cmd_name, &why);
        msg.react(ctx, '❌')
            .await
            .ok();
    }
}

#[help]
async fn help(ctx: &Context, msg: &Message, args: Args, _help_options: &'static HelpOptions, groups: &[&'static CommandGroup], _owners: HashSet<UserId>) -> CommandResult{
    let mut embed_send = CreateEmbed::default();
    embed_send.color(colors::VIOLET);
    if args.len() == 0 {
        embed_send.title("**Meus comandos:**")
            .description(format!("Sou Violet estou aqui para ajudar meus criadores e até mesmo você com logs e reports de bugs nas aplicações.\n\nMeu prefixo é: `{}`, e abaixo você pode encontrar a lista de meus comandos:", config::get_bot_prefix()))
            .field("Ajuda ⁉:", format!("`{}help`", config::get_bot_prefix()), false);

        for group in groups.iter() {
            let name_module = format!("{}:", group.name);
            embed_send.field(name_module, "a", false);
        }
    }

    msg.channel_id.send_message(ctx, |f| f
        .reference_message(msg)
        .set_embed(embed_send)
    )
        .await?;

    // let help_data = help_commands::create_customised_help_data(ctx, msg, &args, groups, &owners, help_options)
    //     .await;

    // match help_data {

    // }
    Ok(())
}
