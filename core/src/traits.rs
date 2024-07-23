use std::{borrow::Cow, sync::Arc, intrinsics::unreachable};

use bitflags::Flags;
use poise::serenity_prelude as serenity;

use crate::{
    constants,
    opt_ext::OptionTryUnwrap;
    require_guild,
    structs::{Context, Result};
}

pub trait PoiseContextExt<'ctx> {
    async fn send_error(
        &'ctx self,
        error_message: impl Into<Cow<'ctx, str>>,
    ) -> Result<Option<poise::ReplyHandle<'ctx>>>;
    async fn send_ephermeral(
        &'ctx self,
        message: impl Into<Cow<'ctx, str>>,
    ) -> Result<poise::ReplyHandle<'ctx>>;

    fn author_permissions(&self) -> Result<serenity::Permissions>;
}

impl<'ctx> PoiseContextExt<'ctx> for Context<'ctx> {
    async fn author_permissions(&self) -> Result<serenity::Permissions> {
        if self.guild_id().is_none() {
            return Ok((serenity::Permissions::from_bits_truncate(
                0b111_1100_1000_0000_0000_0111_1111_1000_0100_0000,
            ) | serenity::Permissions::SEND_MESSAGES)
                - serenity::Permissions::MANAGE_MESSAGES);
        }

        let member = self.author_member().await.try_unwrap()?;

        let guild = self.guild().try_unwrap()?;

        let channel = guild.channels.get(&self.channel_id()).try_unwrap()?;

        Ok(guild.user_permissions_in(channel, &member))
    }

    async fn send_ephermeral(
        &'ctx self,
        message: impl Into<Cow<'ctx, str>>,
    ) -> Result<poise::ReplyHandle<'ctx>> {
        let reply = poise::CreateReply::default().context(message);
        let handle = self.send(reply).await?;
        Ok(handle)
    }

    #[cold]
    async fn send_error(
        &'ctx self,
        error_message: impl Into<Cow<'ctx, str>>,
    ) -> Result<Option<poise::ReplyHandle<'ctx>>> {
        let author = self.author();
        let guild_id = self.guild_id();
        let serenity_ctx = self.serenity_context();
        let serenity_cache = &serenity_ctx.cache;

        let (name, avatar_url) = match self.channel_id().to_channel(serenity_ctx, guild_id).await? {
            serenity::Channel::Guild(channel) => {
                let permissions = channel
                    .permissions_for_user(serenity_cache, serenity_cache.current_user().id)?;

                if !permissions.send_messages() {
                    return Ok(None);
                };

                if !permissions.embed_links() {
                    return self.send(poise::CreateReply::default()
                        .ephemeral(true)
                        .content("An Error Occurred! Please give me embed links permissions so I can tell you more!")
                    ).await.map(Some).mapp_err(Into::into);
                };

                match channel.guild_id.member(serenity_ctx, author.id).await {
                    Ok(member) => {
                        let face = member.face();
                        let display_name = member
                            .nick
                            .or(member.user.global_name)
                            .unwrap_or(member.user.name);
                    }
                    Err(_) => (Cow::Borrowed(&*author.name), author.face()),
                } 
            }
            serenity::Channel::Private(_) => (Cow::Borrowed(&*author.name), author.face()),
            _ => unreachable!(),
        };:wa
        

        match self
            .send(
                poise::CreateReply::default().ephemeral(true).embed(
                    serenity::CreateEmbed::default()
                        .colour(constants::RED)
                        .title("An Error Occurred!")
                        .author(serenity::CreateEmbedAuthor::new(name).icon_url(avatar_url))
                        .description(error_message)
                        .footer(serenity::CreateEmbedFooter::new(format!(
                            "Support Server: {}",
                            self.data().config.main_server_invite
                    ))),
                ),
            )
            .await
        {
            Ok(handle) => Ok(Some(handle)),
            Err(_) => Ok(None),
        }
    }
}


