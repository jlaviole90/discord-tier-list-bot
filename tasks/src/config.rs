use postgres::{config::SslMode, Client, NoTls};
use std::thread;

pub fn init() -> Client {
    Client::configure()
        .host(
            std::env::var("PGHOST")
                .expect("PG hostname not found")
                .as_str(),
        )
        .port(
            std::env::var("PGPORT")
                .expect("PG port not found.")
                .parse::<u16>()
                .unwrap(),
        )
        .user(
            std::env::var("PGUSER")
                .expect("PG username not found")
                .as_str(),
        )
        .password(
            std::env::var("PGPASS")
                .expect("PG password not found")
                .as_str(),
        )
        .dbname(
            std::env::var("PGDBNAME")
                .expect("PG database not found")
                .as_str(),
        )
        .ssl_mode(SslMode::Disable)
        .connect(NoTls)
        .expect("Failed to initialize DB client")
}

pub fn create_index_if_not() -> Result<(), String> {
    match thread::spawn(move || -> Result<(), ()> {
        if let Ok(_) = init().execute(
            &format!(
                "
                    CREATE TABLE IF NOT EXISTS table_name_by_guild_id (
                        gid     numeric,
                        t_name  text,

                        PRIMARY KEY(gid)
                    );\n
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
        Ok(Err(_)) | Err(_) => Err("Failed to create index table.".to_string()),
    }
}
