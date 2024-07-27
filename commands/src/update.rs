use core::framework;
use poise::serenity_prelude as serenity;
use tasks::upsert::ValueError;

#[poise::command(slash_command, prefix_command)]
pub async fn update(
    ctx: framework::Context<'_>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value to add"] points: Option<String>,
) -> Result<(), framework::Error> {
    if member.is_none() {
        ctx.reply("Tag a member before specifying a point value!")
            .await?;
        return Ok(());
    }

    if points.is_none() {
        ctx.reply("Points value is required!").await?;
        return Ok(());
    }

    for c in points.clone().unwrap().chars() {
        if !c.is_numeric() {
            ctx.reply("Points can only be a number!").await?;
            return Ok(());
        }
    }
    

    let pts: i64;
    match points.clone().unwrap().parse::<i64>() {
        Ok(v) => pts = v,
        Err(_) => {
            ctx.reply("Sorry! The number you entered was too large! Try a smaller value.")
                .await?;
            return Ok(());
        }
    }

    match tasks::upsert::upsert_user(ctx.guild_id().unwrap(), member.clone().unwrap().id, pts) {
        Err(ValueError::OVERFLOW) => {
            ctx.reply("Sorry! That many points can't be added to the specified user. Try a smaller value.").await?;
            return Ok(());
        }
        Err(ValueError::NONE) => {
            ctx.reply("Failed to update the specified user. Check command syntax!")
                .await?;
            return Ok(());
        }
        Ok(_) => {}
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
