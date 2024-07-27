use poise::serenity_prelude as serenity;
use core::constants::QueryError;
use std::thread;

use crate::{config, select::get_table_names};

pub fn delete_member(guild_id: serenity::GuildId, user_id: serenity::UserId) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        if let Ok(r) = get_table_names(guild_id) {
            // TODO: restructure each guild to maintain a single table of members.
            // Storing names per table is inefficient
            let mut db_client = config::init();
            for name in r {
                if let Err(_) = db_client.execute(
                    &format!(
                        "
                            DELETE FROM t_{guild_id}_{name}
                            WHERE uid = {user_id};\n
                        "
                    ),
                    &[],
                ) {
                    return Err(QueryError::None);
                }
            }
        }
        Ok(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}
