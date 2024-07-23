use poise::serenity_prelude as serenity;
use reqwest::StatusCode;

use core::{
    common::{confirm_dialog_components, confirm_dialog_wait},
    constants::NEUTRAL_COLOUR,
    structs::{Data, FrameworkContext, Result},
};

fn is_guild_owner(cache: &serenity::Cache, user_id: serenity::UserId) -> bool {
    cache
        .guilds()
        .into_iter()
        .find_map(|id| cache.guild(id).map(|g| g.owner_id == user_id))
        .unwrap_or(false)
}

async fn add_ofs_role(data: &Data, http: &serenity::Http, user_id: serenity::UserId) -> Result<()> {
    match http
        .add_member_role(data.config.main_server, user_id, data.config.ofs_role, None)
        .await
    {
        // Unknown member
        Err(serenity::Error::Http(serenity::HttpError::UnsuccessfulRequest(err)))
            if err.error.code == 10007 =>
        {
            Ok(())
        }

        r => r.map_err(Into::into),
    }
}

pub async fn guild_member_addition(
    framework_ctx: FrameworkContext<'_>,
    member: &serenity::Member,
) -> Result<()> {
    let data = framework_ctx.user_data();
    let ctx = framework_ctx.serenity_context;

    if member.guild_id == data.config.main_server && is_guild_owner(&ctx.cache, member.user.id) {
        add_ofs_role(&data, &ctx.http, member.user.id).await?;
    }

    Ok(())
}

