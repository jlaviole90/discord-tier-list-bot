use core::constants::QueryError;
use extract_map::ExtractMap;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn create(name: String, guild_id: serenity::GuildId) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        let mut db_client = config::init();
        if let Ok(r) = db_client.query(
            &format!(
                "
                    SELECT * FROM table_name_by_guild_id
                    WHERE gid = {guild_id} AND t_name = {name};\n
                "
            ),
            &[],
        ) {
            for row in r {
                let t_name: &str = row.get(1);
                if t_name.eq(name.as_str()) {
                    return Err(QueryError::Exists);
                }
            }
        }
        if let Ok(_) = db_client.execute(
            &format!(
                "
                    CREATE TABLE IF NOT EXISTS t_{guild_id}_{name} (
                        uid             numeric,
                        d_name          text,
                        pts             bigint,

                        PRIMARY KEY(uid)
                    );\n
                "
            ),
            &[],
        ) {
            if let Ok(_) = index_new_table(name, guild_id) {
                return Ok(());
            }
        }
        Err(QueryError::None)
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(QueryError::Exists)) => Err(QueryError::Exists),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn insert_new(
    guild_id: serenity::GuildId,
    table_name: String,
    members: ExtractMap<serenity::UserId, serenity::Member>,
) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        let mut db_client = config::init();
        // This may not be completely optimal, but if one
        // fails there really isn't any point in doing the rest.
        for member in members {
            if let Err(_) = db_client.execute(
                &format!(
                    "
                        INSERT INTO t_{}_{}(uid, d_name, pts)
                        VALUES ({}, \'{}\', 0);\n
                    ",
                    &guild_id,
                    &table_name,
                    &member.user.id,
                    &member.display_name()
                ),
                &[],
            ) {
                return Err(QueryError::None);
            }
        }
        Ok(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => {
            Err(QueryError::None)
        }
    }
}

pub fn index_new_table(table_name: String, guild_id: serenity::GuildId) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    INSERT INTO table_name_by_guild_id(gid, t_name)
                    VALUES ({guild_id}, \'{table_name}\');\n
                "
            ),
            &[],
        ) {
            Ok(())
        } else {
            Err(QueryError::None)
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => {
            Err(QueryError::None)
        }
    }
}
