use serde::{Deserialize, Serialize};

use crate::{api::guild::GuildMember, common::{id::{ChannelId, GuildId, LobbyId, UserId}, timestamp::Timestamp}};

#[derive(Serialize, Deserialize)]
pub struct VoiceState {
	/// The guild ID this voice state is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Option<GuildId>>,
	/// The channel ID this user is connected to
	pub channel_id: Option<ChannelId>,
	/// The ID of the lobby this user is connected to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lobby_id: Option<LobbyId>,
	/// The user ID this voice state is for
	pub user_id: UserId,
	/// The guild member this voice state is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member: Option<GuildMember>,
	/// The session ID this voice state is from
	pub session_id: String,
	/// Whether this user is deafened by the guild, if any
	pub deaf: bool,
	/// Whether this user is muted by the guild, if any
	pub mute: bool,
	/// Whether this user is locally deafened
	pub self_deaf: bool,
	/// Whether this user is locally muted
	pub self_mute: bool,
	/// Whether this user is streaming using "Go Live"
	#[serde(skip_serializing_if = "Option::is_none")]
	pub self_stream: Option<bool>,
	/// Whether this user's camera is enabled
	pub self_video: bool,
	/// Whether this user's permission to speak is denied
	pub suppress: bool,
	/// When which the user requested to speak
	pub request_to_speak_timestamp: Option<Timestamp>,
	/// Volume level of the user (0-100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_volume: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct VoiceRegion {
	/// The unique ID for the region
	pub id: String,
	/// The name of the region
	pub name: String,
	/// Whether this is the closest to the current user's client
	pub optimal: bool,
	/// Whether this is a deprecated voice region (avoid switching to these)
	pub deprecated: bool,
	/// Whether this is a custom voice region (used for events, etc.)
	pub custom: bool,
}

