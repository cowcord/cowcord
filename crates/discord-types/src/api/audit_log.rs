use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::channel::PermissionOverwriteType;
use crate::api::integrations::{IntegrationAccount, IntegrationType};
use crate::common::id::{
	ApplicationId,
	AuditLogEntryId,
	ChannelId,
	GenericSnowflake,
	IntegrationId,
	MessageId,
	RoleId,
	ScheduledEventId,
	UserId,
};

#[derive(Serialize, Deserialize)]
pub struct AuditLogEntry {
	/// ID of the affected entity (webhook, user, role, etc.)
	pub target_id: Option<GenericSnowflake>,
	/// Changes made to the `target_id``
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes: Option<Vec<AuditLogChange>>,
	/// The user who made the changes
	pub user_id: Option<UserId>,
	/// The ID of the entry
	pub id: AuditLogEntryId,
	/// The type of action that occurred
	pub action_type: AuditLogActionType,
	/// Additional info for certain action types
	#[serde(skip_serializing_if = "Option::is_none")]
	pub options: Option<OptionalAuditEntryInfo>,
	/// The reason for the change (max 512 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String>,
}

/// You should assume that you may run into any field for the changed object, though none are guaranteed to be present. In most cases only a subset of the object's fields will be in the changes array.
/// If no object is noted, there won't be a changes array in the entry, though other fields like the target_id still exist and many have fields in the [options array](https://docs.discord.food/resources/audit-log#optional-audit-entry-info).
/// The Object Changed column notes which object's values may be included in the entry. Though there are exceptions, possible keys in the changes array typically correspond to the object's fields. The descriptions and types for those fields can be found in the linked documentation for the object.
/// The table below lists audit log events and values (the action_type field) that you may receive.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AuditLogActionType {
	/// Guild settings were updated
	GUILD_UPDATE = 1,
	/// Channel was created
	CHANNEL_CREATE = 10,
	/// Channel settings were updated
	CHANNEL_UPDATE = 11,
	/// Channel was deleted
	CHANNEL_DELETE = 12,
	/// Permission overwrite was added to a channel
	CHANNEL_OVERWRITE_CREATE = 13,
	/// Permission overwrite was updated for a channel
	CHANNEL_OVERWRITE_UPDATE = 14,
	/// Permission overwrite was deleted from a channel
	CHANNEL_OVERWRITE_DELETE = 15,
	/// Member was removed from guild
	MEMBER_KICK = 20,
	/// Members were pruned from guild
	MEMBER_PRUNE = 21,
	/// Member was banned from guild
	MEMBER_BAN_ADD = 22,
	/// Member was unbanned from guild
	MEMBER_BAN_REMOVE = 23,
	/// Member was updated in guild
	MEMBER_UPDATE = 24,
	/// Member was added or removed from a role
	MEMBER_ROLE_UPDATE = 25,
	/// Member was moved to a different voice channel
	MEMBER_MOVE = 26,
	/// Member was disconnected from a voice channel
	MEMBER_DISCONNECT = 27,
	/// Bot user was added to guild
	BOT_ADD = 28,
	/// Role was created
	ROLE_CREATE = 30,
	/// Role was edited
	ROLE_UPDATE = 31,
	/// Role was deleted
	ROLE_DELETE = 32,
	/// Guild invite was created
	INVITE_CREATE = 40,
	/// Guild invite was updated
	INVITE_UPDATE = 41,
	/// Guild invite was deleted
	INVITE_DELETE = 42,
	/// Webhook was created
	WEBHOOK_CREATE = 50,
	/// Webhook properties or channel were updated
	WEBHOOK_UPDATE = 51,
	/// Webhook was deleted
	WEBHOOK_DELETE = 52,
	/// Emoji was created
	EMOJI_CREATE = 60,
	/// Emoji name was updated
	EMOJI_UPDATE = 61,
	/// Emoji was deleted
	EMOJI_DELETE = 62,
	/// Single message was deleted
	MESSAGE_DELETE = 72,
	/// Multiple messages were deleted
	MESSAGE_BULK_DELETE = 73,
	/// Message was pinned to a channel
	MESSAGE_PIN = 74,
	/// Message was unpinned from a channel
	MESSAGE_UNPIN = 75,
	/// Integration was added to guild
	INTEGRATION_CREATE = 80,
	/// Integration was updated (e.g. its scopes were updated)
	INTEGRATION_UPDATE = 81,
	/// Integration was removed from guild
	INTEGRATION_DELETE = 82,
	/// Stage instance was created (stage channel becomes live)
	STAGE_INSTANCE_CREATE = 83,
	/// Stage instance details were updated
	STAGE_INSTANCE_UPDATE = 84,
	/// Stage instance was deleted (stage channel no longer live)
	STAGE_INSTANCE_DELETE = 85,
	/// Sticker was created
	STICKER_CREATE = 90,
	/// Sticker details were updated
	STICKER_UPDATE = 91,
	/// Sticker was deleted
	STICKER_DELETE = 92,
	/// Event was created
	GUILD_SCHEDULED_EVENT_CREATE = 100,
	/// Event was updated
	GUILD_SCHEDULED_EVENT_UPDATE = 101,
	/// Event was cancelled
	GUILD_SCHEDULED_EVENT_DELETE = 102,
	/// Thread was created in a channel
	THREAD_CREATE = 110,
	/// Thread was updated
	THREAD_UPDATE = 111,
	/// Thread was deleted
	THREAD_DELETE = 112,
	/// Permissions were updated for a command
	APPLICATION_COMMAND_PERMISSION_UPDATE = 121,
	/// Soundboard sound was created
	SOUNDBOARD_SOUND_CREATE = 130,
	/// Soundboard sound was updated
	SOUNDBOARD_SOUND_UPDATE = 131,
	/// Soundboard sound was deleted
	SOUNDBOARD_SOUND_DELETE = 132,
	/// AutoMod rule was created
	AUTO_MODERATION_RULE_CREATE = 140,
	/// AutoMod rule was updated
	AUTO_MODERATION_RULE_UPDATE = 141,
	/// AutoMod rule was deleted
	AUTO_MODERATION_RULE_DELETE = 142,
	/// Message was blocked by AutoMod
	AUTO_MODERATION_BLOCK_MESSAGE = 143,
	/// Message was flagged by AutoMod
	AUTO_MODERATION_FLAG_TO_CHANNEL = 144,
	/// Member was timed out by AutoMod
	AUTO_MODERATION_USER_COMMUNICATION_DISABLED = 145,
	/// Member was quarantined by AutoMod
	AUTO_MODERATION_QUARANTINE_USER = 146,
	/// Creator monetization request was created
	CREATOR_MONETIZATION_REQUEST_CREATED = 150,
	/// Creator monetization terms were accepted
	CREATOR_MONETIZATION_TERMS_ACCEPTED = 151,
	/// Onboarding prompt was created
	ONBOARDING_PROMPT_CREATE = 163,
	/// Onboarding prompt was updated
	ONBOARDING_PROMPT_UPDATE = 164,
	/// Onboarding prompt was deleted
	ONBOARDING_PROMPT_DELETE = 165,
	/// Onboarding was created
	ONBOARDING_CREATE = 166,
	/// Onboarding was updated
	ONBOARDING_UPDATE = 167,
	/// Voice channel status was updated
	VOICE_CHANNEL_STATUS_CREATE = 192,
	/// Voice channel status was deleted
	VOICE_CHANNEL_STATUS_DELETE = 193,
	/// Exception was created for a guild scheduled event
	GUILD_SCHEDULED_EVENT_EXCEPTION_CREATE = 200,
	/// Exception was updated for a guild scheduled event
	GUILD_SCHEDULED_EVENT_EXCEPTION_UPDATE = 201,
	/// Exception was deleted for a guild scheduled event
	GUILD_SCHEDULED_EVENT_EXCEPTION_DELETE = 202,
	/// Member verification settings were updated
	GUILD_MEMBER_VERIFICATION_UPDATE = 210,
	/// Guild profile was updated
	GUILD_PROFILE_UPDATE = 211,
}

#[derive(Serialize, Deserialize)]
pub struct OptionalAuditEntryInfo {
	/// The ID of the application whose permissions were targeted
	pub application_id: ApplicationId,
	/// The name of the AutoMod rule that was triggered
	pub auto_moderation_rule_name: String,
	/// The trigger type of the AutoMod rule that was triggered
	pub auto_moderation_rule_trigger_type: String,
	/// The channel in which the entities were targeted
	pub channel_id: ChannelId,
	/// Number of entities that were targeted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub count: Option<String>,
	/// Number of days after which inactive members were kicked
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delete_member_days: Option<String>,
	/// The ID of the guild scheduled event exception that was targeted
	pub event_exception_id: ScheduledEventId,
	/// The ID of the overwritten entity
	pub id: GenericSnowflake,
	/// The type of integration which performed the action
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_type: Option<String>,
	/// Number of members removed by the prune
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members_removed: Option<String>,
	/// The ID of the message that was targeted
	pub message_id: MessageId,
	/// The name of the role (only present if type is "0")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_name: Option<String>,
	/// The new status of the voice channel
	pub status: String,
	/// The type of overwritten entity
	pub r#type: PermissionOverwriteType,
}

/// If new_value is not present in the change object while old_value is, it indicates that the property has been reset or set to null. If old_value isn't included, it indicated that the property was previously null.
/// Some events don't follow the same pattern as other audit log events. Details about these exceptions are explained in [the next section](https://docs.discord.food/resources/audit-log#audit-log-change-exceptions).
#[derive(Serialize, Deserialize)]
pub struct AuditLogChange {
	/// New value of the key
	/// mixed types, matches object field's type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_value: Option<Value>,
	/// Old value of the key
	/// mixed types, matches object field's type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub old_value: Option<Value>,
	/// Name of the changed entity, with a few exceptions
	pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartialIntegration {
	/// The ID of the integration
	pub id: IntegrationId,
	/// The name of the integration
	pub name: String,
	/// The type of integration
	pub r#type: IntegrationType,
	/// The integration's account information
	pub account: IntegrationAccount,
	/// The OAuth2 application for Discord integrations
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<ApplicationId>,
}

#[derive(Serialize, Deserialize)]
pub struct PartialRole {
	/// The ID of the role
	pub id: RoleId,
	/// The name of the role
	pub name: String,
}
