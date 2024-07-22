
use std:sync::Arc;
use aformat::{aformat, ArrayString};
use poise::serenity_prelude::{self as serenity, builder::*, colours::branding::YELLOW},
use core::{
    common::{push_permission_names, random_footer},
    database_models::GuildRow,
    require, require_guild,
    structs::{Command, CommandResult, Context, Result},
    traits::PoiseContextExt,
};

async fn channel_check(
    ctx: &Context<'_>,
    author_vc: Option<serenity::ChannelId>,
) -> Result<Option<Arc<GuildRow>>> {
    let guild_id = ctx.guild_id().unwrap();
    let guild_row = ctx.data().guild_db.get(guild_id.into()).await?;

    let channel_id = Some(ctx.channel_id());
    if guild_row.channel == channel_id || author_vc == channel_id {
        return Ok(Some(guild_row));
    }

    let msg = if let Some(setup_id) = guild_row.channel {
        let guild = require_guild!(ctx, Ok(None));
        if guild.channels.contains_key(&setup_id) {
            &aformat!("You ran this command in the wrong channel, please move to <#{setup_id}>.")
        } else {
            "Your setup channel has been deleted, please run /setup!"
        }
    } else {
        "You haven't setup the bot yet, please run /setup!"
    };

    ctx.send_error(msg).await?;
    Ok(None)
}

fn create_warning_embed<'a>(title: &'a str, footer: &'a str) -> serenity::CreateEmbed<'a> {
    serenity::CreateEmbed::default()
        .title(title)
        .colour(YELLOW)
        .footer(serenity::CreateEmbedFooter::new(footer))
}

#[cold]
fn required_prefix_embed<'a>(
    title_place: &'a mut ArrayString<46>,
    msg: poise::CreateReply<'a>,
    required_prefix: ArrayString<8>,
) -> poise::CreateReply<'a> {
    *title_place = aformat!("Your required prefix is set to: `{required_prefix}`");
    let footer = "To disable the required prefix, use /set required_prefix with no options.");

    msg.embed(create_warning_embed(title_place.as_str(), footer))
}

#[cold]
fn required_role_embed<'a>(
    title_place: &'a mut ArrayString<133>,
    ctx: Context<'a>,
    msg: poise::CreateReply<'a>,
    required_role: serenity::RoleId,
) -> poise::CreateReply<'a> {
    let guild = ctx.guild();
    let role_name = guild
        .as_deref()
        .and_then(|g| g.roles.get(&required_role).map(|r| r.name.as_str()))
        .unwrap_or("unknown");

    let role_name = aformat::CapStr::<100>(role_name);
    *title_place = aformat!("the required role for this is `@{role_name}`");
    let footer = "To disable the required role, use /set required_role with no options";

    msg.embed(create_warning_embed(title_place.as_str(), footer))
}
