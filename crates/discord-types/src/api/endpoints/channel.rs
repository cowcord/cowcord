use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};

use crate::api::channel::{
	AutoArchiveDuration,
	Channel,
	ChannelFlags,
	ChannelType,
	DefaultReaction,
	ForumLayoutType,
	IconEmoji,
	PartialForumTag,
	PermissionOverwrite,
	SearchTagSetting,
	SortOrderType,
	VideoQualityMode,
};
use crate::common::id::{ChannelId, ForumTagId, GuildId, SkuId, UserId};
use crate::common::image::ImageHash;

/// Method: `GET`
///
/// Returns a list of active [private channel](https://docs.discord.food/resources/channel#channel-object) objects the user is participating in.
pub const GET_PRIVATE_CHANNELS: &str = "/users/@me/channels";

pub type GetPrivateChannelsResponse = Vec<Channel>;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `dm_channels.read` scope
///
/// Returns an existing [DM channel](https://docs.discord.food/resources/channel#channel-object) object with a user.
pub fn GET_DM_CHANNEL(user_id: &UserId) -> String {
	format!("/users/@me/dms/{}", user_id)
}

pub type GetDmChannelResponse = Channel;

/// Method: `POST`
///
/// Supports OAuth2 for authentication with the `gdm.join` scope
///
/// One recipient creates or returns an existing [DM channel](https://docs.discord.food/resources/channel#channel-object), none or multiple recipients create a [group DM channel](https://docs.discord.food/resources/channel#channel-object).
///
/// If multiple channels with a single recipient exist, the most recent channel is returned.
///
/// Clients should not use this endpoint to create multiple new DMs in a short period of time. A DM is only counted as created if the user sends the first message in the DM and the channel did not have existing message history. Users may only create 10 new DMs to non-bot users in a 10-minute window. Suspicious DM activity may be flagged by Discord and require [additional verification steps](https://docs.discord.food/resources/user#required-action-type) or lead to **immediate account termination**.
///
/// One of `recipient_id`, `recipients` or `access_tokens` is required. Bots cannot DM other bots or create group DMs without `access_tokens`.
///
/// Returns a [private channel](https://docs.discord.food/resources/channel#channel-object) object. Fires a [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event.
pub const CREATE_PRIVATE_CHANNEL: &str = "/users/@me/channels";

#[derive(Serialize, Deserialize)]
pub struct CreatePrivateChannelRequest {
	/// The recipient to DM
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipient_id: Option<UserId>,
	/// The users to include in the private channel
	///
	/// By including only the client user id in this you can create a group dm with only 1 other person
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipients: Option<Vec<UserId>>,
	/// The access tokens of users that have granted your app the `gdm.join` scope
	///
	/// Requires access_tokens to be provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub access_tokens: Option<Vec<String>>,
	/// A mapping of user IDs to their respective nicknames
	///
	/// Requires access_tokens to be provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nicks: Option<HashMap<UserId, String>>,
}

pub type CreatePrivateChannelResponse = Channel;

/// Method: `GET`
///
/// Does not include threads.
///
/// If the user is not in the guild, the guild must be discoverable.
///
/// Returns a list of [guild channel](https://docs.discord.food/resources/channel#channel-object) objects for the guild.
pub fn GET_GUILD_CHANNELS(
	query: &GetGuildChannelsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/channels?{}",
		guild_id,
		serde_urlencoded::to_string(query).unwrap_or_default()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildChannelsQueryParams {
	/// Whether to return calculated permissions for the invoking user in each channel (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<bool>,
}

pub type GetGuildChannelsResponse = Vec<Channel>;

/// Method: `GET`
///
/// If the user is not in the guild, the guild must be discoverable.
///
/// Returns a list of snowflakes representing up to 10 of the top read channels in the guild.
#[deprecated]
pub fn GET_GUILD_TOP_READ_CHANNELS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/top-read-channels", guild_id)
}

pub type GetGuildTopReadChannels = Vec<ChannelId>;

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new channel in the guild. Requires the `MANAGE_CHANNELS` permission.
///
/// If setting permission overwrites, only permissions you have in the guild can be allowed/denied.
///
/// Setting `MANAGE_ROLES` permission in channels is only possible for guild administrators.
///
/// Fires a [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event.
///
/// Returns the new [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
pub fn CREATE_GUILD_CHANNEL(guild_id: &GuildId) -> String {
	format!("/guilds/{}/channels", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildChannelRequest {
	/// The name of the channel (1-100 characters)
	pub name: String,
	/// Sorting position of the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Option<i16>>,
	/// The type of channel (default GUILD_TEXT )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<Option<ChannelType>>,
	/// The channel topic (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topic: Option<Option<ArrayString<1024>>>,
	/// Whether the channel is NSFW
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw: Option<Option<bool>>,
	/// Duration in seconds a user has to wait before sending another message (max 21600)
	///
	/// Bots and users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNELS` are unaffected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user: Option<Option<u16>>,
	/// The bitrate (in bits) of the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bitrate: Option<Option<u32>>,
	/// The user limit of the voice channel (max 99, 0 refers to no limit)
	///
	/// The maximum user limit for stage channels is always 10000 and cannot be set to 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_limit: Option<Option<u8>>,
	/// Explicit permission overwrites for members and roles
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permission_overwrites: Option<Option<Vec<PermissionOverwrite>>>,
	/// The ID of the parent category for the guild channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<Option<ChannelId>>,
	/// The voice region ID for the voice channel (automatic when null )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtc_region: Option<Option<String>>,
	/// The camera video quality mode of the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_quality_mode: Option<Option<VideoQualityMode>>,
	/// The ID of the SKU showcased by the store channel
	pub sku_id: SkuId,
	/// The ID of the special branch granted by the store channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub branch_id: Option<Option<BranchId>>,
	/// Default duration in minutes for newly created threads to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_auto_archive_duration: Option<Option<AutoArchiveDuration>>,
	/// Default duration in seconds a user has to wait before sending another message in newly created threads; this field is copied to the thread at creation time and does not live update
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_thread_rate_limit_per_user: Option<Option<i64>>,
	/// The tags that can be used in a thread-only channel (max 20)
	///
	/// Only the `name` field is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_tags: Option<Option<Vec<PartialForumTag>>>,
	/// The emoji to show in the add reaction button on a thread in a thread-only channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_reaction_emoji: Option<Option<DefaultReaction>>,
	/// The default layout of a forum channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_forum_layout: Option<Option<ForumLayoutType>>,
	/// The default sort order of a thread-only channel's threads
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_sort_order: Option<Option<SortOrderType>>,
	/// The default tag setting of a thread-only channel (default `match_some`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_tag_setting: Option<Option<SearchTagSetting>>,
}

pub type CreateGuildChannelResponse = Channel;

/// Method: `PATCH`
///
/// Modifies the positions of a set of [channel](https://docs.discord.food/resources/channel#channel-object) objects for the guild.
///
/// Requires the MANAGE_CHANNELS permission.
///
/// Only channels to be modified are required.
///
/// This endpoint takes a JSON array of parameters in the following format:
///
/// Fires multiple [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway events.
///
/// Returns a 204 empty response on success.
pub fn MODIFY_GUILD_CHANNEL_POSITIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/channels", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildChannelPositionsRequestInner {
	/// The ID of the channel
	pub id: ChannelId,
	/// Sorting position of the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Option<i16>>,
	/// Syncs the permission overwrites with the new parent, if moving to a new category
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lock_permissions: Option<Option<bool>>,
	/// The ID of the parent category for the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<Option<ChannelId>>,
}

pub type ModifyGuildChannelPositionsRequest = Vec<ModifyGuildChannelPositionsRequestInner>;

/// Method: `GET`
///
/// Requires the `VIEW_CHANNEL` permission for the guild.
///
/// If the channel is a thread, a [thread member](https://docs.discord.food/resources/channel#thread-member-object) object is included in the returned result.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) object for a given channel ID.
pub fn GET_CHANNEL(channel_id: &ChannelId) -> String {
	format!("/channels/{}", channel_id)
}

pub type GetChannelResponse = Channel;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Updates a channel's settings.
///
/// Fires a [Channel Update](https://bc099d90.discord-userdoccers.pages.dev/topics/gateway-events#channel-update) or [Thread Update](https://bc099d90.discord-userdoccers.pages.dev/topics/gateway-events#channel-update) Gateway event.
///
/// Returns a [channel](https://bc099d90.discord-userdoccers.pages.dev/resources/channel#channel-object) on success.
pub fn MODIFY_CHANNEL(channel_id: &ChannelId) -> String {
	format!("/channels/{}", channel_id)
}

///
/// If modifying a thread and setting archived to false, when locked is also false, only the SEND_MESSAGES permission is required. Otherwise, requires the MANAGE_THREADS permission. Requires the thread to have archived set to false or be set to false in the request. Fires a [Thread Update](https://bc099d90.discord-userdoccers.pages.dev/topics/gateway-events#thread-update) Gateway event.
/// If modifying a guild channel, requires the MANAGE_CHANNELS permission for the guild. If modifying permission overwrites, the MANAGE_ROLES permission is required. Only permissions you have in the guild or parent channel (if applicable) can be allowed/denied (unless you have a MANAGE_ROLES overwrite in the channel). Fires a [Channel Update](https://bc099d90.discord-userdoccers.pages.dev/topics/gateway-events#channel-update) Gateway event. If modifying a category, individual [Channel Update](https://bc099d90.discord-userdoccers.pages.dev/topics/gateway-events#channel-update) events will fire for each child channel that also changes.
#[derive(Serialize, Deserialize)]
pub struct ModifyChannelRequest {
	/// The name of the channel (1-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<100>>,
	/// The type of channel
	///
	/// Only conversion between text and news is supported and only in guilds with the "NEWS" feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ChannelType>,
	/// Sorting position of the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Option<i16>>,
	/// The channel topic (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topic: Option<Option<ArrayString<1024>>>,
	/// The group DM's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<ImageHash>,
	/// Whether the channel is NSFW
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw: Option<Option<bool>>,
	/// Duration in seconds a user has to wait before sending another message (max 21600); bots, as well as users with the permission MANAGE_MESSAGES or MANAGE_CHANNELS , are unaffected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user: Option<Option<u16>>,
	/// The bitrate (in bits) of the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bitrate: Option<Option<u32>>,
	/// the user limit of the voice channel (max 99, 0 refers to no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_limit: Option<Option<u8>>,
	/// Explicit permission overwrites for members and roles
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permission_overwrites: Option<Option<Vec<PermissionOverwrite>>>,
	/// The ID of the parent category for the guild channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<Option<ChannelId>>,
	/// The ID of the owner for the group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner: Option<Option<UserId>>,
	/// The voice region ID for the voice channel (automatic when null)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtc_region: Option<Option<VideoQualityMode>>,
	/// The camera video quality mode of the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_quality_mode: Option<Option<VideoQualityMode>>,
	/// Default duration in minutes for newly created threads to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_auto_archive_duration: Option<Option<AutoArchiveDuration>>,
	/// Default duration in seconds a user has to wait before sending another message in newly created threads; this field is copied to the thread at creation time and does not live update
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_thread_rate_limit_per_user: Option<u32>,
	/// Duration in minutes to automatically archive the thread after recent activity (one of 60, 1440, 4320, 10080)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_archive_duration: Option<AutoArchiveDuration>,
	/// Whether the thread is archived
	#[serde(skip_serializing_if = "Option::is_none")]
	pub archived: Option<Option<bool>>,
	/// Whether the thread is locked; when a thread is locked, only users with `MANAGE_THREADS` can unarchive it
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locked: Option<Option<bool>>,
	/// Whether non-moderators can add other non-moderators to a thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invitable: Option<Option<bool>>,
	/// The channel's flags (only `GUILD_FEED_REMOVED`, `PINNED`, `ACTIVE_CHANNELS_REMOVED`, and `REQUIRE_TAG` can be set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ChannelFlags>,
	/// The tags that can be used in a thread-only channel (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_tags: Option<ArrayVec<PartialForumTag, 20>>,
	/// The IDs of tags that are applied to a thread in a thread-only channel (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub applied_tags: Option<ArrayVec<ForumTagId, 5>>,
	/// The emoji to show in the add reaction button on a thread in a thread-only channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_reaction_emoji: Option<Option<DefaultReaction>>,
	/// The default layout of a forum channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_forum_layout: Option<Option<ForumLayoutType>>,
	/// The default sort order of a thread-only channel's threads
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_sort_order: Option<Option<SortOrderType>>,
	/// The default tag setting of a thread-only channel (default `match_some`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_tag_setting: Option<Option<SearchTagSetting>>,
	/// The emoji to show next to the channel name in channels list
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_emoji: Option<Option<IconEmoji>>,
	/// The background color of the channel icon emoji encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub theme_color: Option<Option<u32>>,
}

pub type ModifyChannelResponse = Channel;
