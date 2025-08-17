use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::discovery::DiscoverableGuild;
use crate::api::emoji::{Emoji, EmojiApplication, EmojiGuild, TopEmoji};
use crate::common::id::{ApplicationId, EmojiId, GuildId, RoleId};
use crate::common::image::ImageHash;

/// Method: `GET`
///
/// Returns a list of [emoji](https://docs.discord.food/resources/emoji#emoji-object) objects for the given guild.
/// Includes the user field if the user has the `CREATE_EXPRESSIONS` or `MANAGE_EXPRESSIONS` permission.
pub fn GET_GUILD_EMOJIS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/emojis", guild_id)
}

pub type GetGuildEmojis = Vec<Emoji>;

/// Method: `GET`
///
/// Returns an [emoji](https://docs.discord.food/resources/emoji#emoji-object) object for the given guild and emoji IDs.
/// Includes the user field if the user has the `CREATE_EXPRESSIONS` or `MANAGE_EXPRESSIONS` permission.
pub fn GET_GUILD_EMOJI(
	guild_id: &GuildId,
	emoji_id: &EmojiId,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

pub type GetGuildEmoji = Emoji;

/// Method: `GET`
///
/// Returns the most-used emojis for the given guild.
pub fn GET_GUILD_TOP_EMOJIS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/top-emojis", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildTopEmojisResponse {
	/// The most-used emojis for the guild
	pub items: Vec<TopEmoji>,
}

/// Method: `GET`
///
/// This endpoint requires the guild to be discoverable,
/// not be [auto-removed](https://docs.discord.food/resources/discovery#discoverable-guild-object),
/// and have [guild expression discoverability](https://docs.discord.food/resources/discovery#discovery-metadata-object) enabled.
///
/// Returns a [discoverable guild](https://docs.discord.food/resources/discovery#discoverable-guild-object) object for the guild that owns the given emoji.
pub fn GET_EMOJI_GUILD(emoji_id: &EmojiId) -> String {
	format!("/emojis/{}/guild", emoji_id)
}

pub type GetEmojiGuild = DiscoverableGuild;

/// Method: `GET`
///
/// If the source is a guild, this endpoint requires the guild to be discoverable,
/// not be [auto-removed](https://docs.discord.food/resources/discovery#discoverable-guild-object),
/// and have [guild expression discoverability](https://docs.discord.food/resources/discovery#discovery-metadata-object) enabled.
///
/// Returns an object containing information on the guild or application that owns the given emoji.
pub fn GET_EMOJI_SOURCE(emoji_id: &EmojiId) -> String {
	format!("/emojis/{}/source", emoji_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetEmojiSourceResponse {
	/// The type of emoji source
	pub r#type: String,
	/// The guild that owns the given emoji
	pub guild: Option<EmojiGuild>,
	/// The application that owns the given emoji
	pub application: Option<EmojiApplication>,
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new emoji for the guild. Requires the `CREATE_EXPRESSIONS` permission.
///
/// Emojis have a maximum file size of 256 KiB. Attempting to upload an emoji larger than this limit will fail with a 400 bad request.
///
/// Returns the new [emoji](https://docs.discord.food/resources/emoji#emoji-object) object on success. Fires a [Guild Emojis Update](https://docs.discord.food/topics/gateway-events#guild-emojis-update) Gateway event.
pub fn CREATE_GUILD_EMOJI(guild_id: &GuildId) -> String {
	format!("/guilds/{}/emojis", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildEmojiRequest {
	/// The name of the emoji
	/// (2-32 characters)
	pub name: ArrayString<32>,
	/// 128x128 emoji image
	pub image: ImageHash,
	/// The roles allowed to use this emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<RoleId>>,
}

pub type CreateGuildEmojiResponse = Emoji;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies the given emoji.
///
/// For emojis created by the current user, requires either the `CREATE_EXPRESSIONS` or `MANAGE_EXPRESSIONS` permission.
/// For other emojis, requires the `MANAGE_EXPRESSIONS` permission.
///
/// Returns the updated [emoji](https://docs.discord.food/resources/emoji#emoji-object) object on success.
/// Fires a [Guild Emojis Update](https://docs.discord.food/topics/gateway-events#guild-emojis-update) Gateway event.
pub fn MODIFY_GUILD_EMOJI(
	guild_id: &GuildId,
	emoji_id: &EmojiId,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildEmojiRequest {
	/// The name of the emoji (2-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<32>>,
	/// The roles allowed to use this emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Option<Vec<RoleId>>>,
}

pub type ModifyGuildEmojiResponse = Emoji;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Deletes the given emoji.
///
/// For emojis created by the current user, requires either the `CREATE_EXPRESSIONS` or `MANAGE_EXPRESSIONS` permission.
/// For other emojis, requires the `MANAGE_EXPRESSIONS` permission.
///
/// Returns a 204 empty response on success.
/// Fires a [Guild Emojis Update](https://docs.discord.food/topics/gateway-events#guild-emojis-update) Gateway event.
pub fn DELETE_GUILD_EMOJI(
	guild_id: &GuildId,
	emoji_id: &EmojiId,
) -> String {
	format!("/guilds/{}/emojis/{}", guild_id, emoji_id)
}

/// Method: `GET`
///
/// Returns an object containing a list of [emoji](https://docs.discord.food/resources/emoji#emoji-object) objects for the given application under the `items` key.
/// Includes the `user` field.
pub fn GET_APPLICATION_EMOJIS(application_id: &ApplicationId) -> String {
	format!("/applications/{}/emojis", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationEmojisResponse {
	/// The emojis uploaded to the application
	pub items: Vec<Emoji>,
}

/// Method: `GET`
///
/// Returns an [emoji](https://docs.discord.food/resources/emoji#emoji-object) object for the given application and emoji IDs.
/// Includes the `user` field.
pub fn GET_APPLICATION_EMOJI(
	application_id: &ApplicationId,
	emoji_id: &EmojiId,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}

pub type GetApplicationEmojiResponse = Emoji;

/// Method: `POST`
///
/// Creates a new emoji for the application.
///
/// Emojis have a maximum file size of 256 KiB. Attempting to upload an emoji larger than this limit will fail with a 400 bad request.
///
/// Applications may have a maximum of 2,000 emoji.
///
/// Returns the new [emoji](https://docs.discord.food/resources/emoji#emoji-object) object on success.
pub fn CREATE_APPLICATION_EMOJI(application_id: &ApplicationId) -> String {
	format!("/applications/{}/emojis", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationEmojiRequest {
	/// The name of the emoji
	/// (2-32 characters)
	///
	/// name must be unique.
	pub name: ArrayString<32>,
	/// 128x128 emoji image
	pub image: ImageHash,
}

/// Method: `PATCH`
///
/// Modifies the given emoji.
///
/// The names of application emoji must be unique.
///
/// Returns the updated [emoji](https://docs.discord.food/resources/emoji#emoji-object) object on success.
pub fn MODIFY_APPLICATION_EMOJI(
	application_id: &ApplicationId,
	emoji_id: &EmojiId,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyApplicationEmojiRequest {
	/// The name of the emoji
	/// (2-32 characters)
	///
	/// name must be unique.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<32>>,
}

/// Method: `DELETE`
///
/// Delete the given emoji.
///
/// Returns a 204 empty response on success.
pub fn DELETE_APPLICATION_EMOJI(
	application_id: &ApplicationId,
	emoji_id: &EmojiId,
) -> String {
	format!("/applications/{}/emojis/{}", application_id, emoji_id)
}
