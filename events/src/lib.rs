use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Result;
use serenity::FullEvent as Event;

type Error = Box<dyn std::error::Error + Send + Sync>;

use core::framework;

pub async fn listen(_ctx: framework::FrameworkContext<'_>, event: &Event) -> Result<(), Error> {
    match event {
        Event::GuildMemberAddition { new_member } => add_new_member(new_member).await,
        Event::GuildMemberUpdate {
            old_if_available: _,
            new,
            event: _,
        } => update_member(new).await,
        Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _,
        } => remove_member(guild_id, user).await,
        Event::Message { new_message } => check_for_key_words(new_message).await,
        _ => Ok(()),
    }
}

async fn add_new_member(member: &serenity::Member) -> Result<(), Error> {
    let _ = tasks::upsert::insert_new_member(
        member.guild_id,
        member.user.id.clone(),
        member.display_name().to_string(),
    );
    Ok(())
}

async fn update_member(new: &Option<serenity::Member>) -> Result<(), Error> {
    let _ = tasks::upsert::update_member_name(
        new.clone().unwrap().guild_id,
        new.clone().unwrap().user.id,
        new.clone().unwrap().display_name().to_string(),
    );
    Ok(())
}

async fn remove_member(guild_id: &serenity::GuildId, member: &serenity::User) -> Result<(), Error> {
    let _ = tasks::delete::delete_member(guild_id.clone(), member.id);
    Ok(())
}

async fn check_for_key_words(_message: &serenity::Message) -> Result<(), Error> {
    // TODO
    // search for key words, and log the messages
    Ok(())
}
