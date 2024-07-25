use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn has_table(guild_id: serenity::GuildId) -> Option<()> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let ex_query = format!(
            "
                SELECT * FROM table_name_by_guild_id t
                WHERE t.gid = {guild_id};\n
            "
        );

        let r = db_client.query(&ex_query, &[]);

        if r.is_ok() && r.unwrap().is_empty() {
            Ok(())
        } else {
            Err(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => None,
        Ok(Err(_)) | Err(_) => Some(()),
    }
}
