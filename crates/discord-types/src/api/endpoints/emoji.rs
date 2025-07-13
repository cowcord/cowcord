#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::api::discovery::DiscoverableGuild;
use crate::api::emoji::{Emoji, EmojiApplication, EmojiGuild, TopEmoji};
use crate::api::types::{CdnUri, Snowflake};

/// Type: get
pub fn GET_GUILD_EMOJIS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/emojis", guild_id)
}

pub type GetGuildEmojisResponse = Vec<Emoji>;

/// Type: get
pub fn GET_GUILD_EMOJI_ENDPOINT(
	guild_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

pub type GetGuildEmojiResponse = Emoji;

/// Type: get
pub fn GET_GUILD_TOP_EMOJIS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/top-emojis", guild_id)
}

#[derive(Deserialize)]
pub struct GetGuildTopEmojisResponse {
	pub items: Vec<TopEmoji>,
}

/// Type: get
pub fn GET_EMOJI_GUILD_ENDPOINT(emoji_id: Snowflake) -> String {
	format!("/emojis/{}/guild", emoji_id)
}

pub type GetEmojiGuildResponse = DiscoverableGuild;

/// Type: get
pub fn GET_EMOJI_SOURCE_ENDPOINT(emoji_id: Snowflake) -> String {
	format!("/emojis/{}/source", emoji_id)
}

#[derive(Deserialize)]
pub struct GetEmojiSourceResponse {
	/// https://docs.discord.food/resources/emoji#emoji-source-type
	pub r#type:      String,
	pub guild:       Option<EmojiGuild>,
	pub application: Option<EmojiApplication>,
}

/// Type: post
/// supports the X-Audit-Log-Reason header
/// requires CREATE_EXPRESSIONS permission
pub fn CREATE_GUILD_EMOJI_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/emojis", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateGuildEmojiRequest {
	pub name:  String,
	pub image: CdnUri,
	pub roles: Vec<Snowflake>,
}

pub type CreateGuildEmojiResponse = Emoji;

/// Type: patch
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_EXPRESSIONS permission, if the emoji was created byt the user though, CREATE_EXPRESSIONS permission works too
pub fn MODIFY_GUILD_EMOJI_ENDPOINT(
	guild_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyGuildEmojiRequest {
	pub name:  String,
	pub roles: Option<Vec<Snowflake>>,
}

pub type ModifyGuildEmojiResponse = Emoji;

/// Type: delete
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_EXPRESSIONS permission, if the emoji was created byt the user though, CREATE_EXPRESSIONS permission works too
pub fn REMOVE_GUILD_EMOJI_ENDPOINT(
	guild_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

/// Type: get
pub fn GET_APPLICATION_EMOJIS_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/emojis", application_id)
}

#[derive(Deserialize)]
pub struct GetApplicationEmojisResponse {
	pub items: Vec<Emoji>,
}

/// Type: get
pub fn GET_APPLICATION_EMOJI_ENDPOINT(
	application_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}

pub type GetApplicationEmojiResponse = Emoji;

/// Type: post
pub fn CREATE_APPLICATION_EMOJI_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/emojis", application_id)
}

#[derive(Serialize)]
pub struct CreateApplicationEmojiRequest {
	pub name:  String,
	pub image: CdnUri,
}

pub type CreateApplicationEmojiResponse = Emoji;

/// Type: patch
pub fn MODIFY_APPLICATION_EMOJI_ENDPOINT(
	application_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyApplicationEmojiRequest {
	pub name: String,
}

pub type ModifyApplicationEmojiResponse = Emoji;

/// Type: delete
pub fn DELETE_APPLICATION_EMOJI_ENDPOINT(
	application_id: Snowflake,
	emoji_id: Snowflake,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}
