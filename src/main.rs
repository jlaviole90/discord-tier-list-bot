#![allow(stable_features)]
#![feature(let_chains)]
#![warn(
    rust_2018_idioms,
    missing_copy_implementations,
    noop_method_call,
    unused
)]
#![warn(clippy::pedantic)]
// clippy::pedantic complains about u64 -> i64 and back when db conversion, however it is fine
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
)]#![allow(stable_features)]
#![feature(let_chains)]
#![warn(
    rust_2018_idioms,
    missing_copy_implementations,
    noop_method_call,
    unused
)]
#![warn(clippy::pedantic)]
// clippy::pedantic complains about u64 -> i64 and back when db conversion, however it is fine
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

use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
};

use anyhow::Ok;
use parking_lot::Mutex;

use poise::serenity_prelude as serenity;
use serenity::small_fixed_array::FixedString;

use tts_core::{
    analytics, create_db_handler, database,
    structs::{Data, PollyVoice, RegexCache, Result, TTSMode},
};
use tts_tasks::Looper as _;

mod startup;

use startup::*;use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
};

use anyhow::Ok;
use parking_lot::Mutex;

use poise::serenity_prelude as serenity;
use serenity::small_fixed_array::FixedString;

use tts_core::{
    analytics, create_db_handler, database,
    structs::{Data, PollyVoice, RegexCache, Result, TTSMode},
};
use tts_tasks::Looper as _;

mod startup;

use startup::*;

const TOKEN: &str = "MTI2NDc1ODUyNTc3OTc3NTY2Mg.GOVs7-.QuK1FxmuH8sagW339-nyQpy4ZCIZ2OZdpk2z-kMTI2NDc1ODUyNTc3OTc3NTY2Mg.GOVs7-.QuK1FxmuH8sagW339-nyQpy4ZCIZ2OZdpk2z-k";

fn main() -> Result<()> {
    let start_time = std::time::SystemTime::now();

    println!("Starting tokio runtime");
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(_main(start_time))
}

async fn _main(start_time: std::time::SystemTime) -> Result<()> {
    println!("Loading and performing migrations");
    let (pool, config) = migrations::load_db_and_conf().await?;

    println!("Initializing HTTP client");
    let reqwest = reqwestt::Client::new();
    let auth_key = config.main.service_auth_key.as_deref();

    let mut http_builder = serenity::HttpBuilder::new(config.main.token.as_deref().unwrap());
    if let Some(proxy) = &config.main.proxy_url {
        println!("connecting via proxy");
        http_builder = http_builder
            .proxy(proxy.as_str())
            .ratelimiter_disabled(true);
    }

    let http = Arc::new(http_builder.build());

    println!("Performing startup join");
    let service = || config.main.service.clone();
    let (
        webhooks,
        guilds_db,
        userinfo_db,
        nickname_db,
    ) = tokio::try_join!(
        get_webhooks(&http, config.webhooks),
        create_db_handler!(pool.clone(), "guilds", "guild_id"),
        create_db_handler!(pool.clone(), "userinfo", "user_id"),
        create_db_handler!(pool.clone(), "nicknames", "guild_id", "user_id")
    )?;

    println!("Setting up webhook logging");
    tasks::logging::WebhookLogger::init(
        http.clone(),
        webhooks.logs.clone(),
        webhooks.errors.clone(),
    );

    println!("Sending startup message");
    let startup_message = send_startup_message(&http, &webhooks.logs).await?;

    println!("Spawning analytics handler");
    let analytics = Arc::new(analytics::Handler::new(pool.clone()));
    tokio::spawn(analytics.clone().start());

    let data = Arc::new(Data {
        pool,
        system_info: Mutex::new(sysinfo::System::new()),
        bot_list_tokens: Mutex::new(config.bot_list_tokens),

        fully_started: AtomicBool::new(false),
        update_startup_lock: tokio::sync::Mutex::new(()),

        website_info: Mutex::new(config.website_info),
        config: config.main,
        reqwest,
        analytics,
        webhooks,
        start_time,
        startup_message,
        regex_cache: RegexCache::new()?,
        guilds_db,
        userinfo_db,
        nickname_db
    });

    let framework_options = poise::FrameworkOptions {
        commands: commands::commands(),
        event_handler: |fw_ctx, event| Box::pin(events::listen(fw_ctx, event)),
        on_error: |error| {
            Box::pin(async move {
                let res = core::errors::handle(error).await;
                res.unwrap_or_else(|err| tracing::error!("on_error: {:?}", err));
            })
        },
        allowed_mentions: Some(
            serenity::CreateAllowedMentions::default()
                .replied_user(true),
                // TODO: set up role exclusions
                all_users(true),
        ),
        pre_command: analytics::pre_command,
        prefix_opptions: poise::PrefixFrameworkOptions {
            dynamic_prefix: Some(|ctx| Box::pin(commands::get_prefix(ctx))),
            ..poise::PrefixFrameworkOptions::default()
        },
        command_check: Some(|ctx| Box::pin(commands::command_check(ctx))),
        ..poise::FrameworkOptions::default()
    };

    let mut client = serenity::ClientBuilder::new_with_http(http, events::get_intents())
        .framework(poise::Framework::new(framework_options))
        .data(data as _)
        .await?;

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        wait_until_shutdown().await;

        tracing::warn!("Received control C and shutting down.");
        shard_manager.shutdown_all().await;
    });

    client.start_autosharded().await.map_err(Into::into)
}

#[cfg(unix)]
async fn wait_until_shutdown() {
    use tokio::signal::unix as signal;

    let [mut s1, mut s2, mut s3] = [
        signal::signal(signal::SignalKind::hangup()).unwrap(),
        signal::signal(signal::SignalKind::interupt()).unwrap(),
        signal::signal(signal::SignalKind::terminate()).unwrap(),
    ];

    tokio::select!(
        v = s1.recv() => v.unwrap(),
        v = s2.recv() => v.unwrap(),
        v = s3.recv() => v.unwrap(),
    );
}

#[cfg(windows)]
async fn wait_until_shutdown() {
    let (mut s1, mut s2) = (
        tokio::signal::windows::ctrl_c().unwrap(),
        tokio::signal::windows::ctrl_break().unwrap(),
    );

    tokio::select!(
        v = s1.recv() => v.unwrap()
        v = s2.recv() => v.unwrap(),
    );
}
