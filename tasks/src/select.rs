use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn has_table(guild_id: serenity::GuildId) -> Option<String> {
    match thread::spawn(move || -> Result<(), postgres::Row> {
        let mut db_client = config::init();
        let ex_query = format!(
            "
                SELECT t.t_name FROM table_name_by_guild_id t
                WHERE t.gid = {guild_id};\n
            "
        );

        let r = db_client.query(&ex_query, &[]);
        if r.is_err() { return Ok(()); }

        let row = r.unwrap();
        if row.is_empty() {
            Ok(())
        } else {
            Err(row.first().unwrap().clone())
        }
    })
    .join()
    {
        Ok(Ok(_)) => None,
        Ok(Err(r)) => Some(r.get(0)),
        Err(_) => None,
    }
}

pub fn get_values(guild_id: serenity::GuildId) -> Result<Vec<postgres::Row>, String> {
    match thread::spawn(move || -> Result<Vec<postgres::Row>, ()> {
        let mut db_client = config::init();
        let sel_query = format!(
            "
                SELECT t.d_name, t.pts FROM {guild_id} t
                ORDER BY t.pts DESC;\n
            "
        );

        let r = db_client.query(&sel_query, &[]);
        if r.is_err() { 
            Err(()) 
        } else {
            Ok(r.unwrap())
        }
    })
    .join()
    {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(_)) | Err(_) => Err("Values not found".to_string())
    }
}
