use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use dotenv::dotenv;

pub fn init() -> Client {
    dotenv().ok();
    let mut agent_builder =
        SslConnector::builder(SslMethod::tls()).expect("Failed to build SSL connection");

    agent_builder
        .set_ca_file("/etc/ssl/cert.pem")
        .expect("Could not find database cert. Did you add it to the project?");

    let connector = postgres_openssl::MakeTlsConnector::new(agent_builder.build());

    Client::configure()
        .host(std::env::var("PGHOST").expect("PG hostname not found").as_str())
        .user(std::env::var("PGUSER").expect("PG username not found").as_str())
        .password(std::env::var("PGPASS").expect("PG password not found").as_str())
        .dbname(std::env::var("PGDBNAME").expect("PG database not found").as_str())
        .connect(connector)
        .expect("Failed top initialize DB client")
}
