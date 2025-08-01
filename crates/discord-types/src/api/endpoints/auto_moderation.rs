use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::api::auto_moderation::{
	AutomodAction,
	AutomodAlertActionType,
	AutomodEventType,
	AutomodRaidResolutionReason,
	AutomodRule,
	AutomodTriggerMetadata,
	AutomodTriggerType,
};
use crate::common::id::{AutomodMessageId, AutomodRuleId, ChannelId, GuildId, MessageId, RoleId};
use crate::common::timestamp::Timestamp;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a list of [automod rule](https://docs.discord.food/resources/auto-moderation#automod-rule-object) objects for the configured rules in the guild.
pub fn GET_GUILD_AUTOMOD_RULES(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/rules", guild_id)
}

pub type GetGuildAutomodRules = Vec<AutomodRule>;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns an [automod rule](https://docs.discord.food/resources/auto-moderation#automod-rule-object) object for the given rule ID in the guild.
pub fn GET_GUILD_AUTOMOD_RULE(
	guild_id: &GuildId,
	automod_rule_id: &AutomodRuleId,
) -> String {
	format!(
		"/guilds/{}/auto-moderation/rules/{}",
		guild_id, automod_rule_id
	)
}

pub type GetGuildAutomodRule = AutomodRule;

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Creates a new automod rule in the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// See [trigger types](https://docs.discord.food/resources/auto-moderation#automod-trigger-type) for limits on how many rules of each trigger type can be created per guild.
///
/// Returns an [automod rule](https://docs.discord.food/resources/auto-moderation#automod-rule-object) on success. Fires an [Auto Moderation Rule Create](https://docs.discord.food/topics/gateway-events#auto-moderation-rule-create) Gateway event.
pub fn CREATE_GUILD_AUTOMOD_RULE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/rules", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildAutomodRuleRequest {
	/// The name of the rule
	pub name: String,
	/// The type of event that triggers the rule
	pub event_type: AutomodEventType,
	/// The type of trigger that invokes the rule
	pub trigger_type: AutomodTriggerType,
	/// Metadata used to determine whether the rule should be triggered
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trigger_metadata: Option<AutomodTriggerMetadata>,
	/// The actions that will execute when the rule is triggered
	pub actions: Vec<AutomodAction>,
	/// Whether the rule is enabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// The IDs of the roles that won't be affected by the rule (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exempt_roles: Option<ArrayVec<RoleId, 20>>,
	/// The IDs of the channels that won't be affected by the rule (max 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exempt_channels: Option<ArrayVec<ChannelId, 50>>,
}

pub type CreateGuildAutomodRuleResponse = AutomodRule;

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Validates a potential rule request's schema for the guild.
pub fn VALIDATE_GUILD_AUTOMOD_RULE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/rules/validate", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ValidateGuildAutomodRuleRequest {
	/// The trigger metadata to validate
	pub trigger_metadata: AutomodTriggerMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateGuildAutomodRuleResponse {
	/// The validated trigger metadata
	pub trigger_metadata: AutomodTriggerMetadata,
}

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies an existing rule in the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns an [automod rule](https://docs.discord.food/resources/auto-moderation#automod-rule-object) on success.
/// Fires an [Auto Moderation Rule Update](https://docs.discord.food/topics/gateway-events#auto-moderation-rule-update) Gateway event.
pub fn MODIFY_GUILD_AUTOMOD_RULE(
	guild_id: &GuildId,
	automod_rule_id: &AutomodRuleId,
) -> String {
	format!(
		"/guilds/{}/auto-moderation/rules/{}",
		guild_id, automod_rule_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildAutomodRuleRequest {
	/// The name of the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The type of event that triggers the rule
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_type: Option<AutomodEventType>,
	/// Metadata used to determine whether the rule should be triggered
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trigger_metadata: Option<AutomodTriggerMetadata>,
	/// The actions that will execute when the rule is triggered
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actions: Option<Vec<AutomodAction>>,
	/// Whether the rule is enabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// The IDs of the roles that won't be affected by the rule (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exempt_roles: Option<ArrayVec<RoleId, 20>>,
	/// The IDs of the channels that won't be affected by the rule (max 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exempt_channels: Option<ArrayVec<ChannelId, 50>>,
}

pub type ModifyGuildAutomodRuleResponse = AutomodRule;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Deletes a rule in the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a 204 empty response on success.
/// Fires an [Auto Moderation Rule Delete](https://docs.discord.food/topics/gateway-events#auto-moderation-rule-delete) Gateway event.
pub fn DELETE_GUILD_AUTOMOD_RULE(
	guild_id: &GuildId,
	automod_rule_id: &AutomodRuleId,
) -> String {
	format!(
		"/guilds/{}/auto-moderation/rules/{}",
		guild_id, automod_rule_id
	)
}

/// Method: `POST`
///
/// Executes an alert action on an [AutoMod alert](https://docs.discord.food/resources/auto-moderation#automod-alert).
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a 204 empty response on success.
/// Fires a [Message Update](https://docs.discord.food/topics/gateway-events#message-update) Gateway event.
pub fn EXECUTE_AUTOMOD_ALERT_ACTION(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/alert-action", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteAutomodAlertActionRequest {
	/// The ID of the channel where the alert was sent
	pub channel_id: ChannelId,
	/// The ID of the AutoMod system message
	pub message_id: MessageId,
	/// The type of alert action to execute
	pub alert_action_type: AutomodAlertActionType,
}

/// Method: `PUT`
///
/// Sets the incident actions for the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_AUTOMOD_INCIDENT_ACTIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/incident-actions", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyAutomodIncidentActionsRequest {
	/// When invites will be re-enabled (max 24 hours from now)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invites_disabled_until: Option<Timestamp>,
	/// When DMs will be re-enabled (max 24 hours from now)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dms_disabled_until: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyAutomodIncidentActionsResponse {
	/// When invites will be re-enabled (max 24 hours from now)
	pub invites_disabled_until: Timestamp,
	/// When DMs will be re-enabled (max 24 hours from now)
	pub dms_disabled_until: Timestamp,
}

/// Method: `POST`
///
/// Resolves an [AutoMod incident](https://docs.discord.food/resources/auto-moderation#automod-incident-notification).
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a 204 empty response on success.
/// Fires a [Message Update](https://docs.discord.food/topics/gateway-events#message-update) Gateway event.
pub fn RESOLVE_AUTOMOD_INCIDENT(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/false-alarm", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ResolveAutomodIncidentRequest {
	/// The ID of the AutoMod system message
	pub alert_message_id: AutomodMessageId,
	/// The reason for resolving the notification
	pub reason: AutomodRaidResolutionReason,
}

/// Method: `POST`
///
/// Reports an ongoing raid [AutoMod incident](https://docs.discord.food/resources/auto-moderation#automod-incident-notification).
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a 204 empty response on success.
/// Fires a [Message Create](https://docs.discord.food/topics/gateway-events#message-update) Gateway event.
pub fn REPORT_AUTOMOD_INCIDENT(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/report-raid", guild_id)
}

/// Method: `POST`
///
/// Clears a mention raid [AutoMod incident](https://docs.discord.food/resources/auto-moderation#automod-incident-notification).
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a 204 empty response on success.
pub fn CLEAR_MENTION_RAID_INCIDENT(guild_id: &GuildId) -> String {
	format!("/guilds/{}/auto-moderation/clear-mention-raid", guild_id)
}
