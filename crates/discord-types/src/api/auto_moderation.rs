use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::id::{AutomodRuleId, ChannelId, GuildId, MessageId, RoleId, UserId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct AutomodAlertEmbed {
	/// The name of the automod rule that was triggered
	pub rule_name: String,
	/// The ID of the decision that was executed
	pub decision_id: String,
	/// The reason for the decision that was executed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub decision_reason: Option<String>,
	/// The outcome of the decision that triggered the rule
	pub decision_outcome: String,
	/// The ID of the channel in which the user content was posted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<ChannelId>,
	/// The ID of the message that triggered the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flagged_message_id: Option<MessageId>,
	/// The word or phrase configured that triggered the rule
	pub keyword: String,
	/// The substring in content that triggered the rule
	pub keyword_matched_content: String,
	/// The type of profile update that was blocked
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_profile_update_type: Option<String>,
	/// The reason for quarantining the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quarantine_user: Option<String>,
	/// The action taken on the quarantined user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quarantine_user_action: Option<String>,
	/// The user action that triggered the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quarantine_event: Option<String>,
	/// The outcome of the voice channel status update that triggered the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice_channel_status_outcome: Option<String>,
	/// The name of the user application that triggered the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_name: Option<String>,
	/// The ID of the user that triggered the rule, if the author is a user application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interaction_user_id: Option<UserId>,
	/// The type of interaction callback that triggered the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interaction_callback_type: Option<String>,
	/// Duration (in seconds) after which the timeout expires
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout_duration: Option<u64>,
	/// The actions that were executed on the AutoMod alert
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alert_actions_execution: Option<AutomodAlertActionsExecution>,
}

pub enum AutomodDecisionOutcome {
	/// The action was flagged by AutoMod
	flagged,
	/// The action was blocked by AutoMod
	blocked,
}

pub enum AutomodProfileUpdateType {
	/// When a user updates their nickname in the guild
	nickname_update,
	/// When a user resets their nickname in the guild
	nickname_reset,
}

pub enum AutomodQuarantineUserReason {
	/// The user's username triggered the rule
	username,
	/// The user's display name triggered the rule
	display_name,
	/// The user's guild nickname triggered the rule
	nickname,
	/// The user's clan tag triggered the rule
	clan_tag,
}

pub enum AutomodQuarantineEventType {
	/// When a user joins the guild
	guild_join,
	/// When a user sends a message in the guild
	message_send,
	/// When a user updates their username
	username_update,
	/// When a user updates their clan tag
	clan_tag_update,
}

pub enum AutomodInteractionCallbackType {
	/// A modal interaction callback triggered the rule
	modal,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodAlertActionsExecution {
	/// The alert actions execution protocol version (currently 0)
	pub v: u8,
	/// The actions that were executed on the AutoMod alert, keyed by action type
	pub actions: HashMap<String, AutomodAlertAction>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodAlertActionType {
	/// Marks the alert as completed
	SET_COMPLETED = 1,
	/// Marks the alert as not completed
	UNSET_COMPLETED = 2,
	/// Deletes the user message that triggered the alert
	DELETE_USER_MESSAGE = 3,
	/// Reports an issue with the alert to Discord
	SUBMIT_FEEDBACK = 4,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodAlertAction {
	/// The ID of the user that executed the action
	pub actor: UserId,
	/// ts pmo 🥀
	///
	/// When the action was executed
	pub ts: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodIncidentNotificationEmbed {
	/// The type of notification that was triggered (default raid )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub notification_type: Option<String>,
	/// The ID of the decision that was executed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub decision_id: Option<String>,
	/// The ID of the user that executed the action (only applicable to activity_alerts_enabled  notification types )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action_by_user_id: Option<UserId>,
	/// The type of raid that was detected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub raid_type: Option<String>,
	/// When the raid was detected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub raid_datetime: Option<Timestamp>,
	/// The approximate number of join attempts as part of the raid
	#[serde(skip_serializing_if = "Option::is_none")]
	pub join_attempts: Option<u16>,
	/// The approximate number of sent DMs as part of the raid
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dms_sent: Option<u16>,
	/// When the mention activity restrictions will end (only applicable to mention_raid  notification types )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suspicious_mention_activity_until: Option<Timestamp>,
	/// The reason for resolving the notification
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resolved_reason: Option<String>,
}

pub enum AutomodIncidentNotificationType {
	/// Activity alerts were enabled in the guild
	activity_alerts_enabled,
	/// A raid was detected
	raid,
	/// A mention raid was detected
	mention_raid,
	/// An anonymous interaction response was blocked
	interaction_blocked,
}

pub enum AutomodRaidType {
	/// A join raid was detected
	JOIN_RAID,
	/// A mention raid was detected
	MENTION_RAID,
}

pub enum AutomodRaidResolutionReason {
	/// The increased activity was expected
	LEGITIMATE_ACTIVITY,
	/// The increased activity was caused by legitimate accounts
	LEGITIMATE_ACCOUNTS,
	/// The increased activity was caused by legitimate DMs
	LEGITIMATE_DMS,
	/// The increased activity was caused by DM spam and the spammers were removed
	DM_SPAM,
	/// The increased activity was caused by a join raid and the raiders were removed
	JOIN_RAID,
	/// The increased activity was caused by another reason
	OTHER,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodRule {
	/// The ID of the rule
	pub id: AutomodRuleId,
	/// The ID of the guild which this rule belongs to
	pub guild_id: GuildId,
	/// The name of the rule
	pub name: String,
	/// The ID of the user that created the rule
	pub creator_id: UserId,
	/// The type of event that triggers the rule
	pub event_type: AutomodEventType,
	/// The type of trigger that invokes the rule
	pub trigger_type: AutomodTriggerType,
	/// Metadata used to determine whether the rule should be triggered
	pub trigger_metadata: AutomodTriggerMetadata,
	/// The actions that will execute when the rule is triggered
	pub actions: Vec<AutomodAction>,
	/// Whether the rule is enabled
	pub enabled: bool,
	/// The IDs of the roles that won't be affected by the rule (max 20)
	pub exempt_roles: Vec<RoleId>,
	/// The IDs of the channels that won't be affected by the rule (max 50)
	pub exempt_channels: Vec<ChannelId>,
}

/// Characterizes the type of content which can trigger the rule.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodTriggerType {
	/// When message content contains words from a user defined list of keywords (max 6)
	KEYWORD = 1,
	/// When message content represents generic spam (max 1)
	SPAM = 3,
	/// When message content contains words from internal predefined wordsets (max 1)
	KEYWORD_PRESET = 4,
	/// When message content contains more unique mentions than allowed (max 1)
	MENTION_SPAM = 5,
	/// When a user's profile contains words from a user defined list of keywords (max 1)
	USER_PROFILE = 6,
	/// When a user violates the guild rules (max 1)
	GUILD_POLICY = 7,
}

/// Additional data used to determine whether a rule should be triggered. Different fields are relevant based on the [trigger type](https://docs.discord.food/resources/auto-moderation#automod-trigger-type).
#[derive(Serialize, Deserialize)]
pub struct AutomodTriggerMetadata {
	/// Substrings which will be searched for in content (1-60 characters, max 1000)
	pub keyword_filter: Vec<String>,
	/// Regular expression patterns which will be matched against content (1-260 characters, max 10)
	pub regex_patterns: Vec<String>,
	/// The internally predefined wordsets which will be searched for in content
	pub presets: Vec<AutomodKeywordPresetType>,
	/// Substrings which should not trigger the rule (1-60 characters, max 100 or 1000 respectively)
	pub allow_list: Vec<String>,
	/// Number of unique role and user mentions allowed per message (max 50)
	pub mention_total_limit: u8,
	/// Whether to automatically detect mention raids
	pub mention_raid_protection_enabled: bool,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodKeywordPresetType {
	/// Words that may be considered forms of swearing or cursing
	PROFANITY = 1,
	/// Words that refer to sexually explicit behavior or activity
	SEXUAL_CONTENT = 2,
	/// Personal insults or words that may be considered hate speech
	SLURS = 3,
}

/// Indicates in what event context a rule should be checked.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodEventType {
	/// When a member sends or edits a message in the guild
	MESSAGE_SEND = 1,
	/// When a member joins or updates their profile
	GUILD_MEMBER_EVENT = 2,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodAction {
	/// The type of action
	pub r#type: AutomodActionType,
	/// Additional metadata needed during execution for this specific action type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<AutomodActionMetadata>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutomodActionType {
	/// Block a member's message and prevent it from being posted; a custom explanation can be specified and shown to members whenever their message is blocked
	BLOCK_MESSAGE = 1,
	/// Log user content to a specified channel
	SEND_ALERT_MESSAGE = 2,
	/// Timeout user for a specified duration
	TIMEOUT_USER = 3,
	/// Block guild join, profile update, or quarantine user indefinitely; quarantined users, similar to timed out users, are prevented from interacting with the guild in any way
	QUARANTINE_USER = 4,
}

/// Additional data used when an action is executed. Different fields are relevant based on the [action type](https://docs.discord.food/resources/auto-moderation#automod-action-type).
#[derive(Serialize, Deserialize)]
pub struct AutomodActionMetadata {
	/// The channel where user content should be logged
	pub channel_id: ChannelId,
	/// Duration (in seconds) after which the timeout expires (max 2419200)
	pub duration_seconds: u32,
	/// Additional explanation that will be shown to members whenever their message is blocked
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AutomodIncidentsData {
	/// When the last raid was detected
	pub raid_detected_at: Option<Timestamp>,
	/// When the last DM spam was detected
	pub dm_spam_detected_at: Option<Timestamp>,
	/// When invites will be re-enabled (max 24 hours from now)
	pub invites_disabled_until: Option<Timestamp>,
	/// When DMs will be re-enabled (max 24 hours from now)
	pub dms_disabled_until: Option<Timestamp>,
}
