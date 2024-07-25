use extract_map::ExtractMap;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn create(name: String, guild_id: serenity::GuildId) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        // TODO: re-engineer for multiple tables of the same name
        let create_query = format!(
            "
                CREATE TABLE IF NOT EXISTS {name} (
                    gid             numeric,
                    uid             numeric,
                    d_name          text,
                    pts             bigint,

                    PRIMARY KEY(d_name, gid)
                );\n
            "
        );

        let r = db_client.execute(&create_query, &[]);

        if r.is_err() {
            return Err(());
        }

        let idx = index_new_table(name, guild_id);
        if idx.is_err() {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err("Failed to create new table. Please try again later".to_string()),
    }
}

pub fn insert_new(
    name: String,
    guild_id: serenity::GuildId,
    members: ExtractMap<serenity::UserId, serenity::Member>,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();
        for member in members {
            let gid = &guild_id;
            let uid = &member.user.id;
            let mem = &member.display_name();
            let ins_query = format!(
                "
                    INSERT INTO {name}(gid, uid, d_name, pts)
                    VALUES ({gid}, {uid}, \'{mem}\', 0);\n
                "
            );

            let r = db_client.execute(&ins_query, &[]);

            if r.is_err() {
                return Err(());
            }
        }
        Ok(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => Err("Failed to insert members into new table. Rolling back".to_string()),
    }
}

pub fn index_new_table(table_name: String, guild_id: serenity::GuildId) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        let mut db_client = config::init();

        let ins_query = format!(
            "
                INSERT INTO table_name_by_guild_id(gid, t_name)
                VALUES ({guild_id}, \'{table_name}\');\n
            "
        );

        let r = db_client.execute(&ins_query, &[]);

        if r.is_err() {
            Err(())
        } else {
            Ok(())
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        // TODO: idempotency
        Ok(Err(_)) | Err(_) => Err("Failed to insert new table into index. Rolling back.".to_string()),
    }
}
