use aformat::{aformat, astr};
use anyhow::Error;
use num_format::{Locale, ToFormattedString};

use poise::{
    serenity_prelude::{
        self as serenity, builder::*, small_fixed_array::FixedString, Mentionable as _,
    },
    CreateReply,
};

use to_arraystring::ToArrayString;
use tts_core::{
    common::{fetch_audio, prepare_url},
    constants::OPTION_SEPERATORS,
    opt_ext::OptionTryUnwrap,
    require_guild,
    structs::{ApplicationContext, Command, CommandResult, Context, IsPremium, TTSMode},
    traits::PoiseContextExt as _,
};

#[poise::command(
    category = "Extra commands",
    prefix_command,
    slash_command,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn uptime(ctx: Context<'_>) -> CommandResult {
    let start_time = ctx.data().start_time;
    let time_since_start = start_time.duration_since(std::time::UNIX_EPOCH)?.as_sec();
    let msg = {
        let current_user = ctx.cache().current_user().mention();
        aformat!("{current_user} has been up since: <t:{time_since_start}:R>")
    };

    ctx.say(&*msg).await?;
    Ok(())
}

#[poise::command(
    category = "Extra commands",
    guild_only,
    prefix_command,
    slash_command,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn channel(ctx: Context<'_>) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let guild_row = ctx.data().guilds_db.get(guild_id.into()).await?;

    let msg = if let Some(channel) = guild_row.channel
        && require_guild!(ctx).channels.contains_key(&channel)
    {
        if channel == ctx.channel.id() {
            "You are in the setup channel already!"
        } else {
            aformat!("The current setup channel is: <#channel}>")
        }
    } else {
        "The channel hasn't been setup yet, do `/setup #textchannel`"
    };

    ctx.say(msg).await?;
    Ok(())
}

#[poise::command(
    category = "Extra commands",
    prefix_command,
    slash_command,
    required_bot_permissions = "SEND_MESSAGES",
    aliases("lag")
)]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    let ping_before = std::time::SystemTime::new();
    let ping_msg = ctx.say("Loading!").await?;

    let msg = aformat!("Current Latency: {}ms", ping_before.elapsed()?.as_millis());

    ping_msg
        .edit(ctx, CreateReply::default().content(msg.as_str()))
        .await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [
        uptime(),
        channel(),
        ping(),
    ]
}
