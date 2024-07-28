use core::constants::QueryError;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::{config, select::get_table_names};

// TODO:
// needing some form of idempotency.
// some calls chain together and should
// rollback all executions/queries
// on a single failed call (should never happen?)

pub fn update_user_points(
    guild_id: serenity::GuildId,
    table_name: String,
    user_id: serenity::UserId,
    points: i64,
) -> Result<(), QueryError> {
    thread::spawn(move || {
        let mut db_client = config::init();

        let tables: Vec<String>;
        match get_table_names(guild_id) {
            Ok(v) => tables = v,
            _ => return Err(QueryError::NotFound),
        }

        for name in tables {
            if name.eq_ignore_ascii_case(&table_name) {
                if let Some(p) = db_client
                    .query(
                        &format!(
                            "
                                SELECT pts FROM t_{guild_id}_{name};\n
                            "
                        ),
                        &[],
                    )
                    .map_err(|_| QueryError::None)?
                    .first()
                {
                    let pts: i64 = p.get(0);
                    if points.checked_add(pts).is_none() {
                        return Err(QueryError::Overflow);
                    }

                    if let Ok(_) = db_client.query(
                        &format!(
                            "
                                UPDATE t_{guild_id}_{name}
                                SET pts = pts + {points}
                                WHERE uid = {user_id};\n
                            "
                        ),
                        &[],
                    ) {
                        return Ok(());
                    }
                }
            }
        }

        Err(QueryError::NotFound)
    })
    .join()
    .unwrap()
}

pub fn update_table_alias(
    guild_id: serenity::GuildId,
    old_alias: String,
    new_alias: String,
) -> Result<(), QueryError> {
    thread::spawn(move || {
        let mut db_client = config::init();
        db_client
            .execute(
                &format!(
                    "
                        UPDATE table_name_by_guild_id
                        SET t_name = \'{new_alias}\'
                        WHERE gid = {guild_id} and t_name = \'{old_alias}\';\n
                    "
                ),
                &[],
            )
            .map_err(|_| QueryError::None)
            .map(|_| {
                db_client
                    .execute(
                        &format!(
                            "
                                ALTER TABLE t_{guild_id}_{old_alias}
                                RENAME TO t_{guild_id}_{new_alias};\n
                            "
                        ),
                        &[],
                    )
                    .map_err(|_| QueryError::None)
                    .map(|_| ())
            })?
    })
    .join()
    .unwrap()
}

pub fn upsert_table_index(guild_id: serenity::GuildId, new_name: String) -> Result<(), QueryError> {
    thread::spawn(move || {
        config::init()
            .execute(
                &format!(
                    "
                        INSERT INTO table_name_by_guild_id(t_name)
                        VALUES (\'{new_name}\')
                        WHERE gid = {guild_id};\n
                    "
                ),
                &[],
            )
            .map_err(|_| QueryError::None)
            .map(|_| ())
    })
    .join()
    .unwrap()
}

pub fn insert_new_member(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    display_name: String,
) -> Result<(), QueryError> {
    thread::spawn(move || match get_table_names(guild_id) {
        Ok(r) => {
            for name in r {
                if let Err(_) = config::init().execute(
                    &format!(
                        "
                            INSERT INTO t_{guild_id}_{name}(uid, d_name, pts)
                            VALUES({user_id}, \'{display_name}\', 0);\n
                        "
                    ),
                    &[],
                ) {
                    return Err(QueryError::None);
                }
            }
            Ok(())
        }
        _ => return Err(QueryError::None),
    })
    .join()
    .unwrap()
}

pub fn update_member_name(
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    name: String,
) -> Result<(), QueryError> {
    thread::spawn(move || match get_table_names(guild_id) {
        Ok(r) => {
            for t_name in r {
                if let Err(_) = config::init().execute(
                    &format!(
                        "
                            UPDATE t_{guild_id}_{t_name}
                            SET d_name = \'{name}\'
                            WHERE uid = {user_id};\n
                        "
                    ),
                    &[],
                ) {
                    return Err(QueryError::None);
                }
            }
            Ok(())
        }
        _ => Err(QueryError::None),
    })
    .join()
    .unwrap()
}
