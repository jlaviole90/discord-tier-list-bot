use core::framework;
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn top(
    ctx: framework::Context<'_>,
    #[description = "Number of values to display"] num: Option<u8>,
) -> Result<(), framework::Error> {
    let mut num_cal: u8 = num.unwrap();
    if num_cal > 25 {
        ctx.reply("Sorry! Discord embed rules limit this command to 25.")
            .await?;
        num_cal = 25;
    }

    let t_name = tasks::select::has_table(ctx.guild_id().clone().unwrap());
    if let None = t_name {
        ctx.reply(
            "You haven't set up a table for your server yet! Use command \"create\" to do so!",
        )
        .await?;
        return Ok(());
    }

    let rows = tasks::select::get_values(ctx.guild_id().clone().unwrap());
    let user_rows: Vec<postgres::Row>;
    if rows.is_err() {
        ctx.reply("Data not found! Do you have any friends in your server?")
            .await?;
        return Ok(());
    } else {
        user_rows = rows.unwrap();
    }

    let mut users = Vec::new();
    for r in user_rows {
        let name: &str = r.get(0);
        let value: i64 = r.get(1);
        if value != 0 {
            users.push(Field {
                name: name.to_string(),
                value: value.to_string(),
                inline: false,
            });
        }
    }

    let emb = serenity::CreateEmbed::new()
        .title(t_name.unwrap())
        // Let user specify a color
        .color(0xff0000)
        .fields(
            users
                .into_iter()
                .take(num_cal.into())
                .map(|f| (f.name, f.value, f.inline)),
        );

    ctx.send(CreateReply::default().embed(emb)).await?;
    Ok(())
}

struct Field {
    name: String,
    value: String,
    inline: bool,
}
