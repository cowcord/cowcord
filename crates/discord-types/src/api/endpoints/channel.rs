use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use hex::Hex;
use serde::{Deserialize, Serialize};

use crate::api::channel::{
	AutoArchiveDuration, Channel, ChannelFlags, ChannelType, DefaultReaction, ForumLayoutType,
	IconEmoji, MessageRequestConsentStatus, PartialForumTag, PermissionOverwrite,
	SafetyWarningType, SearchTagSetting, SortOrderType, ThreadMember, ThreadMemberFlags,
	ThreadOnlyChannelMessageParams, ThreadPostData, ThreadSortDirection, ThreadSortType,
	VideoQualityMode,
};
use crate::api::messages::Message;
use crate::api::users::LinkedAccount;
use crate::common::id::{
	BranchId, ChannelId, ChannelPermissionId, EmojiId, ForumTagId, GuildId, MessageId,
	SafetyWarningId, SkuId, UserId,
};
use crate::common::image::ImageHash;
use crate::common::timestamp::Timestamp;
use crate::utils::url::ToStringQuery;

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
	format!("/guilds/{}/channels{}", guild_id, query.to_string_query())
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
/// Returns the new [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event.
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
/// Returns a 204 empty response on success.
/// Fires multiple [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway events.
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
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) or [Thread Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn MODIFY_CHANNEL(channel_id: &ChannelId) -> String {
	format!("/channels/{}", channel_id)
}

///
/// If modifying a thread and setting archived to false, when locked is also false, only the SEND_MESSAGES permission is required. Otherwise, requires the MANAGE_THREADS permission. Requires the thread to have archived set to false or be set to false in the request. Fires a [Thread Update](https://docs.discord.food/topics/gateway-events#thread-update) Gateway event.
/// If modifying a guild channel, requires the MANAGE_CHANNELS permission for the guild. If modifying permission overwrites, the MANAGE_ROLES permission is required. Only permissions you have in the guild or parent channel (if applicable) can be allowed/denied (unless you have a MANAGE_ROLES overwrite in the channel). Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event. If modifying a category, individual [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) events will fire for each child channel that also changes.
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
	#[serde(with = "hex::as_num")]
	pub theme_color: Option<Option<Hex>>,
}

pub type ModifyChannelResponse = Channel;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Deletes a channel, or closes a private message.
///
/// Requires the `MANAGE_CHANNELS` permission for the guild, or `MANAGE_THREADS` if the channel is a thread.
///
/// Deleting a category does not delete its child channels; they will have their `parent_id` removed and a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event will fire for each of them.
///
/// Deleting a guild channel cannot be undone. Use this with caution, as it is impossible to undo this action when performed on a guild channel. In contrast, when used with a private message, it is possible to undo the action by opening a private message with the recipient again.
///
/// For Community guilds, the Rules or Guidelines channel and the Community Updates channel cannot be deleted.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Delete](https://docs.discord.food/topics/gateway-events#channel-delete) or [Thread Delete](https://docs.discord.food/topics/gateway-events#thread-delete) Gateway event.
pub fn DELETE_CHANNEL(
	query: &DeleteChannelQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!("/channels/{}{}", channel_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct DeleteChannelQueryParams {
	/// Whether to leave the group DM without sending a system message (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silent: Option<bool>,
}

pub type DeleteChannelResponse = Channel;

/// Method: `DELETE`
///
/// Deletes a channel's read state for the current user.
///
/// This should be used to delete various read states of channels the client has not been able to access for a while.
///
/// Returns a 204 empty response on success.
pub fn DELETE_READ_STATE(channel_id: &ChannelId) -> String {
	format!("/channels/{}/messages/ack", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct DeleteReadtStateRequest {
	/// The version of the read state feature you are using (default 1, should be 2 to allow the usage of read state types other than `CHANNEL`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<u8>,
	/// The read state type to delete (default CHANNEL)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read_state_type: Option<u8>,
}

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Sets a voice channel's status.
///
/// Requires the `SET_VOICE_CHANNEL_STATUS` permission and additionally the `MANAGE_CHANNELS` permission if the current user is not connected to the voice channel.
///
/// Returns a 204 empty response on success.
/// Fires a [Voice Channel Status Update](https://docs.discord.food/topics/gateway-events#voice-channel-status-update) Gateway event.
pub fn MODIFY_CHANNEL_STATUS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/voice-status", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyChannelStatusRequest {
	/// The status of the voice channel (max 500 characters)
	pub status: Option<ArrayString<500>>,
}

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Edits the channel permission overwrites for a user or role in a channel.
///
/// Only usable for guild channels.
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Only permissions you have in the guild or parent channel (if applicable) can be allowed/denied (unless you have a `MANAGE_ROLES` overwrite in the channel).
///
/// Returns a 204 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn MODIFY_CHANNEL_PERMISSIONS(
	channel_id: &ChannelId,
	permission_id: &ChannelPermissionId,
) -> String {
	format!("/channels/{}/permissions/{}", channel_id, permission_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyChannelPermissionsRequest {
	/// Either 0 (role) or 1 (member)
	pub r#type: u8,
	/// The bitwise value of all allowed permissions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allow: Option<String>,
	/// The bitwise value of all disallowed permissions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deny: Option<String>,
}

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Deletes a channel permission overwrite for a user or role in a channel.
///
/// Only usable for guild channels.
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Returns a 204 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn DELETE_CHANNEL_PERMISSION(
	channel_id: &ChannelId,
	permission_id: &ChannelPermissionId,
) -> String {
	format!("/channels/{}/permissions/{}", channel_id, permission_id)
}

/// Method: `POST`
///
/// Follows a News Channel to send messages to a target channel.
///
/// Requires the `MANAGE_WEBHOOKS` permission in the target channel.
///
/// Returns a [followed channel](https://docs.discord.food/resources/channel#followed-channel-object) object on success.
/// Fires a [Webhooks Update](https://docs.discord.food/topics/gateway-events#webhooks-update) Gateway event for the target channel.
pub fn FOLLOW_CHANNEL(channel_id: &ChannelId) -> String {
	format!("/channels/{}/followers", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct FollowChannelRequesr {
	/// The ID of the target channel
	pub webhook_channel_id: ChannelId,
}

/// Method: `POST`
///
/// Posts a typing indicator for the specified channel.
///
/// If the user has hit the specified per-user rate limit in the channel, the response will instead be a 200 OK with the below response body.
///
/// Official clients expire a typing indicator 10 seconds after the last [Typing Start](https://docs.discord.food/topics/gateway-events#typing-start) Gateway event.
///
/// Returns a 204 empty response on success, unless a ratelimit has been hit, then it returns the `TriggerTypingResponse` object.
/// Fires a [Typing Start](https://docs.discord.food/topics/gateway-events#typing-start) Gateway event.
pub fn TRIGGER_TYPING_INDICATOR(channel_id: &ChannelId) -> String {
	format!("/channels/{}/typing", channel_id)
}

/// Only is returned when the endpoint ratelimit has been hit.
#[derive(Serialize, Deserialize)]
pub struct TriggerTypingResponse {
	/// Duration (in milliseconds) before the user can send another message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_send_cooldown_ms: Option<u32>,
	/// Duration (in milliseconds) before the user can create a new thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thread_create_cooldown_ms: Option<u32>,
}

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `voice` scope
///
/// Checks if the current user is eligible to ring a call in the DM channel.
pub fn GET_CALL_ELIGIBILITY(channel_id: &ChannelId) -> String {
	format!("/channels/{}/call", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetCallEligibilityResponse {
	/// Whether the user is additionally eligible to ring the other recipient(s)
	pub ringable: bool,
}

/// Method: `PATCH`
///
/// Modifies the active call in the private channel.
///
/// This endpoint requires an active call to do anything.
///
/// Returns a 204 empty response on success.
/// Fires a [Call Update](https://docs.discord.food/topics/gateway-events#call-update) Gateway event.
pub fn MODIFY_CALL(channel_id: &ChannelId) -> String {
	format!("/channels/{}/call", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyCallRequest {
	/// The voice region ID for the call
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
}

/// Method: `POST`
///
/// Supports OAuth2 for authentication with the `voice` scope
///
/// Rings the recipients of a private channel to notify them of an active call.
///
/// This endpoint requires an active call to do anything.
///
/// Returns a 204 empty response on success.
/// Fires a [Call Update](https://docs.discord.food/topics/gateway-events#call-update) Gateway event.
pub fn RING_CHANNEL_RECIPIENTS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/call/ring", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct RingChannelRecipientsRequest {
	/// The recipients to ring (default all)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipients: Option<Option<Vec<UserId>>>,
}

/// Method: `POST`
///
/// Supports OAuth2 for authentication with the `voice` scope
///
/// Stops ringing the recipients of a private channel.
///
/// This endpoint requires an active call to do anything.
///
/// Returns a 204 empty response on success.
/// Fires a [Call Update](https://docs.discord.food/topics/gateway-events#call-update) Gateway event.
pub fn STOP_RINGING_CHANNEL_RECIPIENTS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/call/stop-ringing", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct StopRingingChannelRecipientsRequest {
	/// The recipients to stop ringing (default current user)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipients: Option<Option<Vec<UserId>>>,
}

/// Method: `PUT`
///
/// Adds a recipient to a private channel.
///
/// Using this endpoint on a DM will create a group DM with the current user, DM recipient, and the new recipient. The received [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event will contain an extra origin_channel_id field, which is the ID of the DM that was converted.
///
/// Regular group DMs can have up to 10 recipients. Employee group DMs can have up to 25 recipients.
///
/// In a managed group DM, only the managing application can add recipients.
///
/// If operating on a DM, returns a [group DM channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event.
///
/// If operating on a group DM, returns a 204 empty response on success.
/// Fires a [Channel Recipient Add](https://docs.discord.food/topics/gateway-events#channel-recipient-add) Gateway event.
pub fn ADD_CHANNEL_RECIPIENT(
	channel_id: &ChannelId,
	user_id: &UserId,
) -> String {
	format!("/channels/{}/recipients/{}", channel_id, user_id)
}

#[derive(Serialize, Deserialize)]
pub struct AddChannelRecipientRequest {
	/// Access token of a user that has granted your app the `gdm.join` scope
	///
	/// Only required for OAuth2 requests.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub access_token: Option<String>,
	/// Nickname of the user being added
	///
	/// Not applicable when operating on a DM.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<String>,
}

/// Method: `DELETE`
///
/// Removes a recipient from a group DM.
///
/// Requires ownership of the target channel.
///
/// Returns a 204 empty response on success.
/// Fires a [Channel Recipient Remove](https://docs.discord.food/topics/gateway-events#channel-recipient-remove) Gateway event.
pub fn REMOVE_CHANNEL_RECIPIENT(
	channel_id: &ChannelId,
	user_id: &UserId,
) -> String {
	format!("/channels/{}/recipients/{}", channel_id, user_id)
}

/// Method: `PUT`
///
/// Modifies a message request's status.
///
/// Consent statuses other than ACCEPTED are only usable by Discord employees.
///
/// Returns a [DM channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn UPDATE_MESSAGE_REQUEST(channel_id: &ChannelId) -> String {
	format!("/channels/{}/recipients/@me", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMessageRequestRequest {
	/// The new consent status
	pub consent_status: MessageRequestConsentStatus,
}

pub type UpdateMessageRequestResponse = Channel;

/// Method: `DELETE`
///
/// Rejects and deletes a pending message request.
///
/// Returns a [DM channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update), Message ACK, [Channel Delete](https://docs.discord.food/topics/gateway-events#channel-delete), and optionally [DM Settings Upsell Show](https://docs.discord.food/topics/gateway-events#dm-settings-upsell-show) Gateway events.
pub fn REJECT_MESSAGE_REQUEST(channel_id: &ChannelId) -> String {
	format!("/channels/{}/recipients/@me", channel_id)
}

pub type RejectMessageRequestResponse = Channel;

/// Method: `PUT`
///
/// Rejects and deletes multiple pending message requests.
///
/// Returns a list of [DM channel](https://docs.discord.food/resources/channel#channel-object) objects on success.
/// Fires multiple [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update), Message ACK, [Channel Delete](https://docs.discord.food/topics/gateway-events#channel-delete), and optionally [DM Settings Upsell Show](https://docs.discord.food/topics/gateway-events#dm-settings-upsell-show) Gateway events.
pub const BATCH_REJECT_MESSAGE_REQUESTS: &str = "/channels/recipients/@me/batch-reject";

#[derive(Serialize, Deserialize)]
pub struct BatchRejectMessageRequestsRequest {
	/// The IDs of the message requests to reject (max 100)
	pub channel_ids: ArrayVec<ChannelId, 100>,
}

pub type BatchRejectMessageRequestsResponse = Vec<Channel>;

/// Method: `GET`
///
/// Returns a list of [supplemental message request](https://docs.discord.food/resources/channel#supplemental-message-request-structure) objects with the message that triggered each message request.
pub fn GET_SUPPLEMENTAL_MESSAGE_REQUEST_DATA(
	query: &GetSupplementalMessageRequestDataQueryParams
) -> String {
	format!(
		"/users/@me/message-requests/supplemental-data{}",
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetSupplementalMessageRequestDataQueryParams {
	/// The IDs of the message requests to fetch (1-25)
	pub channel_ids: ArrayVec<ChannelId, 25>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSupplementalMessageRequestDataResponseInner {
	/// The ID of the message request
	pub channel_id: ChannelId,
	/// The trigger message
	pub message_preview: Message,
}

pub type GetSupplementalMessageRequestDataRequest =
	Vec<GetSupplementalMessageRequestDataResponseInner>;

/// Method: `POST`
///
/// Acknowledges that a group DM contains users the current user has blocked.
///
/// Returns a 200 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn ACKNOWLEDGE_BLOCKED_USER_WARNING(channel_id: &ChannelId) -> String {
	format!("/channels/{}/blocked-user-warning-dismissal", channel_id)
}

/// Method: `POST`
///
/// Dismisses safety warnings in a DM.
///
/// Returns a 200 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn ACKNOWLEDGE_SAFETY_WARNINGS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/safety-warnings/ack", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct AcknowledgeBlockedUserWarningRequest {
	/// The IDs of the warnings to dismiss (1-100)
	pub warning_ids: ArrayVec<SafetyWarningId, 100>,
}

/// Method: `POST`
///
/// Adds a safety warning to a DM.
///
/// Returns a 200 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
#[cfg(feature = "employee")]
pub fn ADD_SAFETY_WARNING(channel_id: &ChannelId) -> String {
	format!("/channels/{}/add-safety-warning", channel_id)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "employee")]
pub struct AddSafetyWarningRequest {
	/// The type of safety warning to add
	pub safety_warning_type: SafetyWarningType,
}

/// Method: `DELETE`
///
/// Deletes all safety warnings in a DM.
///
/// Returns a 200 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
#[cfg(feature = "employee")]
pub fn DELETE_SAFETY_WARNINGS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/safety-warnings", channel_id)
}

/// Method: `POST`
///
/// Reports all safety warnings in a DM as false positives.
///
/// Returns a 200 empty response on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
#[cfg(feature = "employee")]
pub fn REPORT_SAFETY_WARNING_FALSE_POSITIVE(channel_id: &ChannelId) -> String {
	format!(
		"/channels/{}/safety-warning/report-false-positive",
		channel_id
	)
}

/// Method: `GET`
///
/// Returns all active threads in the guild, including public and private threads.
/// Threads are ordered by their id, in descending order.
#[cfg(feature = "bot")]
pub fn GET_GUILD_ACTIVE_THREADS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/threads/active", guild_id)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "bot")]
pub struct GetGuildActiveThreadsResponse {
	/// The active threads
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
}

/// Method: `GET`
///
/// Returns all active threads in the channel, including public and private threads.
///
/// Threads are ordered by their id, in descending order.
///
/// User must be a member of the guild.
///
/// This endpoint is deprecated and removed in api v10.
/// It is replaced by [Get Guild Active Threads](https://docs.discord.food/resources/channel#get-guild-active-threads).
#[deprecated(
	note = "Deprecated, removed in api v10 + This endpoint is not usable by user accounts."
)]
pub fn GET_ACTIVE_THREADS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/threads/active", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetActiveThreadsResponse {
	/// The active threads
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
}

/// Method: `GET`
///
/// Returns archived threads in the channel that are public.
///
/// Threads are ordered by archive_timestamp, in descending order.
///
/// Requires the `READ_MESSAGE_HISTORY` permission.
///
/// When called on a `GUILD_TEXT` channel, returns threads of [type](https://docs.discord.food/resources/channel#channel-type) `PUBLIC_THREAD`.
/// When called on a `GUILD_NEWS` channel returns threads of [type](https://docs.discord.food/resources/channel#channel-type) `NEWS_THREAD`.
pub fn GET_PUBLIC_ARCHIVED_THREADS(
	query: &GetPublicArchivedThreadsQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/threads/archived/public{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetPublicArchivedThreadsQueryParams {
	/// Get threads before this timestamp
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<Timestamp>,
	/// Max number of threads to return (2-100, default 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPublicArchivedThreadsResponse {
	/// The public, archived threads
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
	/// Whether there are potentially additional threads that could be returned on a subsequent call
	pub has_more: bool,
}

/// Method: `GET`
///
/// Threads are ordered by archive_timestamp, in descending order.
///
/// Requires both the `READ_MESSAGE_HISTORY` and `MANAGE_THREADS` permissions.
///
/// Returns archived threads in the channel that are of [type](https://docs.discord.food/resources/channel#channel-type) `PRIVATE_THREAD`.
pub fn GET_PRIVATE_ARCHIVED_THREADS(
	query: &GetPrivateArchivedThreadsQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/threads/archived/private{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetPrivateArchivedThreadsQueryParams {
	/// Get threads before this timestamp
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<Timestamp>,
	/// Max number of threads to return (2-100, default 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPrivateArchivedThreadsResponse {
	/// The private, archived threads
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
	/// Whether there are potentially additional threads that could be returned on a subsequent call
	pub has_more: bool,
}

/// Method: `GET`
///
/// Threads are ordered by their id, in descending order.
///
/// Requires the `READ_MESSAGE_HISTORY` permission.
///
/// Returns archived threads in the channel that are of [type](https://docs.discord.food/resources/channel#channel-type) `PRIVATE_THREAD`, and the user has joined.
pub fn GET_JOINED_PRIVATE_ARCHIVED_THREADS(
	query: &GetJoinedPrivateArchivedThreadsQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/users/@me/threads/archived/private{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetJoinedPrivateArchivedThreadsQueryParams {
	/// Get threads before this channel ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<ChannelId>,
	/// Max number of threads to return (2-100, default 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GetJoinedPrivateArchivedThreadsResponse {
	/// The private, archived threads the current user has joined
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
	/// Whether there are potentially additional threads that could be returned on a subsequent call
	pub has_more: bool,
}

/// Method: `GET`
///
/// Requires the `READ_MESSAGE_HISTORY` permission.
///
/// Returns threads in the channel that match the search parameters.
pub fn SEARCH_THREADS(
	query: &SearchThreadsQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/threads/search{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct SearchThreadsQueryParams {
	/// Search query to look for matching threads (max 100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<500>>,
	/// Max number of words to skip between matching tokens in the search query (max 100, default 2)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slop: Option<u8>,
	/// The tag IDs to filter results by (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag: Option<ArrayVec<ForumTagId, 20>>,
	/// How to restrict the returned threads by tag (default `match_some`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag_setting: Option<SearchTagSetting>,
	/// Whether to restrict the search to only active or archived threads (default both)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub archived: Option<bool>,
	/// The sorting algorithm to use
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_by: Option<ThreadSortType>,
	/// The direction to sort (default desc)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_order: Option<ThreadSortDirection>,
	/// Max number of threads to return (1-25, default 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Number of threads to skip before returning results (max 9975)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u16>,
	/// Get threads before this thread ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_id: Option<ChannelId>,
	/// Get threads after this thread ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_id: Option<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchThreadsResponse {
	/// The threads that match the search parameters
	pub threads: Vec<Channel>,
	/// A thread member object for each returned thread the current user has joined
	pub members: Vec<ThreadMember>,
	/// Whether there are potentially additional threads that could be returned on a subsequent call
	pub has_more: bool,
	/// The total number of threads that match the search parameters
	pub total_results: u32,
	/// The first messages of each thread
	///
	/// Only returned in thread-only channels
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_messages: Option<Vec<Message>>,
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new thread from an existing message.
///
/// The ID of the created thread will be the same as the ID of the message, and as such a message can only have a single thread created from it.
///
/// When called on a `GUILD_TEXT` channel, creates a `PUBLIC_THREAD`.
/// When called on a `GUILD_NEWS` channel, creates a `NEWS_THREAD`.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) on success.
/// Fires a [Thread Create](https://docs.discord.food/topics/gateway-events#thread-create) and a [Message Update](https://docs.discord.food/topics/gateway-events#message-update) Gateway event.
pub fn CREATE_THREAD_FROM_MESSAGE(
	channel_id: &ChannelId,
	message_id: &MessageId,
) -> String {
	format!("/channels/{}/messages/{}/threads", channel_id, message_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateThreadFromMessageRequest {
	/// The name of the channel (1-100 characters)
	pub name: ArrayString<100>,
	/// Duration in minutes to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080, default 4320)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_archive_duration: Option<AutoArchiveDuration>,
	/// Duration in seconds a user has to wait before sending another message (max 21600)
	///
	/// Bots, as well as users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNELS`, are unaffected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user: Option<u16>,
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new thread that is not connected to an existing message.
///
/// Requires the `CREATE_PUBLIC_THREADS` or `CREATE_PRIVATE_THREADS` permission, depending on the type of thread being created; unless its a thread-only channel, then they only need `SEND_MESSAGES`.
///
/// The maximum request size when sending a message is 100 MiB.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) (with an optional extra [message](https://docs.discord.food/resources/message#message-object) key containing the starter message) on success.
/// Fires a [Thread Create](https://docs.discord.food/resources/channel#channel-object) Gateway event.
pub fn CREATE_THREAD(channel_id: &ChannelId) -> String {
	format!("/channels/{}/threads", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateThreadRequest {
	/// The name of the channel (1-100 characters)
	pub name: String,
	/// Duration in minutes to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080, default 4320)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_archive_duration: Option<i64>,
	/// Duration in seconds a user has to wait before sending another message (max 21600); bots, as well as users with the permission MANAGE_MESSAGES or MANAGE_CHANNELS , are unaffected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user: Option<i64>,
	/// the type of thread to create (default PRIVATE_THREAD )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<i64>,
	/// Whether non-moderators can add other non-moderators to a thread; only available when creating a private thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invitable: Option<bool>,
	/// The IDs of the tags that are applied to a thread in a thread-only channel (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub applied_tags: Option<Vec<ForumTagId>>,
	/// Contents of the first message in the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<ThreadOnlyChannelMessageParams>,
}

/// Method: `POST`
///
/// Requires the `READ_MESSAGE_HISTORY` permission.
///
/// Returns a mapping of thread IDs to their [post data](https://docs.discord.food/resources/channel#thread-post-data-structure) in a thread-only channel.
pub fn GET_CHANNEL_POST_DATA(channel_id: &ChannelId) -> String {
	format!("/channels/{}/post-data", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetChannelPostDatarRequest {
	/// The IDs of the threads to get post data for
	pub thread_ids: Vec<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub struct GetChannelPostDataResponse {
	/// A mapping of thread IDs to their post data
	pub threads: HashMap<ChannelId, ThreadPostData>,
}

/// Method: `GET`
///
/// Requires the VIEW_CHANNEL permission.
///
/// Starting in api v11, this endpoint will always return paginated results. Paginated results can be enabled before API v11 by setting `with_member` to true.
///
/// Returns an array of [thread members](https://docs.discord.food/resources/channel#thread-member-object) objects that are members of the thread.
#[deprecated(
	note = "This endpoint is not usable by user accounts and is restricted according to whether the `GUILD_MEMBERS` Privileged Intent is enabled for the application."
)]
pub fn GET_THREAD_MEMBERS(
	query: &GetThreadMembersQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/thread-members{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetThreadMembersQueryParams {
	/// Whether to include a guild member object for each thread member
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_member: Option<bool>,
	/// Get thread members after this user ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<ChannelId>,
	/// Max number of thread members to return (1-100, default 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

/// Method: `GET`
///
/// Requires the `VIEW_CHANNEL` permission.
///
/// Returns a [thread member](https://docs.discord.food/resources/channel#thread-member-object) object for the specified user if they are a member of the thread.
#[cfg(feature = "bot")]
pub fn GET_THREAD_MEMBER(
	query: &GetThreadMemberQueryParams,
	channel_id: &ChannelId,
	user_id: &UserId,
) -> String {
	format!(
		"/channels/{}/thread-members/{}{}",
		channel_id,
		user_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "bot")]
pub struct GetThreadMemberQueryParams {
	/// Whether to include a guild member object for the thread member
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_member: Option<bool>,
}

#[cfg(feature = "bot")]
pub type GetThreadMemberResponse = ThreadMember;

/// Method: `PUT`
///
/// Adds the current user to a thread.
///
/// Requires the `VIEW_CHANNEL` permission.
/// Also requires the thread is not archived.
///
/// Returns a 204 empty response on success.
/// Fires a [Thread Members Update](https://docs.discord.food/topics/gateway-events#thread-members-update) and a [Thread Create](https://docs.discord.food/topics/gateway-events#thread-create) Gateway event.
pub fn JOIN_THREAD(channel_id: &ChannelId) -> String {
	format!("/channels/{}/thread-members/@me", channel_id)
}

/// Method: `PUT`
///
/// Adds another member to a thread.
///
/// Requires the `SEND_MESSAGES` permission.
/// Also requires the thread is not archived.
///
/// Returns a 204 empty response on success.
/// Fires a [Thread Members Update](https://docs.discord.food/topics/gateway-events#thread-members-update) Gateway event.
pub fn ADD_THREAD_MEMBER(
	channel_id: &ChannelId,
	user_id: &UserId,
) -> String {
	format!("/channels/{}/thread-members/{}", channel_id, user_id)
}

/// Method: `PATCH`
///
/// Updates the current user's thread settings.
///
/// User must be a member of the thread.
///
/// Returns a [thread member](https://docs.discord.food/resources/channel#thread-member-object) on success, or a 204 empty response if nothing changed.
/// Fires a [Thread Member Update](https://docs.discord.food/topics/gateway-events#thread-member-update) Gateway event.
pub fn MODIFY_THREAD_SETTINGS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/thread-members/@me/settings", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyThreadSettingsRequest {
	/// The user's thread flags flags (all except the first can be set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ThreadMemberFlags>,
	/// Whether the user has muted the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub muted: Option<bool>,
	/// The mute metadata for the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute_config: Option<Option<MuteConfig>>,
}

pub type ModifyThreadSettingsResponse = Option<ThreadMember>;

/// Method: `DELETE`
///
/// Removes the current user from a thread.
///
/// Also requires the thread is not archived.
///
/// Returns a 204 empty response on success.
/// Fires a [Thread Members Update](https://docs.discord.food/topics/gateway-events#thread-members-update) Gateway event.
pub fn LEAVE_THREAD(channel_id: &ChannelId) -> String {
	format!("/channels/{}/thread-members/@me", channel_id)
}

/// Method: `DELETE`
///
/// Removes a member from a thread.
///
/// Requires the `MANAGE_THREADS` permission, or the creator of the thread if it is a `PRIVATE_THREAD`.
/// Also requires the thread is not archived.
///
/// Returns a 204 empty response on success.
/// Fires a [Thread Members Update](https://docs.discord.food/topics/gateway-events#thread-members-update) Gateway event.
pub fn REMOVE_THREAD_MEMBER(
	channel_id: &ChannelId,
	user_id: &UserId,
) -> String {
	format!("/channels/{}/thread-members/{}", channel_id, user_id)
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new tag in the thread-only channel.
///
/// Requires the `MANAGE_CHANNELS` permission.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn CREATE_CHANNEL_TAG(channel_id: &ChannelId) -> String {
	format!("/channels/{}/tags", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannelTagRequest {
	/// The name of the tag (max 50 characters)
	pub name: ArrayString<50>,
	/// Whether this tag can only be added to or removed from threads by members with the MANAGE_THREADS permission (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub moderated: Option<bool>,
	/// The ID of a guild's custom emoji
	///
	/// At most one of `emoji_id` and `emoji_name` may be set to a non-null value.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_id: Option<Option<EmojiId>>,
	/// The unicode character of the emoji
	///
	/// At most one of `emoji_id` and `emoji_name` may be set to a non-null value.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_name: Option<Option<String>>,
}

pub type CreateChannelTagResponse = Channel;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Replaces a tag in the thread-only channel.
///
/// Requires the `MANAGE_CHANNELS` permission.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn MODIFY_CHANNEL_TAG(
	channel_id: &ChannelId,
	tag_id: &ForumTagId,
) -> String {
	format!("/channels/{}/tags/{}", channel_id, tag_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyChannelTagRequest {
	/// The name of the tag (max 50 characters)
	pub name: ArrayString<50>,
	/// Whether this tag can only be added to or removed from threads by members with the MANAGE_THREADS permission (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub moderated: Option<bool>,
	/// The ID of a guild's custom emoji
	///
	/// At most one of `emoji_id` and `emoji_name` may be set to a non-null value.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_id: Option<Option<EmojiId>>,
	/// The unicode character of the emoji
	///
	/// At most one of `emoji_id` and `emoji_name` may be set to a non-null value.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_name: Option<Option<String>>,
}

pub type ModifyChannelTagResponse = Channel;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Deletes a tag in the thread-only channel.
///
/// Requires the `MANAGE_CHANNELS` permission.
///
/// Returns a [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn DELETE_CHANNEL_TAG(
	channel_id: &ChannelId,
	tag_id: &ForumTagId,
) -> String {
	format!("/channels/{}/tags/{}", channel_id, tag_id)
}

pub type DeleteChannelTagResponse = Channel;

/// Method: `GET`
///
/// Only usable with OAuth2 for authentication with the `dm_channels.read` scope
///
/// Returns the linked accounts for users in a group DM.
pub fn GET_CHANNEL_LINKED_ACCOUNTS(
	query: &GetChannelLinkedAccountsQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/linked-accounts{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetChannelLinkedAccountsQueryParams {
	/// User IDs to get linked accounts for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_ids: Option<Vec<UserId>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetChannelLinkedAccountsRequest {
	/// The connected accounts for every linked user
	pub linked_accounts: HashMap<UserId, Vec<LinkedAccount>>,
}

/// Method: `DELETE`
///
/// Unlinks the linked lobby from the channel.
///
/// Requires the `MANAGE_CHANNELS` permission.
///
/// Returns the updated [channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Lobby Update](https://docs.discord.food/topics/gateway-events#lobby-update) and [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn REMOVE_LOBBY_LINK(channel_id: &ChannelId) -> String {
	format!("/channels/{}/linked-lobby", channel_id)
}
