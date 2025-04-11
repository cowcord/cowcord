#![allow(non_camel_case_types)]

use serde::Deserialize;

use super::application::Application;
use super::channel::Channel;
use super::directory::GuildScheduledEvent;
use super::guild::GuildMember;
use super::types::{Snowflake, Timestamp};
use super::user::User;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Invite {
	pub code:                       String,
	pub r#type:                     u8, // https://docs.discord.sex/resources/invite#invite-type
	pub channel:                    Option<Channel>,
	pub guild_id:                   Snowflake,
	pub guild:                      InviteGuild,
	pub inviter:                    User,
	pub flags:                      u8, // https://docs.discord.sex/resources/invite#invite-flags
	pub target_type:                u8, // https://docs.discord.sex/resources/invite#invite-target-type
	pub target_user:                User,
	pub target_application:         Application,
	pub approximate_member_count:   u32,
	pub approximate_presence_count: u32,
	pub expires_at:                 Option<Timestamp>,
	pub stage_instance:             InviteStageInstance,
	pub guild_scheduled_event:      GuildScheduledEvent,
	pub new_member:                 bool,
	pub show_verification_form:     bool,
	pub is_nickname_changeable:     bool,
}

pub enum InviteType {
	GUILD = 0,
	GROUP_DM = 1,
	FRIEND = 2,
}

pub enum InviteFlags {
	IS_GUEST_INVITE = 1 << 0,
	IS_VIEWED = 1 << 1,
	IS_ENHANCED = 1 << 2,
	IS_APPLICATION_BYPASS = 1 << 3,
}

pub enum InviteTargetType {
	STREAM = 1,
	EMBEDDED_APPLICATION = 2,
	ROLE_SUBSCRIPTIONS = 3,
	CREATOR_PAGE = 4,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct InviteMetadata {
	pub uses:       u32,
	pub max_uses:   u32,
	pub max_age:    u32,
	pub temporary:  bool,
	pub created_at: Timestamp,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct InviteGuild {
	pub id:                         Snowflake,
	pub name:                       String,
	pub icon:                       Option<String>,
	pub description:                Option<String>,
	pub banner:                     Option<String>,
	pub splash:                     Option<String>,
	pub verification_level:         u8, // https://docs.discord.sex/resources/guild#verification-level
	pub features:                   Vec<String>, // https://docs.discord.sex/resources/guild#guild-features
	pub vanity_url_code:            Option<String>,
	pub premium_subscription_count: u32,
	#[deprecated]
	pub nsfw:                       bool,
	pub nsfw_level:                 u8, // https://docs.discord.sex/resources/guild#nsfw-level
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct InviteStageInstance {
	pub members:           Vec<GuildMember>,
	pub participant_count: u16,
	pub speaker_count:     u16,
	pub topic:             String,
}
