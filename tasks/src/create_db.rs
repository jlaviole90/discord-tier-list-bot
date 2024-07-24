use extract_map::ExtractMap;
use poise::serenity_prelude as serenity;
use std::thread;

use crate::config;

pub fn create(name: String) -> bool {
    let res = thread::spawn(move || {
        let mut db_client = config::init();
        let create_query = format!(
            "
                CREATE TABLE IF NOT EXISTS {name} (
                    guild_id         text,
                    display_name     text,
                    points           bigint,

                    PRIMARY KEY(display_name, guild_id)
                );\n
            "
        );

        let _ = db_client
            .execute(create_query.as_str(), &[])
            .inspect_err(|why| println!("Failed to create table {why}"));
    }).join();

    res.is_ok()
}

pub fn insert_new(
    name: String,
    guild_id: serenity::GuildId,
    members: ExtractMap<serenity::UserId, serenity::Member>,
) -> bool {
    let res = thread::spawn(move || {
        let mut db_client = config::init();
        for member in members {
            let gid = &guild_id.to_string();
            let mem = &member.display_name();
            let ins_query = format!(
                "
                     
                    INSERT INTO {name}(guild_id, display_name, points)
                    VALUES (\'{gid}\', \'{mem}\', 0);\n
                "
            );

            let _ = db_client
                .execute(ins_query.as_str(), &[])
                .inspect_err(|why| println!("Failed to insert user {mem} - {why}"));
        }
    }).join();

    res.is_ok()
}
