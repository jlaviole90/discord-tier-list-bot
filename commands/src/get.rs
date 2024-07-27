use core::framework;
use poise::{serenity_prelude as serenity, CreateReply};

const TOP_SYNTAX: &str = "Command syntax: top [number: number]";

#[poise::command(slash_command, prefix_command)]
pub async fn top(
    ctx: framework::Context<'_>,
    #[description = "Number of values to display"] num: Option<u8>,
) -> Result<(), framework::Error> {
    if num.is_none() {
        framework::reply_syntax(ctx, TOP_SYNTAX).await;
        return Ok(());
    }

    let mut num_cal: u8 = num.unwrap();
    if num_cal > 25 {
        ctx.reply("Sorry! Discord embed rules limit this command to 25.")
            .await?;
        num_cal = 25;
    }

    let t_name: String;
    match tasks::select::has_table(ctx.guild_id().clone().unwrap()) {
        Some(name) => t_name = name,
        None => {
            ctx.reply(
                "You haven't set up a table for your server yet! Use command \"create\" to do so!",
            )
            .await?;
            return Ok(());
        }
    }

    let user_rows: Vec<postgres::Row>;
    match tasks::select::get_values(ctx.guild_id().clone().unwrap(), t_name.clone()) {
        Ok(rows) => user_rows = rows,
        Err(_) => {
            ctx.reply("Data not found! Your table may be corrupted.")
                .await?;
            return Ok(());
        }
    }

    let mut users = Vec::new();
    for r in user_rows {
        users.push(Field {
            name: r.get(0),
            value: r.get(1),
            inline: false,
        });
    }

    let emb = serenity::CreateEmbed::new()
        .title(t_name)
        // TODO: let user specify a color
        .color(0xff0000)
        .fields(
            users
                .into_iter()
                .take(num_cal.into())
                .filter(|f| f.value != 0)
                .map(|f| (f.name, f.value.to_string(), f.inline)),
        );

    ctx.send(CreateReply::default().embed(emb)).await?;
    Ok(())
}

struct Field {
    name: String,
    value: i64,
    inline: bool,
}
