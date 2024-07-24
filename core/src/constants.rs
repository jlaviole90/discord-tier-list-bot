use poise::serenity_prelude as serenity;

pub const RED: u32 = 0xff0000;
pub const NEUTRAL_COLOUR: u32 = 0x3498db;

pub const OPTION_SEPERATORS: [&str; 4] = [
    ":small_orange_diamond:",
    ":small_blue_diamond:",
    ":small_red_triangle:",
    ":str:",
];

pub const DB_SETUP_QUERY: &str = "
    CREATE TABLE userinfo (
        user_id             bigint     PRIMARY KEY,
        guild_id            bigint
        dm_blocked          bool       DEFAULT False,
        dm_welcomed         bool       DEFAULT false,
    
        FOREIGN KEY         (guild_id)
        REFERENCES guilds   (guild_id)
        ON DELETE CASCADE,
    );

    CREATE TABLE guilds (
        guild_id        bigint      PRIMARY KEY,
        channel         bigint      DEFAULT 0,
        required_role   bigint,
        bot_ignore      bool        DEFAULT True,
        auto_join       bool        DEFAULT False,
        msg_length      smallint    DEFAULT 30,
        prefix          varchar(6)  DEFAULT '-',
        required_prefix varchar(6),
    );

    CREATE TABLE nicknames (
        guild_id bigint,
        user_id  bigint,
        name     text,

        PRIMARY KEY (guild_id, user_id),

        FOREIGN KEY       (guild_id)
        REFERENCES guilds (guild_id)
        ON DELETE CASCADE,

        FOREIGN KEY         (user_id)
        REFERENCES userinfo (user_id)
        ON DELETE CASCADE
    );

    CREATE TABLE analytics (
        event          text  NOT NULL,
        count          int   NOT NULL,
        is_command     bool  NOT NULL,
        date_collected date  NOT NULL DEFAULT CURRENT_DATE,
        PRIMARY KEY (event, is_command, date_collected)
    );

    CREATE TABLE errors (
        traceback   text    PRIMARY KEY,
        message_id  bigint  NOT NULL,
        occurrences int     DEFAULT 1
    );

    INSERT INTO guilds(guild_id) VALUES(0);
    INSERT INTO userinfo(user_id) VALUES(0);
    INSERT INTO nicknames(guild_id, user_id) VALUES (0, 0);
";

pub fn get_intents() -> serenity::GatewayIntents {
    serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_PRESENCES
}

