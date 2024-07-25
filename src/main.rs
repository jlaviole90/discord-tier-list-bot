#![allow(stable_features)]
#![feature(async_closure)]
#![feature(let_chains)]
#![warn(
    rust_2018_idioms,
    missing_copy_implementations,
    noop_method_call,
    unused
)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::cast_possible_truncation
)]
#![allow(
    clippy::unreadable_literal,
    clippy::wildcard_imports,
    clippy::too_many_lines,
    clippy::similar_names
)]

use core::{constants, framework};
use poise::serenity_prelude as serenity;
use dotenv::dotenv;

pub fn main() {
    let start_time = std::time::Instant::now();
    run(start_time);
}

#[tokio::main]
async fn run(start_time: std::time::Instant) {
    dotenv().ok();
    let token: String = std::env::var("DISCORD_TOKEN").expect("Discord token not found.");
    let intents = constants::get_intents();

    let data = std::sync::Arc::new(framework::Data {
        start_time,
        token: token.to_string(),
    });

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
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

