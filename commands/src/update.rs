use core::{constants::QueryError, framework};
use poise::serenity_prelude as serenity;

const UPDATE_SYNTAX: &str = "Command syntax: update [table_name: string] [user: tagged server memeber] [points: number]";

#[poise::command(slash_command, prefix_command)]
pub async fn update(
    ctx: framework::Context<'_>,
    #[description = "Table Name"] table_name: Option<String>,
    #[description = "Tagged Member"] member: Option<serenity::User>,
    #[description = "Points value to add"] points: Option<String>,
) -> Result<(), framework::Error> {
    for c in points.clone().unwrap().chars() {
        if !c.is_numeric() || table_name.is_none() || member.is_none() || points.is_none() {
            framework::reply_syntax(ctx, UPDATE_SYNTAX).await;
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
        },
    }

    match tasks::upsert::upsert_user(ctx.guild_id().unwrap(), member.clone().unwrap().id, pts) {
        Err(QueryError::Overflow) => {
            ctx.reply("Sorry, that user can't hold that many points! Try a smaller value.").await?;
            return Ok(());
        },
        Err(QueryError::None) => {
            ctx.reply("Sorry, I failed to update the specified user. Please try again later.")
                .await?;
            return Ok(());
        },
        _ => {},
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

const RENAME_SYNTAX: &str = "Command syntax: rename [old_name: string] [new_name: string]";

#[poise::command(slash_command, prefix_command)]
pub async fn rename(
    ctx: framework::Context<'_>,
    #[description = "Old name"] old_name: Option<String>,
    #[description = "New name"] new_name: Option<String>,
) -> Result<(), framework::Error> {
    if old_name.is_none() || new_name.is_none() {
        framework::reply_syntax(ctx, RENAME_SYNTAX).await;
        return Ok(());
    }

    if let Err(_) =
        tasks::upsert::update_table_alias(ctx.guild_id().unwrap(), new_name.clone().unwrap())
    {
        ctx.reply("Encountered an error executing your request. Please try again later.")
            .await?;
        return Ok(());
    }

    let nname = new_name.unwrap();
    let oname = old_name.unwrap();
    ctx.reply(format!("Updated, {oname} is now named {nname}"))
        .await?;
    Ok(())
}
