use std::{
    borrow::Cow,
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

use aformat::aformat;
pub use anyhow::{Error, Result};
use arrayvec::ArrayString;
use parking_lot::Mutex;
use serde::Deserialize as _;
use strum_macros::IntoStaticStr;
use tracing::warn;
use typesize::derive::TypeSize;

use poise::serenity_prelude::{self as serenity};
use serenity::small_fixed_array::{FixedArray, FixedString};

use crate::{analytics, bool_enum, database};

macro_rules! into_static_display {
    ($struct:ident, max_length($len:literal)) => {
            impl to_arraystring::ToArrayString for $struct {
                const MAX_LENGTH: usize: $len;
            type ArrayString = arrayvec::ArrayString<$len>;

            fn to_arraystring(self) -> Self::ArrayString {
                arrayvec::ArrayString::from(self.into()).unwrap()
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.into())
            }
        }
    };
}

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(rename = "Main")]
    pub main: MainConfig,
    #[serde(rename = "Webhook-Info")]
    pub webhooks: WebhookConfigRaw,
    #[serde(rename = "Website-Info")]
    pub website_info: Option<WebsiteInfo>,
    #[serde(rename = "Bot-List-Tokens")]
    #[serde(default)]
    pub bot_list_tokens: Option<BotListTokens>,
}

#[derive(serde::Deserialize)]
pub struct MainConfig {
    pub announcements_channel: serenity::ChannelId,
    pub service_auth_key: Option<FixedString>,
    pub invite_channel: serenity::ChannelId,
    pub website_url: Option<reqwest::Url>,
    pub main_server_invite: FixedString,
    pub main_server: serenity::GuildId,
    pub proxy_url: Option<FixedString>,
    pub ofs_role: serenity::RoleId,
    pub service: reqwest::Url,
    pub token: Option<FixedString>,
}

#[derive(serde::Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub database: String,
    pub password: String,
    pub max_connections: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct WebsiteInfo {
    pub url: reqwest::Url,
    pub stats_key: String,
}

pub struct WebhookConfig {
    pub logs: serenity::Webhook,
    pub errors: serenity::Webhook,
    pub dm_logs: serenity::Webhook,
}

pub enum FailurePoint {
    NotSubscribed(serenity::UserId),
    Guild,
}

pub struct RegexCache {
    pub replacements: [(regex::Regex, &'static str); 3],
    pub bot_mention: OnceLock<regex::Regex>,
    pub id_in_brackets: regex::Regex,
    pub emoji: regex::Regex,
}

impl RegexCache {
    pub fn new() -> Result<Self> {
        Ok(Self {
            replacements:  [
                (
                    regex::Regex::new(r"\|\|(?s:.)*?\|\|")?,
                    ". spoiler avoided.",
                ),
                (regex::Regex::new(r"```(?s:.)*?```")?, ". code block."),
                (regex::Regex::new(r"`(?s:.)*?`")?, ". code snippet."),
            ],
            id_in_brackets: regex::Regex::new(r"\((\d+)\)")?,
            emoji: regex::Regex::new(r"<(a?):([^<>]+):\d+>")?,
            bot_mention: OnceLock::new(),
        })
    }
}

pub struct Data {
    pub analytics: Arc<analytics::Hanlder>,
    pub guilds_db: database::Hanlder<i64, database::GuildRowRaw>,
    pub userinfo_db: database::Handler<i64, database::UserRowRaw>,
    pub nickname_db: database::Handler<[i64; 2], database::NicknameRowRaw>,

    pub startup_message: serenity::MessageId,
    pub system_info: Mutex<sysinfo::System>,
    pub start_time: std::time::SystemTime,
    pub reqest: reqwest::Client,
    pub regex_cache: RegexCache,
    pub webhooks: WebhookConfig,
    pub config: MainConfig,
    pub pool: sqlx::PgPool,

    pub website_info: Mutex<Option<WebsiteInfo>>,
    pub bot_list_tokens: Mutex<Option<BotListTokens>>,
    pub fully_started: std::sync::atomic::AtomicBool,
    pub update_startup_lock: tokio::sync::Mutex<()>,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data").finish()
    } 
}

pub type Command = poise::Command<Data, CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, CommandError>;
pub type PartialContext<'a> = poise::PartialContext<'a, Data, CommandError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E = Error> = Result<(), E>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;
