use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn delete_member(guild_id: serenity::GuildId, user_id: serenity::UserId) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Err(_) = config::init().execute(
            &format!(
                "
                    DELETE FROM t_{guild_id}
                    WHERE uid = {user_id};\n
                "
            ),
            &[],
        ) {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err("Failed to remove member from guild table.".to_string()),
    }
}
