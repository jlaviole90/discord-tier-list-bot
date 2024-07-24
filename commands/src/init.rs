use core::framework;

#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: framework::Context<'_>,
    #[description = "Table name"] name: Option<String>,
) -> Result<(), core::framework::Error> {
    // TODO: add a blocker for guilds who have already created a table

    let guild_id = ctx.guild_id().unwrap();
    let users = &ctx.guild().unwrap().members;

    let n = name.unwrap();
    let created = tasks::create_db::create(n.clone());
    if created {
        tasks::create_db::insert_new(n, guild_id, users.clone());
    }

    return Ok(());
}
