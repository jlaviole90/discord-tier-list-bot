use core::constants::QueryError;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn has_table(guild_id: serenity::GuildId) -> Result<Option<Vec<String>>, QueryError> {
    thread::spawn(move || {
        config::init()
            .query(
                &format!(
                    "
                        SELECT t.t_name FROM table_name_by_guild_id t
                        WHERE t.gid = {guild_id};\n
                    "
                ),
                &[],
            )
            .map_err(|_| QueryError::None)
            .map(|r| {
                if r.len() == 0 {
                    return None;
                }

                let mut names: Vec<String> = Vec::new();
                for n in r {
                    let name: &str = n.get(0);
                    names.push(name.to_string());
                }
                Some(names)
            })
    })
    .join()
    .unwrap()
}

pub fn get_values(
    guild_id: serenity::GuildId,
    name: String,
) -> Result<Vec<postgres::Row>, QueryError> {
    thread::spawn(move || {
        config::init()
            .query(
                &format!(
                    "
                        SELECT g.d_name, g.pts FROM t_{guild_id}_{name} g
                        ORDER BY g.pts DESC;\n
                    "
                ),
                &[],
            )
            .map_err(|_| QueryError::None)
    })
    .join()
    .unwrap()
}

pub fn get_table_names(guild_id: serenity::GuildId) -> Result<Vec<String>, QueryError> {
    thread::spawn(move || {
        config::init()
            .query(
                &format!(
                    "
                        SELECT t_name FROM table_name_by_guild_id
                        WHERE gid = {guild_id};\n
                    "
                ),
                &[],
            )
            .map_err(|_| QueryError::None)
            .map(|r| {
                let mut names: Vec<String> = Vec::new();
                for row in r {
                    let name: &str = row.get(0);
                    names.push(name.to_string());
                }
                names
            })
    })
    .join()
    .unwrap()
}
