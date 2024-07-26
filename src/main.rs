use core::{constants, framework};
use poise::serenity_prelude as serenity;
use tasks::config;

pub fn main() {
    let start_time = std::time::Instant::now();
    run(start_time);
}

#[tokio::main]
async fn run(start_time: std::time::Instant) {
    let token: String = std::env::var("DISCORD_TOKEN").expect("Discord token not found.");
    let intents = constants::get_intents();

    let data = std::sync::Arc::new(framework::Data {
        start_time,
        token: token.to_string(),
    });

    if let Err(_) = config::create_index_if_not() {
        panic!("Critical failure initializing databases. Shutting down");
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event| Box::pin(events::listen(ctx, event)),
            commands: vec![
                commands::age(), // Test command
                commands::init::create(), // Create table for server
                commands::update::update(), // Update user points for server
                commands::update::rename(), // Rename table
                commands::get::top(), // Display stats
            ],
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .data(data)
        .await;

    client.unwrap().start().await.unwrap();
}

