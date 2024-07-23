use std::num::NonZeroU8;

use arrayvec::ArrayString;
use typesize::derive::TypeSize;

use poise::serenity_prelude::{ChannelId, GuildId, RoleId, UserId};

fn truncate_convert<const MAX_SIZE: usize>(
    mut s: String,
    field_name: &'static str,
) -> ArrayString<MAX_SIZE> {
    if s.len() > MAX_SIZE {
        tracing::warn!("Max size of database field {field_name} reached!");
        s.truncate(MAX_SIZE);
    }

    ArrayString::from(&s).expect("Truncate to shrink to below the max size!")
}

pub trait Compact {
    type Compacted;
    fn compact(self) -> Self::Compacted;
}

#[allow(clippy::struct_excessive_bools)]
#[derive(sqlx::FromRow)]
pub struct GuildRowRaw {
    pub channel: i64,
    pub required_role: Option<i64>,
    pub auto_join: bool,
    pub bot_ignore: bool,
    pub msg_length: i16,
    pub prefix: String,
    pub required_prefix: Option<String>,
}

#[bool_to_bitflags::bool_to_bitflags(owning_setters)]
#[derive(Debug, typesize::derive::TypeSize)]
pub struct GuildRow {
    pub channel: Option<ChannelId>,
    pub required_role: Option<RoleId>,
    pub auto_join: bool,
    pub bot_ignore: bool,
    pub msg_length: u16,
    pub prefix: ArrayString<8>,
    pub required_prefix: Option<ArrayString<8>>,
}

impl Compact for GuildRowRaw {
    type Compacted = GuildRow;
    fn compact(self) -> Self::Compacted {
        Self::Compacted {
            __generated_flags: GuildRowGeneratedFlags::empty(),
            channel: (self.channel != 0).then(|| ChannelId::new(self.channel as u64)),
            required_role: self.required_role.map(|id| RoleId::new(id as u64)),
            msg_length: self.msg_length as u16,
            prefix: truncate_convert(self.prefix, "guild.prefix"),
            required_prefix: self
                .required_prefix
                .map(|t| truncate_convert(t, "guild.required_prefix")),
        }
            .set_auto_join(self.auto_join)
            .set_bot_ignore(self.bot_ignore)
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRowRaw {
    pub dm_blocked: bool,
    pub dm_welcomed: bool,
    pub bot_banned: bool,
}

#[bool_to_bitflags::bool_to_bitflags(owning_setters)]
#[derive(Debug, typesize::derive::TypeSize)]
pub struct UserRow {
    pub dm_blocked: bool,
    pub dm_welcomed: bool,
    pub bot_banned: bool
}

impl Compact for UserRowRaw {
    type Compacted = UserRow;
    fn compact(self) -> Self::Compacted {
        Self::Compacted {
            __generated_flags: UserRowGeneratedFlags::empty(),
        }
        .set_dm_blocked(self.dm_blocked)
        .set_dm_welcomed(self.dm_welcomed)
        .set_bot_banned(self.bot_banned)
    }
}

#[derive(Debug, TypeSize, sqlx::FromRow)]
pub struct NicknameRow {
    pub name: Option<String>,
}

pub type NicknameRowRaw = NicknameRow;

impl Compact for NicknameRowRaw {
    type Compacted = NicknameRow;
    fn compact(self) -> Self::Compacted {
        self
    }
}
