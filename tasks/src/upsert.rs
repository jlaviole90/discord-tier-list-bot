use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn upsert_user(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    points: u64,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        // TODO: this can definitely be done in 1 query.
        // my SQL skills do not immediately allow that.
        let mut db_client = config::init();
        if let Ok(r) = db_client.query(
            &format!(
                "
                    SELECT g.pts from t_{guild_id} g
                    WHERE g.uid = {user_id};\n
                "
            ),
            &[],
        ) {
            let pts: i64 = r.first().unwrap().get(0);
            if let Ok(_) = db_client.execute(
                &format!(
                    "
                        UPDATE t_{}
                        SET pts = {}
                        WHERE uid = {};\n
                    ",
                    guild_id,
                    (pts + points as i64),
                    user_id
                ),
                &[],
            ) {
                return Ok(());
            }
        }
        Err(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err("Failed to upsert user. Please try again later".to_string()),
    }
}

pub fn update_table_alias(guild_id: serenity::GuildId, new_alias: String) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    UPDATE table_name_by_guild_id
                    SET t_name = \'{new_alias}\'
                    WHERE gid = {guild_id};\n
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
        Ok(Err(_)) | Err(_) => Err("Failed to alter table. Please try again later".to_string()),
    }
}

pub fn upsert_table_index(guild_id: serenity::GuildId, new_name: String) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    INSERT INTO table_name_by_guild_id(t_name)
                    VALUES ({new_name})
                    WHERE gid = {guild_id};\n
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
        Ok(Err(_)) | Err(_) => {
            Err("Failed to update table name. Please try again later.".to_string())
        }
    }
}

pub fn insert_new_member(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    display_name: String,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    INSERT INTO t_{guild_id}(uid, d_name, pts)
                    VALUES({user_id}, {display_name}, 0);\n
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
        Ok(Err(_)) | Err(_) => Err("Failed to insert new member.".to_string()),
    }
}

pub fn update_member_name(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    name: String,
) -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = config::init().execute(
            &format!(
                "
                    UPDATE t_{guild_id}
                    SET d_name = {name}
                    WHERE uid = {user_id};\n
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
        Ok(Err(_)) | Err(_) => Err(
            "Failed to update member's name. This may mean you need to reset your table..."
                .to_string(),
        ),
    }
}
