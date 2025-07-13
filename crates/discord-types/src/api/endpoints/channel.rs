#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::channel::{
	Channel,
	DefaultReaction,
	FollowedChannel,
	ForumTag,
	IconEmoji,
	PermissionOverwrite,
	SupplementalMessageRequest,
	ThreadMember,
	ThreadOnlyChannelMessageParams,
	ThreadPostData,
};
use crate::api::message::Message;
use crate::api::user_settings::MuteConfig;
use crate::api::types::{CdnUri, Snowflake, timestamp::Timestamp};
use crate::utils::request::{API_VERSION, to_string_query};

/// Type: get
pub const GET_PRIVATE_CHANNELS_ENDPOINT: &str = "/users/@me/channels";

pub type GetPrivateChannelsResponse = Vec<Channel>;

/// Type: get
///
/// supports OAuth2 for auth
pub fn GET_DM_CHANNEL_ENDPOINT(user_id: Snowflake) -> String {
	format!("/users/@me/dms/{}", user_id)
}

pub type GetDmChannelResponse = Channel;

/// Type: post
///
/// supports OAuth2 for auth
pub const CREATE_PRIVATE_CHANNEL_ENDPOINT: &str = "/users/@me/channels";

/// One of recipient_id, recipients or access_tokens is required
#[derive(Serialize)]
#[serde(default)]
pub struct CreatePrivateChannelRequest {
	#[deprecated]
	pub recipient_id:  Snowflake,
	pub recipients:    Vec<Snowflake>,
	pub access_tokens: Vec<String>,
	pub nicks:         HashMap<Snowflake, String>,
}

pub type CreatePrivateChannelResponse = Channel;

/// Type: get
pub fn GET_GUILD_CHANNELS_ENDPOINT(
	guild_id: Snowflake,
	permissions: bool,
) -> String {
	format!("/guilds/{}/channels?permissions={}", guild_id, permissions)
}

pub type GetGuildChannelsResponse = Vec<Channel>;

/// Type: get
#[deprecated]
pub fn GET_GUILD_TOP_READ_CHANNELS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/top-read-channels", guild_id)
}

pub type GetGuildTopReadChannelsResponse = Vec<Snowflake>;

/// Type: post
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_CHANNELS permission
pub fn CREATE_GUILD_CHANNEL_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/channels", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateGuildChannelRequest {
	pub name:                               String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position:                           Option<u8>,
	/// https://docs.discord.food/resources/channel#channel-type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type:                             Option<u8>,
	/// Only for: Text, News, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topic:                              Option<String>,
	/// Only for: Text, News, Voice, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw:                               Option<bool>,
	/// Only for: Text, News, Voice, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user:                Option<u16>,
	/// for stages the max bitrate is 64 kbps
	///
	/// but for voice channels it depends on the guild boost tier
	///
	/// no tier: 96 kbps
	///
	/// tier 1: 128 kbps
	///
	/// tier 2: 256 kbps
	///
	/// tier 3: 384 kbps
	///
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bitrate:                            Option<u32>,
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_limit:                         Option<u16>,
	/// Only for: Text, News, Voice, Category, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permission_overwrites:              Option<Vec<PermissionOverwrite>>,
	/// Only for: Text, News, Voice, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id:                          Option<Snowflake>,
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtc_region:                         Option<String>,
	/// https://docs.discord.food/resources/channel#video-quality-mode
	///
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_quality_mode:                 Option<u8>,
	/// Only for: Text, News, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_auto_archive_duration:      Option<u16>,
	/// Only for: Text, News, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_thread_rate_limit_per_user: Option<u16>,
	/// Only for: Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_tags:                     Option<Vec<ForumTag>>,
	/// Only for: Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_reaction_emoji:             Option<DefaultReaction>,
	/// https://docs.discord.food/resources/channel#forum-layout-type
	///
	/// Only for: Forum
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_forum_layout:               Option<u8>,
	/// https://docs.discord.food/resources/channel#sort-order-type
	///
	/// Only for: Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_sort_order:                 Option<u8>,
}

pub type CreateGuildChannelResponse = Channel;

/// Type: patch
pub fn MODIFY_GUILD_CHANNEL_POSITIONS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/channels", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyGuildChannelPositionsRequest {
	pub id:               Snowflake,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position:         Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lock_permissions: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id:        Option<bool>,
}

/// Type: get
///
/// requires VIEW_CHANNEL permission
pub fn GET_CHANNEL_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}", channel_id)
}

/// "If the channel is a thread, a thread member object is included in the returned result."
/// tf does this mean
pub type GetChannelResponse = Channel;

/// Type: patch
///
/// supports the X-Audit-Log-Reason header
pub fn MODIFY_CHANNEL_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyChannelRequest {
	/// Only for: Text, News, Voice, Category, Stage, Forum, Media, Thread, Group DM
	pub name:                               String,
	/// https://docs.discord.food/resources/channel#channel-type
	///
	/// Only for: Text, News
	pub r#type:                             u8,
	/// Only for: Text, News, Voice, Category, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position:                           Option<u16>,
	/// Only for: Text, News, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub topic:                              Option<String>,
	/// Only for: Group DM
	///
	/// probably Option<CdnUri>
	pub icon:                               CdnUri,
	/// Only for: Text, News, Voice, Stage, Forum
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw:                               Option<bool>,
	/// Only for: Text, News, Voice, Stage, Forum, Media, Thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_per_user:                Option<u16>,
	/// for stages the max bitrate is 64 kbps
	///
	/// but for voice channels it depends on the guild boost tier
	///
	/// no tier: 96 kbps
	///
	/// tier 1: 128 kbps
	///
	/// tier 2: 256 kbps
	///
	/// tier 3: 384 kbps
	///
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bitrate:                            Option<u32>,
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_limit:                         Option<u16>,
	/// Only for: Text, News, Voice, Category, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permission_overwrites:              Option<Vec<PermissionOverwrite>>,
	/// Only for: Text, News, Voice, Stage, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id:                          Option<Snowflake>,
	/// Only for: Group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner:                              Option<Snowflake>,
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtc_region:                         Option<String>,
	/// https://docs.discord.food/resources/channel#video-quality-mode
	///
	/// Only for: Voice, Stage
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_quality_mode:                 Option<u8>,
	/// Only for: Text, News, Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_auto_archive_duration:      Option<u16>,
	/// Only for: Text, News, Forum, Media
	///
	/// might be Option<u16>
	pub default_thread_rate_limit_per_user: u16,
	/// Only for: Thread
	pub auto_archive_duration:              u16,
	/// Only for: Thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub archived:                           Option<bool>,
	/// Only for: Thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locked:                             Option<bool>,
	/// Only for: Private Thread
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invitable:                          Option<bool>,
	/// https://docs.discord.food/resources/channel#channel-flags
	///
	/// only GUILD_FEED_REMOVED, PINNED, ACTIVE_CHANNELS_REMOVED, and REQUIRE_TAG can be set
	pub flags:                              u64,
	/// Only for: Forum, Media
	pub available_tags:                     Vec<ForumTag>,
	/// Only for: Thread
	pub applied_tags:                       Vec<Snowflake>,
	/// Only for: Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_reaction_emoji:             Option<DefaultReaction>,
	/// https://docs.discord.food/resources/channel#forum-layout-type
	///
	/// Only for: Forum
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_forum_layout:               Option<u8>,
	/// https://docs.discord.food/resources/channel#sort-order-type
	///
	/// Only for: Forum, Media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_sort_order:                 Option<u8>,
	/// Only for: Text, News, Voice, Stage, Forum
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_emoji:                         Option<IconEmoji>,
	/// Only for: Text, News, Voice, Stage, Forum
	#[serde(skip_serializing_if = "Option::is_none")]
	pub theme_color:                        Option<u32>,
}

pub type ModifyChannelResponse = Channel;

/// Type: delete
///
/// supports the X-Audit-Log-Reason header
/// for guilds, requires MANAGE_CHANNELS or MANAGE_THREADS (if its a thread) permission
/// guild channels are permanently deleted and cannnot be undeleted.
/// private dms however can be reopened by just messaging the person again
pub fn DELETE_CHANNEL_ENDPOINT(
	channel_id: Snowflake,
	silent: bool,
) -> String {
	format!("/channels/{}?silent={}", channel_id, silent)
}

pub type DeleteChannelResponse = Channel;

/// Type: delete
pub fn DELETE_READ_STATE_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/messages/ack", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct DeleteReadStateRequest {
	/// The version of the read state feature you are using (default 1, should be 2 to allow the usage of read state types other than CHANNEL)
	pub version:         u8,
	/// default is CHANNEL, idk what number cooresponds to what so idk prob just use 1
	pub read_state_type: u8,
}

/// Type: put
///
/// supports the X-Audit-Log-Reason header
/// requires SET_VOICE_CHANNEL_STATUS permission if connected and MANAGE_CHANNELS if not
pub fn MODIFY_CHANNEL_STATUS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/voice-status", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyChannelStatusRequest {
	// #[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
}

/// Type: put
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_ROLES permission
pub fn MODIFY_CHANNEL_PERMISSIONS_ENDPOINT(
	channel_id: Snowflake,
	overwrite_id: Snowflake,
) -> String {
	format!("/channels/{}/permissions/{}", channel_id, overwrite_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyChannelPermissionsRequest {
	/// role = 0, member = 1
	pub r#type: u8,
	/// https://docs.discord.food/topics/permissions#permissions
	pub allow:  String,
	/// https://docs.discord.food/topics/permissions#permissions
	pub deny:   String,
}

/// Type: delete
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_ROLES permission
pub fn DELETE_CHANNEL_PERMISSION_ENDPOINT(
	channel_id: Snowflake,
	overwrite_id: Snowflake,
) -> String {
	format!("/channels/{}/permissions/{}", channel_id, overwrite_id)
}

/// Type: post
///
/// requires MANAGE_WEBHOOKS permission in the target channel for the updates
pub fn FOLLOW_CHANNEL_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/followers", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct FollowChannelRequest {
	pub webhook_channel_id: Snowflake,
}

pub type FollowChannelResponse = FollowedChannel;

/// Type: post
pub fn TRIGGER_TYPING_INDICATOR_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/typing", channel_id)
}

#[derive(Deserialize)]
pub struct TriggerTypingIndicatorResponse {
	pub message_send_cooldown_ms:  u32,
	pub thread_create_cooldown_ms: u32,
}

/// Type: get
///
/// supports OAuth2 for auth
pub fn GET_CALL_ELIGIBILITY_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/call", channel_id)
}

#[derive(Deserialize)]
pub struct GetCallEligibilityResponse {
	pub ringable: bool,
}

/// Type: patch
///
/// requires an active call to do anything
pub fn MODIFY_CALL_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/call", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyCallRequest {
	/// https://docs.discord.food/resources/voice#voice-region-object
	pub region: String,
}

/// Type: post
///
/// supports OAuth2 for auth
/// requires an active call to do anything
pub fn RING_CHANNEL_RECIPIENTS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/call/ring", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct RingChannelRecipientsRequest {
	/// default: all
	pub recipients: Option<Vec<Snowflake>>,
}

/// Type: post
///
/// supports OAuth2 for auth
/// requires an active call to do anything
pub fn STOP_RINGING_CHANNEL_RECIPIENTS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/call/stop-ringing", channel_id)
}

#[derive(Serialize)]
pub struct StopRingingChannelRecipientsRequest {
	pub recipients: Option<Vec<Snowflake>>,
}

/// Type: put
pub fn ADD_CHANNEL_RECIPIENT_ENDPOINT(
	channel_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/channels/{}/recipients/{}", channel_id, user_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct AddChannelRecipientRequest {
	/// Access token of a user that has granted your app the gdm.join scope
	///
	/// Only required for OAuth2 requests.
	pub access_token: String,
	/// Not applicable when operating on a DM.
	pub nick:         String,
}

/// if operating on a group dm an empty 204 is responded, but if its a private dm then a group dm channel object is returned
pub type AddChannelRecipientResponse = Option<Channel>;

/// Type: delete
pub fn REMOVE_CHANNEL_RECIPIENT_ENDPOINT(
	channel_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/channels/{}/recipients/{}", channel_id, user_id)
}

/// Type: put
pub fn UPDATE_MESSAGE_REQUEST_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/recipients/@me", channel_id)
}

#[derive(Serialize)]
// message request request 🔥
pub struct UpdateMessageRequestRequest {
	/// https://docs.discord.food/resources/channel#consent-status
	///
	/// any consent status other than ACCEPTED is only usable by discord employees
	pub consent_status: u8,
}

pub type UpdateMessageRequestResponse = Channel;

/// Type: delete
pub fn REJECT_MESSAGE_REQUEST_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/recipients/@me", channel_id)
}

pub type RejectMessageRequestResponse = Channel;

/// Type: put
pub const BATCH_REJECT_MESSAGE_REQUESTS_ENDPOINT: &str = "/channels/recipients/@me/batch-reject";

#[derive(Serialize)]
pub struct BatchRejectMessageRequestsRequest {
	pub channel_ids: Vec<Snowflake>,
}

pub type BatchRejectMessageRequestsResponse = Vec<Channel>;

/// Type: get
pub fn GET_SUPPLEMENTAL_MESSAGE_REQUEST_DATA_ENDPOINT(channel_ids: Vec<Snowflake>) -> String {
	format!(
		"/users/@me/message-requests/supplemental-data?channel_ids={:?}",
		channel_ids
	)
}

pub type GetSupplementalMessageRequestDataResponse = Vec<SupplementalMessageRequest>;

/// Type: post
pub fn ACKNOWLEDGE_BLOCKED_USER_WARNING_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/blocked-user-warning-dismissal", channel_id)
}

/// Type: post
pub fn ACKNOWLEDGE_SAFETY_WARNINGS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/safety-warnings/ack", channel_id)
}

#[derive(Serialize)]
pub struct AcknowledgeSafetyWarningsRequest {
	warning_ids: Vec<Snowflake>,
}

/// Type: post
///
/// only usable by discord employees
pub fn ADD_SAFETY_WARNING_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/add-safety-warning", channel_id)
}

#[derive(Serialize)]
pub struct AddSafetyWarningRequest {
	/// https://docs.discord.food/resources/channel#safety-warning-type
	pub safety_warning_type: u8,
}

/// Type: delete
///
/// only usable by discord employees
pub fn DELETE_SAFETY_WARNINGS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/safety-warnings", channel_id)
}

/// Type: post
///
/// only usable by discord employees
pub fn REPORT_SAFETY_WARNING_FALSE_POSITIVE_ENDPOINT(channel_id: Snowflake) -> String {
	format!(
		"/channels/{}/safety-warning/report-false-positive",
		channel_id
	)
}

/// Type: get
///
/// not usable by user accounts
pub fn GET_GUILD_ACTIVE_THREADS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/threads/active", guild_id)
}

#[derive(Deserialize)]
pub struct GetGuildActiveThreadsResponse {
	pub threads: Vec<Channel>,
	pub members: Vec<ThreadMember>,
}

/// Type: get
///
/// not usable by user accounts
/// removed in api v10
#[deprecated]
pub fn GET_ACTIVE_THREADS_ENDPOINT(channel_id: Snowflake) -> String {
	if API_VERSION.to_string().parse::<u8>().unwrap() > 9 {
		panic!("Use of removed endpoint GET_ACTIVE_THREADS_ENDPOINT")
	}
	format!("/channels/{}/threads/active", channel_id)
}

#[derive(Deserialize)]
pub struct GetActiveThreadsResponse {
	pub threads: Vec<Channel>,
	pub members: Vec<ThreadMember>,
}

/// Type: get
///
/// requires READ_MESSAGE_HISTORY permission
#[deprecated]
pub fn GET_PUBLIC_ARCHIVED_THREADS_ENDPOINT(
	channel_id: Snowflake,
	query: GetPublicArchivedThreadsRequest,
) -> String {
	format!(
		"/channels/{}/threads/archived/public{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetPublicArchivedThreadsRequest {
	pub before: Timestamp,
	pub limit:  u8,
}

#[derive(Deserialize)]
pub struct GetPublicArchivedThreadsResponse {
	pub threads:  Vec<Channel>,
	pub members:  Vec<ThreadMember>,
	pub has_more: bool,
}

/// Type: get
///
/// requires READ_MESSAGE_HISTORY permission
#[deprecated]
pub fn GET_JOINED_PRIVATE_ARCHIVED_THREADS_ENDPOINT(
	channel_id: Snowflake,
	query: GetJoinedPrivateArchivedThreadsRequest,
) -> String {
	format!(
		"/channels/{}/users/@me/threads/archived/private{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetJoinedPrivateArchivedThreadsRequest {
	pub before: Timestamp,
	pub limit:  u8,
}

#[derive(Deserialize)]
pub struct GetJoinedPrivateArchivedThreadsResponse {
	pub threads:  Vec<Channel>,
	pub members:  Vec<ThreadMember>,
	pub has_more: bool,
}

/// Type: get
///
/// requires READ_MESSAGE_HISTORY permission
#[deprecated]
pub fn SEARCH_THREADS_ENDPOINT(
	channel_id: Snowflake,
	query: SearchThreadsRequest,
) -> String {
	format!(
		"/channels/{}/threads/search{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct SearchThreadsRequest {
	pub name:        String,
	pub slop:        u8,
	pub tag:         Vec<Snowflake>,
	/// match_some or match_all, default match_some
	pub tag_setting: String,
	pub archived:    bool,
	/// https://docs.discord.food/resources/channel#thread-sort-type
	pub sort_by:     String,
	/// asc or desc, default desc
	pub sort_order:  String,
	pub limit:       u8,
	pub offset:      u16,
	pub max_id:      Snowflake,
	pub min_id:      Snowflake,
}

#[derive(Deserialize)]
pub struct SearchThreadsResponse {
	pub threads:        Vec<Channel>,
	pub members:        Vec<ThreadMember>,
	pub has_more:       bool,
	pub total_results:  u16,
	pub first_messages: Vec<Message>,
}

/// Type: post
///
/// supports the X-Audit-Log-Reason header
pub fn CREATE_THREAD_FROM_MESSAGE_ENDPOINT(
	channel_id: Snowflake,
	message_id: Snowflake,
) -> String {
	format!("/channels/{}/messages/{}/threads", channel_id, message_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateThreadFromMessageRequest {
	pub name:                  String,
	pub auto_archive_duration: u16,
	pub rate_limit_per_user:   u16,
}

pub type CreateThreadFromMessageResponse = Channel;

/// Type: post
///
/// supports the X-Audit-Log-Reason header
/// requires CREATE_PUBLIC_THREADS or CREATE_PRIVATE_THREADS permission depending on the type created
pub fn CREATE_THREAD_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/threads", channel_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateThreadRequest {
	pub name:                  String,
	pub auto_archive_duration: u16,
	pub rate_limit_per_user:   u16,
	/// https://docs.discord.food/resources/channel#channel-type
	///
	/// required in api v10
	pub r#type:                u8,
	pub invitable:             bool,
	pub applied_tags:          Vec<Snowflake>,
	/// required in api v10
	pub message:               ThreadOnlyChannelMessageParams,
}

pub type CreateThreadResponse = Channel;

/// Type: post (what are you on discord)
///
/// requires READ_MESSAGE_HISTORY permission
pub fn GET_CHANNEL_POST_DATA_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/post-data", channel_id)
}

#[derive(Serialize)]
pub struct GetChannelPostDataRequest {
	pub thread_ids: Vec<Snowflake>,
}

#[derive(Deserialize)]
pub struct GetChannelPostDataResponse {
	pub threads: HashMap<Snowflake, ThreadPostData>,
}

/// Type: get
///
/// requires VIEW_CHANNEL permission
/// not usable by user accounts, and bot accounts also need GUILD_MEMBERS intent
/// in api v11 this will always return paginated results
/// before that though paginated results can be returned by setting with_member to true
pub fn GET_THREAD_MEMBERS_ENDPOINT(
	channel_id: Snowflake,
	query: GetThreadMembersRequest,
) -> String {
	format!(
		"/channels/{}/post-data{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetThreadMembersRequest {
	pub with_member: bool,
	pub after:       Snowflake,
	pub limit:       u8,
}

pub type GetThreadMembersResponse = Vec<ThreadMember>;

/// Type: get
///
/// requires VIEW_CHANNEL permission
/// not usable by user accounts
pub fn GET_THREAD_MEMBER_ENDPOINT(
	channel_id: Snowflake,
	user_id: Snowflake,
	query: GetThreadMemberRequest,
) -> String {
	format!(
		"/channels/{}/thread-members/{}{}",
		channel_id,
		user_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetThreadMemberRequest {
	pub with_member: bool,
}

/// Type: put
///
/// requires VIEW_CHANNEL permission
pub fn JOIN_THREAD_ENDPOINT(channeL_id: Snowflake) -> String {
	format!("/channels/{}/thread-members/@me", channeL_id)
}

/// Type: put
///
/// requires SEND_MESSAGES permission
pub fn ADD_THREAD_MEMBER_ENDPOINT(
	channeL_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/channels/{}/thread-members/{}", channeL_id, user_id)
}

/// Type: patch
pub fn MODIFY_THREAD_SETTINGS_ENDPOINT(channeL_id: Snowflake) -> String {
	format!("/channels/{}/thread-members/@me/settings", channeL_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyThreadSettingsRequest {
	/// https://docs.discord.food/resources/channel#thread-member-flags
	///
	/// all but the first flag can be set
	pub flags:       u64,
	pub muted:       bool,
	pub mute_config: Option<MuteConfig>,
}

/// Type: delete
///
/// requires VIEW_CHANNEL permission
pub fn LEAVE_THREAD_ENDPOINT(channeL_id: Snowflake) -> String {
	format!("/channels/{}/thread-members/@me", channeL_id)
}

/// Type: delete
///
/// requires MANAGE_THREADS permission
pub fn REMOVE_THREAD_MEMBER_ENDPOINT(
	channeL_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/channels/{}/thread-members/{}", channeL_id, user_id)
}

/// Type: post
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_CHANNELS permission
pub fn CREATE_CHANNEL_TAG_ENDPOINT(channeL_id: Snowflake) -> String {
	format!("/channels/{}/tags", channeL_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateChannelTagRequest {
	pub name:       String,
	pub moderated:  bool,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

pub type CreateChannelTagResponse = Channel;

/// Type: put
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_CHANNELS permission
pub fn MODIFY_CHANNEL_TAG_ENDPOINT(
	channeL_id: Snowflake,
	tag_id: Snowflake,
) -> String {
	format!("/channels/{}/tags/{}", channeL_id, tag_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyChannelTagRequest {
	pub name:       String,
	pub moderated:  bool,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
}

pub type ModifyChannelTagResponse = Channel;

/// Type: delete
///
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_CHANNELS permission
pub fn DELETE_CHANNEL_TAG_ENDPOINT(
	channeL_id: Snowflake,
	tag_id: Snowflake,
) -> String {
	format!("/channels/{}/tags/{}", channeL_id, tag_id)
}

pub type DeleteChannelTagResponse = Channel;
