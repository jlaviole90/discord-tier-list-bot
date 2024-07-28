use core::constants::QueryError;
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

pub fn create_root_if_not() -> Result<(), QueryError> {
    thread::spawn(|| {
        let mut db_client = init();

        let _ = db_client.execute(&format!("CREATE DATABASE root;\n"), &[]);

        let _ = db_client.execute(
            &format!("CREATE ROLE root LOGIN PASSWORD 'p@$$w0rd';\n"),
            &[],
        );

        if let Err(_) = db_client.execute(
            &format!(
                "
                    CREATE TABLE IF NOT EXISTS table_name_by_guild_id (
                        gid     numeric,
                        t_name  text,

                        PRIMARY KEY(gid, t_name)
                    );\n
                "
            ),
            &[],
        ) {
            return Err(QueryError::None);
        }

        Ok(())
    })
    .join()
    .unwrap()
}
