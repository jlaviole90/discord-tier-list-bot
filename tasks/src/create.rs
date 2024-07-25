use extract_map::ExtractMap;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn create(name: String, guild_id: serenity::GuildId) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    CREATE TABLE IF NOT EXISTS t_{guild_id} (
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
        Err(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => {
            Err("Failed to create new table. Please try again later".to_string())
        }
    }
}

pub fn insert_new(
    guild_id: serenity::GuildId,
    members: ExtractMap<serenity::UserId, serenity::Member>,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        // This may not be completely optimal, but if one
        // fails there really isn't any point in doing the rest.
        for member in members {
            if let Err(_) = db_client.execute(
                &format!(
                    "
                        INSERT INTO t_{}(uid, d_name, pts)
                        VALUES ({}, \'{}\', 0);\n
                    ",
                    &guild_id,
                    &member.user.id,
                    &member.display_name()
                ),
                &[],
            ) {
                return Err(());
            }
        }
        Ok(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => {
            Err("Failed to insert members into new table. Rolling back".to_string())
        }
    }
}

pub fn index_new_table(table_name: String, guild_id: serenity::GuildId) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
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
            Err(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => {
            Err("Failed to insert new table into index. Rolling back.".to_string())
        }
    }
}
