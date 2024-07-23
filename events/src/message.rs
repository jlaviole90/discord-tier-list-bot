use std::borrow::Cow;

use aformat::aformat;
use to_arraystring::ToArrayString;
use tracing::info;

use self::serenity::{CreateEmbed, CreateEmbedFooter, CreateMessage, ExecuteWebhook, Mentionable};
use poise::serenity_prelude as serenity;

use core::{
    common::random_footer,
    constants::DM_WELCOME_MESSAGE,
    errors,
    opt_ext::OptionTryUnwrap,
    require,
    structs::{Data, FrameworkContext, Result},
};

pub async fn message(
    framework_ctx: FrameworkContext<'_>,
    new_message: &serenity::Message,
) -> Result<()> {
    tokio::try_join!(
        process_mention_msg(framework_ctx, new_message),
    )?;

    Ok(())
}

async fn process_mention_msg(
    framework_ctx: FrameworkContext<'_>,
    message: &serenity::Message,
) -> Result<()> {
    let data = framework_ctx.user_data();
    let Some(bot_mention_regex) = data.regex_cache.bot_mention.get() else {
        return Ok(());
    };

    if !bot_mention_regex.is_match(&message.content) {
        return Ok(());
    };

    let ctx = framework_ctx.serenity_context;
    let bot_user = ctx.cache.current_user().id;
    let guild_id = require!(message.guild_id, Ok(()));
    let channel = message.channel(ctx).await?.guild().unwrap();
    let permissions = channel.permissions_for_user(&ctx.cache, bot_user)?;

    let guild_row = data.guilds_db.get(guild_id.into()).await?;
    let mut prefix = guild_row.prefix.as_str().replace(['`', '\\'], "");

    if permissions.send_messages() {
        prefix.insert_str(0, "Current prefix for this server is: ");
        channel.say(&ctx.http, prefix).await?;
    } else {
        let msg = {
            let guild = ctx.cache.guild(guild_id);
            let guild_name = match guild.as_ref() {
                Some(g) => &g.name,
                None => "Unknown Server",
            };

            format!("My prefix for `{guild_name}` is {prefix} however I do not have permission to send messages so I cannot respond to your commands!")
        };

        match message
            .author
            .dm(&ctx.http, CreateMessage::default().content(msg))
            .await
        {
            Err(serenity::Error::Http(error))
                if error.status_code() == Some(serenity::StatusCode::FORBIDDEN) => {}
            Err(error) => return Err(anyhow::Error::from(error)),
            _ => {}
        }
    }

    Ok(())
}

