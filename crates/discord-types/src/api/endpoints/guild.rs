use serde::{Deserialize, Serialize};
use crate::{api::guild::{Guild, UserGuild}, common::id::GuildId};

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `guilds` scope
///
/// This endpoint returns 200 guilds by default, which is the maximum number of guilds a non-bot user can join.
/// Therefore, pagination is not needed in order to get a list of the users' guilds, and all parameters are optional.
///
/// Returns a list of [user guild](https://ee085bcf.discord-userdoccers.pages.dev/resources/guild#user-guild-object) objects representing the guilds the current user is a member of.
pub fn GET_USER_GUILDS(query: &GetUserGuildsQueryParams) -> String {
	format!("/users/@me/guilds?{}", serde_urlencoded::to_string(query).unwrap_or_default())
}

#[derive(Serialize, Deserialize)]
pub struct GetUserGuildsQueryParams {
	/// Get guilds before this guild ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<GuildId>,
	/// Get guilds after this guild ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<GuildId>,
	/// Max number of guilds to return
    /// (1-200, default 200)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Whether to include approximate member and presence counts
    /// (default false)
    /// 
    /// For OAuth2 requests, this parameter requires the additional `guilds.members.read` scope.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_counts: Option<bool>,
}

pub type GetUserGuildsResponse = Vec<UserGuild>;

/// Method: `GET`
///
/// Returns a list of partial [guild](https://ee085bcf.discord-userdoccers.pages.dev/resources/guild#guild-object) objects representing [non-previewable guilds](https://ee085bcf.discord-userdoccers.pages.dev/resources/guild#guild-previewing) the current user has pending join requests for.
pub const GET_JOIN_REQUEST_GUILDS: &str = "/users/@me/join-request-guilds";

pub type GetJoinRequestGuildsResponse = Vec<Guild>;


/// Method: `DELETE`
///
/// Leaves the given guild ID.
/// 
/// Returns a 204 empty response on success.
/// Fires a [Guild Delete](https://ee085bcf.discord-userdoccers.pages.dev/topics/gateway-events#guild-delete) and a [Guild Member Remove](https://ee085bcf.discord-userdoccers.pages.dev/topics/gateway-events#guild-member-remove) Gateway event.
pub fn LEAVE_GUILD(guild_id: &GuildId) -> String {
	format!("/users/@me/guilds/{}", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct LeaveGuildRequest {
	/// Whether the user is lurking in the guild (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lurking: Option<bool>,
}

/// Method: `POST`
///
/// Creates a new guild.
/// 
/// If not specified, the below parameters use defaults from the 2TffvPucqHkN [guild template](https://ee085bcf.discord-userdoccers.pages.dev/resources/guild-template#guild-template-object).
///
/// Returns a [guild](https://ee085bcf.discord-userdoccers.pages.dev/resources/guild#guild-object) object on success.
/// Fires a [Guild Create](https://ee085bcf.discord-userdoccers.pages.dev/topics/gateway-events#guild-create) Gateway event.
pub const CREATE_GUILD: &str = "/guilds";

#[derive(Serialize, Deserialize)]
pub struct CreateGuildRequest {
	/// The name of the guild
	/// (2-100 characters, excluding trailing and leading whitespace)
	pub name: String,
	/// The description for the guild
	/// (max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<300>>>,
	/// The main voice region ID of the guild
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<Option<String>>,
	/// The guild's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The verification level required for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verification_level: Option<Option<i64>>,
	/// Default message notification level for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_message_notifications: Option<Option<i64>>,
	/// Whose messages are scanned and deleted for explicit content in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub explicit_content_filter: Option<Option<i64>>,
	/// The preferred locale of the guild (default "en-US")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_locale: Option<Option<String>>,
	/// Roles in the new guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Option<Vec<PartialRole>>>,
	/// Channels in the new guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channels: Option<Option<Vec<PartialChannel>>>,
	/// The ID of the guild's AFK channel; this is where members in voice idle for longer than afk_timeout are moved
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_channel_id: Option<Option<Snowflake>>,
	/// The AFK timeout of the guild (one of 60, 300, 900, 1800, 3600, in seconds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_timeout: Option<Option<i64>>,
	/// The ID of the channel where system event messages, such as member joins and premium subscriptions (boosts), are posted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_id: Option<Option<Snowflake>>,
	/// The flags that limit system event messages
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_flags: Option<Option<i64>>,
	/// The template code that inspired this guild, used for analytics
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_template_code: Option<Option<String>>,
	/// Whether the new guild will only be accessible for Discord employees
	#[serde(skip_serializing_if = "Option::is_none")]
	pub staff_only: Option<bool>,
}
