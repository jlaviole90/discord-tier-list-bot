#![allow(stable_features)]
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
    let token = std::env::var("DISCORD_TOKEN").expect("Discord token not found.");
    let intents = constants::get_intents();

    let data = std::sync::Arc::new(framework::Data {
        start_time,
        token: token.to_string(),
    });

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::init::create()],
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token.as_str(), intents)
        .framework(framework)
        .data(data)
        .await;

    client.unwrap().start().await.unwrap();
}

