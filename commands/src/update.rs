use core::framework;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn update(
    ctx: framework::Context<'_>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value to add"] points: Option<String>,
) -> Result<(), framework::Error> {
    if member.is_none() {
        ctx.reply("Tagged member is required.").await?;
        return Ok(());
    }

    if points.is_none() {
        ctx.reply("Points value is required.").await?;
        return Ok(());
    }

    let pts: i64;
    if let Ok(v) = points.clone().unwrap().parse::<i64>() {
        pts = v;
    } else {
        ctx.reply("Sorry! The number you entered was too large! Try something a bit smaller.")
            .await?;
        return Ok(());
    }

    if let Err(_) =
        tasks::upsert::upsert_user(ctx.guild_id().unwrap(), member.clone().unwrap().id, pts)
    {
        ctx.reply("Failed to update the specified user. Check command syntax!")
            .await?;
        return Ok(());
    }

    let name = member.clone().unwrap().name.to_string();
    let d_name = ctx
        .guild()
        .unwrap()
        .members
        .clone()
        .into_iter()
        .find(|m| m.user.name == name)
        .unwrap()
        .nick
        .unwrap()
        .to_string();
    let pts = points.unwrap();
    ctx.reply(format!("Updated {d_name}({name}) with {pts} points."))
        .await?;
    Ok(())
}

/*#[poise::command(slash_command, prefix_command)]
pub async fn subtract(
    ctx: framework::Context<'_>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value to subtract"] points: Option<
) -> Result<(), framework::Error {

}*/

#[poise::command(slash_command, prefix_command)]
pub async fn rename(
    ctx: framework::Context<'_>,
    #[description = "New name"] new_name: Option<String>,
) -> Result<(), framework::Error> {
    if new_name.is_none() {
        ctx.reply("New name required to rename table...").await?;
        return Ok(());
    }

    if let Err(_) =
        tasks::upsert::update_table_alias(ctx.guild_id().unwrap(), new_name.clone().unwrap())
    {
        ctx.reply("Encountered an error executing your request. Please try again later.")
            .await?;
        return Ok(());
    }

    let name = new_name.unwrap();
    ctx.reply(format!("Updated, tier list is now named {name}"))
        .await?;
    Ok(())
}
