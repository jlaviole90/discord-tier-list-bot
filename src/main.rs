use std::collections::HashSet;

use serenity::{all::{internal::tokio, standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions}, Context, GatewayIntents, Http, Message, StandardFramework, UserId}, Client};

const INTENTS_A: GatewayIntents = GatewayIntents::GUILD_MESSAGES;
const INTENTS_B: GatewayIntents = GatewayIntents::DIRECT_MESSAGES;
const INTENTS_C: GatewayIntents = GatewayIntents::MESSAGE_CONTENT;

#[tokio::main]
async fn main() {
    let http = Http::new(""/*token*/);

    let bot_id = match http.get_current_user().await {
        Ok(info) => info.id,
        Err(why) => {
            return Err(format!(
                "Could not access user info {why:?}, bad token input!\nCheck token expiration!"))
        }
    };

    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .configure(|c| c.on_mention(Some(bot_id)).prefix(prefix));

    let mut client = Client::builder(&token, INTENTS_A | INTENTS_B | INTENTS_C)
        .event_handler(events::Handler)
        .framework(framework)   
        .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
        Err("{why:?}".to_string())
    } else {
        Ok(true)
    }
}

#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    // TODO: set up help display
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners);
    Ok(())
}
