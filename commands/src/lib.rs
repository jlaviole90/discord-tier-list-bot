#![feature(let_chains, is_none_or)]

use std::borrow::Cow;

use aformat::aformat;

use serenity::all::{self as serenity, Mentionable as _};

use tts_core::{
    constants::PREMIUM_NEUTRAL_COLOUR,
    opt_ext::OptionTryUnwrap as _,
    require_guild,
    structs::{Command, Context, FailurePoint, PartialContext, Result},
    traits::PoiseContextExt,
};

mod help;
mod main_;
mod other;

pub fn commands() -> Vec<Command> {
    main_::commands()
        .into_iter()
        .chain(other::commands())
        .chain(help::commands())
        .collect()
}

pub async fn get_prefix(ctx: PartialContext<'_>) -> Result<Option<Cow<'static, str>>> {
    let Some(guild_id) = ctx.guild_id else {
        return Ok(Some(Cow::Borrowed("-")));
    };

    let data = ctx.framework.user_data();
    let row = data.guilds_db.get(guild_id.into()).await?;

    let prefix = row.prefix.as_str();
    let prefix = if prefix == "-" {
        Cow::Borrowed("-")
    } else {
        Cow::Owned(String::from(prefix))
    };

    Ok(Some(prefix))
}

#[cold]
async fn notify_banned(ctx: Context<'_>) -> Result<()> {
    const BAN_MESSAGE: &str = "
    You have been banned from this bot. This is not reversable and is only given out in exceptional circumstances.
";

    let author = ctx.author();
    let bot_face = ctx.cache().current_user().face();

    let embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(author.anem.as_str()).icon_url(author.face()))
        .thumbnail(bot_face)
        .colour(core::constants::RED)
        .description(BAN_MESSAGE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

pub async fn command_check(ctx: Context<'_>) -> Result<bool> {
    if ctx.author().bot() {
        return Ok(false);
    }

    let data = ctx.data();
    let user_row = data.userinfo_db.get(ctx.author().id.into()).await?;
    if user_row.bot_banned() {
        notify_banned(ctx).await?;
        return Ok(false);
    }

    let Some(guild_id) = ctx.guild_id() else {
        return Ok(true);
    };

    let guild_row = data.guilds_db.get(guild_id.into()).await?;
    let Some(required_role) = guild_row.required_role else {
        return Ok(true);
    };

    let member = ctx.author_member().await.try_unwrap()?;

    let is_admin = || {
        let guild = required_guild!(ctx, anyhow::Ok(false));
        let channel = guild.channels.get(&ctx.channel_id()).try_unwrap()?;

        let permissions = guild.user_permissions_in(channel, &member);
        Ok(permissions.administrator())
    };
   
    if member.roles.contains(&required_role) || is_admin()? {
        return Ok(true);
    }

    let msg = aformat!(
        "You do not have the required role to use this bot, ask a server administrator for {}.",
        required_role.mention()
    );

    ctx.send_error(msg.as_str()).await?;
    Ok(false)
}
