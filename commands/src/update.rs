use core::framework;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn update(
    ctx: framework::Context<'_>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value"] points: Option<u64>,
) -> Result<(), framework::Error> {
    if member.is_none() {
        ctx.reply("Tagged member is required.").await?;
        return Ok(());
    }

    if points.is_none() {
        ctx.reply("Points value is required.").await?;
        return Ok(());
    }

    let mem = member.unwrap();
    let res =
        tasks::upsert::upsert_user(ctx.guild_id().unwrap(), mem.id, mem.name, points.unwrap());
    if res.is_ok() {
        Ok(())
    } else {
        // TODO
        Ok(())
    }
}

//#[poise::command(slash_command, prefix_command)]
// TODO: on user joins guild event
/*
pub async fn insert(
    ctx: framework::Context<'_>,
    #[description = "New Member"] member: Option<serenity::User>,
) -> Result<(), framework::Error> {
    if member.is_none() {
        ctx.reply("New member is required.").await?;
        return Ok(())
    }

    return Ok(())
}
*/

#[poise::command(slash_command, prefix_command)]
pub async fn rename(
    ctx: framework::Context<'_>,
    #[description = "New name"] new_name: Option<String>,
) -> Result<(), framework::Error> {
    if new_name.is_none() {
        ctx.reply("New name required to rename table...").await?;
        return Ok(());
    }

    Ok(())
}
