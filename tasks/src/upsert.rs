use ::serenity::small_fixed_array::FixedString;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn upsert_user(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    user_name: FixedString<u8>,
    points: u64,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let upsert_query = format!(
            "
                INSERT INTO (
                    SELECT t_name
                    FROM table_name_by_guild_id t
                    WHERE t.gid = {guild_id}
                )(uid, d_name, pts)
                VALUES ({user_id}, {user_name}, {points});\n
            "
        );

        let r = db_client.execute(&upsert_query, &[]);

        if r.is_err() {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err("Failed to upsert user. Please try again later".to_string()),
    }
}

pub fn alter_table_name(old: String, new: String) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let alter_query = format!("ALTER TABLE {old} RENAME TO {new};\n");

        let r = db_client.execute(&alter_query, &[]);

        if r.is_err() {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err("Failed to alter table. Please try again later".to_string()),
    }
}

pub fn upsert_table_index(guild_id: serenity::GuildId, new_name: String) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let upsert_query = format!(
            "
                INSERT INTO table_name_by_guild_id(table_name) t
                VALUES ({new_name})
                WHERE t.gid = {guild_id};\n
            "
        );

        let r = db_client.execute(&upsert_query, &[]);

        if r.is_err() {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => {
            Err("Failed to update table name. Please try again later.".to_string())
        }
    }
}
