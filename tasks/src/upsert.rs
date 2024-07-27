use core::constants::QueryError;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::{config, select::get_table_names};

pub fn upsert_user(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    points: i64,
) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        let mut db_client = config::init();

        let tables: Vec<String>;
        match get_table_names(guild_id) {
            Ok(v) => tables = v,
            _ => return Err(QueryError::NotFound),
        }

        for name in tables {
            match db_client.query(
                &format!(
                    "
                        SELECT pts FROM t_{guild_id}_{name};\n
                    "
                ),
                &[],
            ) {
                Ok(v) => match v.first() {
                    Some(p) => {
                        let pts: i64 = p.get(0);
                        if let None = points.checked_add(pts) {
                            return Err(QueryError::Overflow);
                        }
                    }
                    _ => return Err(QueryError::None),
                },
                _ => return Err(QueryError::None),
            }

            match db_client.query(
                &format!(
                    "
                        UPDATE t_{}_{}
                        SET pts = pts + {}
                        WHERE uid = {};\n
                    ",
                    guild_id, name, points, user_id
                ),
                &[],
            ) {
                Ok(_) => return Ok(()),
                _ => return Err(QueryError::None),
            }
        }

        Ok(())
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(QueryError::Overflow)) => Err(QueryError::Overflow),
        Ok(Err(QueryError::NotFound)) => Err(QueryError::NotFound),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn update_table_alias(
    guild_id: serenity::GuildId,
    old_alias: String,
    new_alias: String,
) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        match config::init().execute(
            &format!(
                "
                    UPDATE table_name_by_guild_id
                    SET t_name = \'{new_alias}\'
                    WHERE gid = {guild_id} and t_name = \'{old_alias}\';\n
                "
            ),
            &[],
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn upsert_table_index(guild_id: serenity::GuildId, new_name: String) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        match config::init().execute(
            &format!(
                "
                    INSERT INTO table_name_by_guild_id(t_name)
                    VALUES ({new_name})
                    WHERE gid = {guild_id};\n
                "
            ),
            &[],
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn insert_new_member(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    display_name: String,
) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        match get_table_names(guild_id) {
            Ok(r) => {
                for name in r {
                    match config::init().execute(
                        &format!(
                            "
                                INSERT INTO t_{guild_id}_{name}(uid, d_name, pts)
                                VALUES({user_id}, \'{display_name}\', 0);\n
                            "
                        ),
                        &[],
                    ) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err(QueryError::None),
                    }
                }
                Err(QueryError::None)
            }
            _ => return Err(QueryError::None),
        }
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}

pub fn update_member_name(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    name: String,
) -> Result<(), QueryError> {
    match thread::spawn(move || -> Result<(), QueryError> {
        match get_table_names(guild_id) {
            Ok(r) => {
                for t_name in r {
                    if let Ok(_) = config::init().execute(
                        &format!(
                            "
                                UPDATE t_{guild_id}_{t_name}
                                SET d_name = \'{name}\'
                                WHERE uid = {user_id};\n
                            "
                        ),
                        &[],
                    ) {
                        return Ok(());
                    } else {
                        return Err(QueryError::None);
                    }
                }
            }
            _ => return Err(QueryError::None),
        }

        Err(QueryError::None)
    })
    .join()
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(QueryError::None),
    }
}
