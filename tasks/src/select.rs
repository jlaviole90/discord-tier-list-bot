use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn has_table(guild_id: serenity::GuildId) -> Option<String> {
    match thread::spawn(move || -> Option<postgres::Row> {
        if let Ok(r) = config::init().query(
            &format!(
                "
                    SELECT t.t_name FROM table_name_by_guild_id t
                    WHERE t.gid = {guild_id};\n
                "
            ),
            &[],
        ) {
            if r.first().is_some() {
                return Some(r.first().unwrap().clone());
            }
        }
        None
    })
    .join()
    {
        Ok(None) => None,
        Ok(Some(r)) => Some(r.get(0)),
        Err(_) => None,
    }
}

pub fn get_values(guild_id: serenity::GuildId) -> Result<Vec<postgres::Row>, String> {
    match thread::spawn(move || -> Result<Vec<postgres::Row>, ()> {
        if let Ok(r) = config::init().query(
            &format!(
                "
                    SELECT g.d_name, g.pts FROM t_{guild_id} g
                    ORDER BY g.pts DESC;\n
                "
            ),
            &[],
        ) {
            Ok(r)
        } else {
            Err(())
        }
    })
    .join()
    {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(_)) | Err(_) => Err("Values not found".to_string()),
    }
}
