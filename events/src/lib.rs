use poise::serenity_prelude as serenity;
use serenity::FullEvent as Event;

use core::framework;

pub async fn listen(ctx: framework::Context<'_>, event: &Event) -> Result<(), ()> {
    match event {
        Event::GuildMemberAddition { new_member } => add_new_member(ctx, new_member).await,
        Event::GuildMemberUpdate {
            old_if_available: _,
            new,
            event: _,
        } => update_member(ctx, new).await,
        Event::GuildMemberRemoval {
            guild_id: _,
            user,
            member_data_if_available: _,
        } => remove_member(ctx, user).await,
        Event::Message { new_message } => check_for_key_words(ctx, new_message).await,
        _ => Ok(()),
    }
}

async fn add_new_member(ctx: framework::Context<'_>, member: &serenity::Member) -> Result<(), ()> {
    if let Err(_) = tasks::upsert::insert_new_member(
        ctx.guild_id().unwrap(),
        member.user.id.clone(),
        member.display_name().to_string(),
    ) {
        let _ = ctx
            .say(format!(
                "Failed to insert new member {} into table...",
                member.display_name()
            ))
            .await;
        Err(())
    } else {
        Ok(())
    }
}

async fn update_member(
    ctx: framework::Context<'_>,
    new: &Option<serenity::Member>,
) -> Result<(), ()> {
    if let Err(_) = tasks::upsert::update_member_name(
        ctx.guild_id().unwrap(),
        new.clone().unwrap().user.id,
        new.clone().unwrap().display_name().to_string(),
    ) {
        let _ = ctx
            .say(format!("Failed to update {}'s nickname...", {
                new.clone().unwrap().display_name()
            }))
            .await;
        Err(())
    } else {
        Ok(())
    }
}

async fn remove_member(ctx: framework::Context<'_>, member: &serenity::User) -> Result<(), ()> {
    if let Err(_) = tasks::delete::delete_member(ctx.guild_id().unwrap(), member.id) {
        let _ = ctx
            .say(format!(
                "Failed to remove {} after they left the server...",
                member.name
            ))
            .await;
        Err(())
    } else {
        Ok(())
    }
}

async fn check_for_key_words(
    _ctx: framework::Context<'_>,
    _message: &serenity::Message,
) -> Result<(), ()> {
    // TODO
    Ok(())
}
