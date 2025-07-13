use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{api_types::{guild::GuildMember, users::PartialUser}, types::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct Channel {
	/// The ID of the channel
	pub id: Snowflake,
	/// The type of channel
	pub r#type: ChannelType,
	/// The ID of the guild the channel is in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// Sorting position of the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<i64>,
	/// Explicit permission overwrites for members and roles
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
	/// The name of the channel (1-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<Option<String>>,
	/// The channel topic (max 4096 characters for thread-only channels, max 1024 characters otherwise)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topic: Option<Option<String>>,
	/// Whether the channel is NSFW
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw: Option<bool>,
	/// The ID of the last message sent (or thread created for thread-only channels, guild added for directory channels) in this channel (may not point to an existing resource)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_message_id: Option<Option<Snowflake>>,
	/// The bitrate (in bits) of the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bitrate: Option<i64>,
	/// The user limit of the voice channel (max 99, 0 refers to no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_limit: Option<i64>,
	/// Duration in seconds seconds a user has to wait before sending another message (max 21600); bots, as well as users with the permission MANAGE_MESSAGES or MANAGE_CHANNELS , are unaffected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user: Option<i64>,
	/// The recipients of the private channel, excluding the requesting user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipients: Option<Vec<PartialUser>>,
	/// The recipient flags of the DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipient_flags: Option<RecipientFlags>,
	/// The group DM's icon hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<String>>,
	/// The nicknames of the users in the group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nicks: Option<Vec<ChannelNick>>,
	/// Whether the group DM is managed by an application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub managed: Option<bool>,
	/// Whether the user has acknowledged the presence of blocked users in the group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub blocked_user_warning_dismissed: Option<bool>,
	/// The safety warnings for the DM channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safety_warnings: Option<Vec<SafetyWarning>>,
	/// The ID of the application that manages the group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<Snowflake>,
	/// The ID of the owner of the group DM or thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner_id: Option<Snowflake>,
	/// The owner of this thread; only included on certain API endpoints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner: Option<Option<GuildMember>>,
	/// The ID of the parent category/channel for the guild channel/thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<Option<Snowflake>>,
	/// When the last pinned message was pinned, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_pin_timestamp: Option<Option<Timestamp>>,
	/// The voice region ID for the voice channel (automatic when null )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtc_region: Option<Option<String>>,
	/// The camera video quality mode of the voice channel (default AUTO )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_quality_mode: Option<i64>,
	/// The number of messages ever sent in a thread; similar to message_count on message creation, but will not decrement the number when a message is deleted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_message_sent: Option<i64>,
	/// The number of messages (not including the initial message or deleted messages) in a thread (if the thread was created before July 1, 2022, it stops counting at 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_count: Option<i64>,
	/// An approximate count of users in a thread, stops counting at 50
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_count: Option<i64>,
	/// The IDs of some of the members in a thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_ids_preview: Option<Vec<Snowflake>>,
	/// Thread-specific channel metadata
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thread_metadata: Option<ThreadMetadata>,
	/// Thread member object for the current user, if they have joined the thread; only included on certain API endpoints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member: Option<ThreadMember>,
	/// Default duration in minutes for newly created threads to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_auto_archive_duration: Option<Option<i64>>,
	/// Default duration in seconds a user has to wait before sending another message in newly created threads; this field is copied to the thread at creation time and does not live update
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_thread_rate_limit_per_user: Option<i64>,
	/// Computed permissions for the invoking user in the channel, including overwrites
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<String>,
	/// The channel's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ChannelFlags>,
	/// The tags that can be used in a thread-only channel (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_tags: Option<Vec<ForumTag>>,
	/// The IDs of tags that are applied to a thread in a thread-only channel (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub applied_tags: Option<Vec<Snowflake>>,
	/// The emoji to show in the add reaction button on a thread in a thread-only channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_reaction_emoji: Option<Option<DefaultReaction>>,
	/// The default layout of a thread-only channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_forum_layout: Option<i64>,
	/// The default sort order of a thread-only channel's threads (default LATEST_ACTIVITY )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_sort_order: Option<Option<i64>>,
	/// The default tag search setting for a thread-only channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_tag_setting: Option<String>,
	/// The emoji to show next to the channel name in channels list
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_emoji: Option<Option<IconEmoji>>,
	/// Whether the DM is a message request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_message_request: Option<bool>,
	/// When the message request was created
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_message_request_timestamp: Option<Option<Timestamp>>,
	/// Whether the DM is a spam message request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_spam: Option<bool>,
	/// The background color of the channel icon emoji encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub theme_color: Option<Option<i64>>,
	/// The status of the voice channel (max 500 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Option<String>>,
	/// When the HD streaming entitlement expires for the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hd_streaming_until: Option<Option<Timestamp>>,
	/// The ID of the user who applied the HD streaming entitlement to the voice channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hd_streaming_buyer_id: Option<Option<Snowflake>>,
	/// The lobby linked to the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linked_lobby: Option<Option<LinkedLobby>>,
}

/// A channel referenced in an [invite](https://5f57c28f.discord-userdoccers.pages.dev/resources/invite#invite-object) or [message](https://5f57c28f.discord-userdoccers.pages.dev/resources/message#message-object).
#[derive(Serialize, Deserialize)]
pub struct PartialChannel {
	/// The ID of the channel
	pub id: Snowflake,
	/// The type of channel
	pub r#type: ChannelType,
	/// The name of the channel (1-100 characters)
	pub name: Option<String>,
	/// The recipients of the DM; only the username field is present
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipients: Option<Vec<PartialUser>>,
	/// The group DM's icon hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<String>>,
	/// The ID of the guild the channel is in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Option<Snowflake>>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelNick {
	/// The ID of the user
	pub id: Snowflake,
	/// The nickname of the user
	pub nick: String,
}

#[derive(Serialize, Deserialize)]
pub struct SafetyWarning {
	/// The ID of the warning
	pub id: String,
	/// The type of warning
	pub r#type: SafetyWarningType,
	/// When the warning expires
	pub expiry: Timestamp,
	/// When the warning was dismissed by the user
	pub dismiss_timestamp: Option<Timestamp>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SafetyWarningType {
	/// User may not want to interact with this person
	STRANGER_DANGER = 1,
	/// User may not want to interact with this person due to inappropriate conversation
	INAPPROPRIATE_CONVERSATION_TIER_1 = 2,
	/// User may not want to interact with this person due to inappropriate conversation
	INAPPROPRIATE_CONVERSATION_TIER_2 = 3,
	/// The recipient's account is likely compromised and should be treated with caution
	LIKELY_ATO = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
	/// A text channel within a guild
	GUILD_TEXT = 0,
	/// A private channel between two users
	DM = 1,
	/// A voice channel within a guild
	GUILD_VOICE = 2,
	/// A private channel between multiple users
	GROUP_DM = 3,
	/// An organizational category that contains up to 50 channels
	GUILD_CATEGORY = 4,
	/// Almost identical to GUILD_TEXT , a channel that users can follow and crosspost into their own guild
	GUILD_NEWS = 5,
	/// A channel in which game developers can sell their game on Discord
	GUILD_STORE = 6,
	/// A temporary sub-channel within a GUILD_NEWS channel
	NEWS_THREAD = 10,
	/// a temporary sub-channel within a GUILD_TEXT , GUILD_FORUM , or GUILD_MEDIA channel
	PUBLIC_THREAD = 11,
	/// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
	PRIVATE_THREAD = 12,
	/// A voice channel for hosting events with an audience in a guild
	GUILD_STAGE_VOICE = 13,
	/// The main channel in a hub containing the listed guilds
	GUILD_DIRECTORY = 14,
	/// A channel that can only contain threads
	GUILD_FORUM = 15,
	/// A channel that can only contain threads in a gallery view
	GUILD_MEDIA = 16,
	/// A game lobby channel
	LOBBY = 17,
	/// A private channel created by the social layer SDK
	EPHEMERAL_DM = 18,
}

bitflags! {
	pub struct ChannelFlags: u64 {
		/// Guild channel is hidden from the guild's feed
		const GUILD_FEED_REMOVED = 1 << 0;
		/// Thread is pinned to the top of its parent thread-only channel
		const PINNED = 1 << 1;
		/// Guild channel has been removed from the guild's active channels
		const ACTIVE_CHANNELS_REMOVED = 1 << 2;
		/// Thread-only channel requires a tag to create threads in
		const REQUIRE_TAG = 1 << 4;
		/// Channel is marked as spam
		const IS_SPAM = 1 << 5;
		/// Guild channel is used as a read-only resource for onboarding and is not shown in the channel list
		const IS_GUILD_RESOURCE_CHANNEL = 1 << 7;
		/// Channel is created by Clyde AI, which has full access to all message content
		const CLYDE_AI = 1 << 8;
		/// Guild channel is scheduled for deletion and is not shown in the UI
		const IS_SCHEDULED_FOR_DELETION = 1 << 9;
		/// Guild channel has summaries disabled
		const SUMMARIES_DISABLED = 1 << 11;
		/// Role subscription tier for this guild channel has not been published yet
		const IS_ROLE_SUBSCRIPTION_TEMPLATE_PREVIEW_CHANNEL = 1 << 13;
		/// Group DM is used for broadcasting a live stream
		const IS_BROADCASTING = 1 << 14;
		/// Media channel has the embedded download options hidden for media attachments
		const HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15;
		/// Group DM is used for guild join request interviews
		const IS_JOIN_REQUEST_INTERVIEW_CHANNEL = 1 << 16;
		/// User does not have permission to view the channel
		const OBFUSCATED = 1 << 17;
	}
}

bitflags! {
	pub struct RecipientFlags: u64 {
		/// User has dismissed the IN_GAME_MESSAGE_NUX message for this DM channel
		const DISMISSED_IN_GAME_MESSAGE_NUX = 1 << 0;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoQualityMode {
	/// Discord chooses the quality for optimal performance
	AUTO = 1,
	/// 720p quality
	FULL = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumLayoutType {
	/// No layout type explicitly set
	DEFAULT = 0,
	/// Threads are displayed in a list
	LIST = 1,
	/// Threads are displayed in a collection of tiles
	GRID = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SortOrderType {
	/// Sort by the most recently active threads
	LATEST_ACTIVITY = 0,
	/// Sort by when the thread was created (from most recent to oldest)
	CREATION_TIME = 1,
}

#[derive(Serialize, Deserialize)]
pub enum SearchTagSetting {
	/// Threads with any of the selected tags will be shown in the results
	match_some,
	/// Threads with all of the selected tags will be shown in the results
	match_all,
}

#[derive(Serialize, Deserialize)]
pub struct FollowedChannel {
	/// The source channel ID
	pub channel_id: Snowflake,
	/// Created target webhook ID
	pub webhook_id: Snowflake,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionOverwrite {
	/// Role or user ID
	pub id: Snowflake,
	/// The type of overwritten entity
	pub r#type: PermissionOverwriteType,
	/// The bitwise value of all allowed permissions
	pub allow: String,
	/// The bitwise value of all disallowed permissions
	pub deny: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PermissionOverwriteType {
	/// Permissions based on a role
	role = 0,
	/// Permissions for a specific member
	member = 1,
}

#[derive(Serialize, Deserialize)]
pub struct ThreadMetadata {
	/// Whether the thread is archived
	pub archived: bool,
	/// Duration in minutes to stop showing in the channel list after inactivity (one of 60, 1440, 4320, 10080)
	pub auto_archive_duration: i64,
	/// Timestamp when the thread's archive status was last changed, used for calculating recent activity
	pub archive_timestamp: Timestamp,
	/// Whether the thread is locked; when a thread is locked, only users with MANAGE_THREADS can interact with it
	pub locked: bool,
	/// Whether non-moderators can add other non-moderators to a thread; only available on private threads
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invitable: Option<bool>,
	/// Timestamp when the thread was created; only populated for threads created after 2022-01-09
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_timestamp: Option<Option<Timestamp>>,
}

#[derive(Serialize, Deserialize)]
pub struct ThreadMember {
	/// The ID of the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<Snowflake>,
	/// The ID of the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<Snowflake>,
	/// The time the current user last joined the thread
	pub join_timestamp: Timestamp,
	/// The user's thread flags
	pub flags: ThreadMemberFlags,
	/// Whether the user has muted the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub muted: Option<bool>,
	/// The mute metadata for the thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute_config: Option<Option<MuteConfig>>,
	/// The member object for the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member: Option<GuildMember>,
}

bitflags! {
	pub struct ThreadMemberFlags: u64 {
		/// User has interacted with the thread
		const HAS_INTERACTED = 1 << 0;
		/// User receives notifications for all messages
		const ALL_MESSAGES = 1 << 1;
		/// User receives notifications only for messages that @mention them
		const ONLY_MENTIONS = 1 << 2;
		/// User does not receive any notifications
		const NO_MESSAGES = 1 << 3;
	}
}

#[derive(Serialize, Deserialize)]
pub struct DefaultReaction {
	/// The ID of a guild's custom emoji
	pub emoji_id: Option<Snowflake>,
	/// The unicode character of the emoji
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IconEmoji {
	/// The ID of a guild's custom emoji
	pub id: Option<Snowflake>,
	/// The unicode character of the emoji
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ForumTag {
	/// The ID of the tag
	pub id: Snowflake,
	/// The name of the tag (max 50 characters)
	pub name: String,
	/// Whether this tag can only be added to or removed from threads by members with the MANAGE_THREADS permission
	pub moderated: bool,
	/// The ID of a guild's custom emoji
	pub emoji_id: Option<Snowflake>,
	/// The unicode character of the emoji
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedLobby {
	/// The ID of the application
	pub application_id: Snowflake,
	/// The ID of the lobby
	pub lobby_id: Snowflake,
	/// The ID of the user who linked the lobby
	pub linked_by: Snowflake,
	/// When the lobby was linked to channel
	pub linked_at: Timestamp,
}

