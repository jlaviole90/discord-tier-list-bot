use core::framework;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn update(
    ctx: framework::Context<'_>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value to add"] points: Option<u64>,
) -> Result<(), framework::Error> {
    if member.is_none() {
        ctx.reply("Tagged member is required.").await?;
        return Ok(());
    }

    if points.is_none() {
        ctx.reply("Points value is required.").await?;
        return Ok(());
    }

    if let Err(_) =
        tasks::upsert::upsert_user(ctx.guild_id().unwrap(), member.unwrap().id, points.unwrap())
    {
        ctx.reply("").await?;
        return Ok(());
    }

    Ok(())
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

    if let Err(_) = tasks::upsert::update_table_alias(ctx.guild_id().unwrap(), new_name.unwrap()) {
        ctx.reply("Encountered an error executing your request. Please try again later.")
            .await?;
        return Ok(());
    }

    Ok(())
}
