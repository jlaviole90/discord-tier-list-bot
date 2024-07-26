CREATE DATABASE tier_list;
ALTER DATABASE tier_list OWNER TO postgres;

IF EXISTS (
    SELECT FROM pg_catalog.pg_roles
    WHERE ROLENAME = 'bot'
    ) THEN RAISE NOTICE 'Bot already exists. Skipping.';
ELSE
    CREATE ROLE bot LOGIN PASSWORD 'racist';
    GRANT pg_read_all_data TO bot;
    GRANT pg_write_all_data TO bot;
END IF;

CREATE TABLE IF NOT EXISTS table_name_by_guild_id (
    uid     numeric PRIMARY KEY,
    d_name  text NOT NULL,
    pts     bigint,
);

\connect tier_list

