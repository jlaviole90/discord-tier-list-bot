use core::constants::QueryError;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn has_table(guild_id: serenity::GuildId) -> Result<Option<Vec<String>>, QueryError> {
    match thread::spawn(move || -> Result<Vec<postgres::Row>, QueryError> {
        match config::init().query(
            &format!(
                "
                    SELECT t.t_name FROM table_name_by_guild_id t
                    WHERE t.gid = {guild_id};\n
                "
            ),
            &[],
        ) {
            Ok(r) => Ok(r),
            _ => Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(r)) => {
            if r.len() == 0 {
                return Ok(None);
            }

            let mut names: Vec<String> = Vec::new();
            for n in r {
                let name: &str = n.get(0);
                names.push(name.to_string());
            }
            Ok(Some(names))
        }
        _ => Err(QueryError::None),
    }
}

pub fn get_values(
    guild_id: serenity::GuildId,
    name: String,
) -> Result<Vec<postgres::Row>, QueryError> {
    match thread::spawn(move || -> Result<Vec<postgres::Row>, QueryError> {
        match config::init().query(
            &format!(
                "
                    SELECT g.d_name, g.pts FROM t_{guild_id}_{name} g
                    ORDER BY g.pts DESC;\n
                "
            ),
            &[],
        ) {
            Ok(r) => Ok(r),
            _ => Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn get_table_names(guild_id: serenity::GuildId) -> Result<Vec<String>, QueryError> {
    match thread::spawn(move || -> Result<Vec<postgres::Row>, QueryError> {
        match config::init().query(
            &format!(
                "
                    SELECT t_name FROM table_name_by_guild_id
                    WHERE gid = {guild_id};\n
                "
            ),
            &[],
        ) {
            Ok(r) => Ok(r),
            _ => Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(r)) => {
            let mut names: Vec<String> = Vec::new();
            for row in r {
                let name: &str = row.get(0);
                names.push(name.to_string());
            }
            Ok(names)
        }
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}
