#![allow(clippy::module_name_repetitions)]
#![feature(let_chains, is_none_or)]

mod guild;
mod member;
mod message;
mod other;
mod ready;

use guild::*;
use member::*;
use message::*;
use other::*;
use ready::*;

use poise::serenity_prelude as serenity;
use serenity::FullEvent as Event;

use core::structs::{FrameworkContext, Result};

pub fn get_intents() -> serenity::GatewayIntents {
    serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT
}

pub async fn listen(framework_ctx: FrameworkContext<'_>, event: &Event) -> Result<()> {
    match event {
        Event::Message { new_message } => message(framework_ctx, new_message).await,
        Event::GuildCreate { guild, is_new } => guild_create(framework_ctx, guild, *is_new).await,
        Event::Ready { data_about_bot } => ready(framework_ctx, data_about_bot).await,
        Event::GuildDelete { incomplete, full } => {
            guild_delete(framework_ctx, incomplete, full.as_ref()).await
        }
        Event::GuildMemberAddition { new_member } => {
            guild_member_addition(framework_ctx, new_member).await
        }
        Event::ChannelDelete { channel, .. } => channel_delete(framework_ctx, channel).await,
        Event::InteractionCreate { interaction } => {
            interaction_create(framework_ctx, interaction).await
        }
        Event::Resume { .. } => {
            resume(&framework_ctx.user_data());
            Ok(())
        }
        _ => Ok(()),
    }
}
