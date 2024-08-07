use core::framework;

#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: framework::Context<'_>,
    #[description = "Table name"] table_name: Option<String>,
) -> Result<(), framework::Error> {
    if table_name.is_none() {
        ctx.reply("Table name is required.").await?;
        return Ok(());
    }

    /* TODO: if this somehow ever has a decent userbase, fund upkeep with premium users
    match tasks::select::has_table(ctx.guild_id().unwrap().clone()) {
        Ok(None) => {}
        _ => {
            ctx.reply(
                "Your server already has a table made, please remove it before creating a new one.",
            )
            .await?;
            return Ok(());
        }
    }
    */

    if let Err(_) =
        tasks::create::create(table_name.clone().unwrap(), ctx.guild_id().unwrap().clone())
    {
        ctx.reply("Error creating your table. Refer to syntax, or contact the bot support.")
            .await?;
        return Ok(());
    }

    let created = tasks::create::insert_new(
        ctx.guild_id().clone().unwrap(),
        table_name.clone().unwrap(),
        ctx.guild().unwrap().members.clone(),
    );
    if created.is_err() {
        ctx.reply("Error inserting server members. Please check your command context.")
            .await?;
        return Ok(());
    }

    let t_name = table_name.unwrap();
    ctx.reply(format!("Successfully created table {t_name}!"))
        .await?;
    Ok(())
}
