use ::serenity::small_fixed_array::FixedString;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn upsert_user(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    points: u64,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let sel_query = format!(
            "
                SELECT g.pts from {guild_id}
                WHERE g.uid = {user_id};\n
            "
        );

        let u = db_client.query(&sel_query, &[]);
        if u.is_err() {
            return Err(());
        }

        let mut pts: i64 = u.unwrap().first().unwrap().get(0);
        pts += points as i64;

        let upsert_query = format!(
            "
                UPDATE {guild_id} g
                SET g.pts = {pts}
                WHERE g.uid = {user_id};\n
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

pub fn update_table_alias(guild_id: serenity::GuildId, new_alias: String) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        let alter_query = format!(
            "
                UPDATE table_name_by_build_id t
                SET t.t_name = \'{new_alias}\'
                WHERE gid = {guild_id};\n
            "
        );

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
                INSERT INTO table_name_by_guild_id(t_name) t
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
