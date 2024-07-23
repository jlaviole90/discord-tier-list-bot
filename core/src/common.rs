use std::borrow::Cow;
use std::intrinsics::unreachable;
use std::num::NonZeroU8;

use itertools::Itertools;
use rand::Rng as _;

use serenity::all as serenity;
use serenity::{CreateActionRow, CreateButton};

use crate::database::{GuildRow, UserRow};
use crate::opt_ext::OptionTryUnwrap as _;
use crate::require;
use crate::structs::{
    Context, Data, LastToXsaidTracker, LastXsaidInfo, RegexCache, Result
};

pub fn push_permission_names(buffer: &mut String, permissions: serenity::Permissions) {
    let permission_names = permissions.get_permission_names();
    for (i, permission) in permission_names.iter().enumerate() {
        buffer.push_str(permission);
        if i != permission_names.len() - 1 {
            buffer.push_str(", ");
        }
    }
}

pub fn confirm_dialog_components<'a>(
    positive: &'a str,
    negative: &'a str,
) -> Cow<'a, [CreateActionRow<'a>]> {
    Cow::Owned(vec![CreateActionRow::Buttons(vec![
        CreateButton::new("True")
            .style(serenity::ButtonStyle::Success)
            .label(positive),
        CreateButton::new("False")
            .style(serenity::ButtonStyle::Danger)
            .label(negative),
    ])])
}

pub async fn confirm_dialog_wait(
    ctx: &serenity::Context,
    message: &serenity::Message,
    author_id: serenity::UserId,
) -> Result<Option<bool>> {
    let interaction = message
        .await_component_interaction(ctx.shard.clone())
        .timeout(std::time::Duration::from_sec(60 * 5))
        .author_id(author_id)
        .await;

    if let Some(interaction) = interaction {
        interaction.defer(&ctx.http).await?;
        match &*interaction.data.custom_id {
            "True" => Ok(Some(true)),
            "False" => Ok(Some(false)),
            _ => unreachable!(),
        }
    } else {
        Ok(None)
    }
}

pub async fn confirm_dialog(
    ctx: Context<'_>,
    prompt: &str,
    positive: &str,
    negative: &str,
) -> Result<Option<bool>> {
    let builder = poise::CreateReply::default()
        .content(prompt)
        .ephemeral(true)
        .components(confirm_dialog_components(positive, negative));

    let reply = ctx.send(builder).await?;
    let message = reply.message().await?;

    confirm_dialog_wait(ctx.serenity_context(), &message, ctx.author().id).await
}
