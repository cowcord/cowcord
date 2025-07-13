#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::models::api_types::auto_moderation::{AutomodAction, AutomodRule, AutomodTriggerMetadata};
use crate::models::api_types::types::{Snowflake, Timestamp};

/// Type: get
///
/// requires MANAGE_GUILD permission
pub fn GET_GUILD_AUTOMOD_RULES_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/rules", guild_id)
}

pub type GetGuildAutomodRulesResponse = Vec<AutomodRule>;

/// Type: get
///
/// supports the X-Audit-Log-Reason header
pub fn GET_GUILD_AUTOMOD_RULE_ENDPOINT(
	guild_id: Snowflake,
	rule_id: Snowflake,
) -> String {
	format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id)
}

pub type GetGuildAutomodRuleResponse = AutomodRule;

/// Type: post
///
/// requires MANAGE_GUILD permission
///
/// requires MANAGE_GUILD permission
pub fn CREATE_GUILD_AUTOMOD_RULE_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/rules", guild_id)
}

#[derive(Serialize)]
pub struct CreateGuildAutomodRuleRequest {
	pub name:             String,
	/// https://docs.discord.food/resources/auto-moderation#automod-event-type
	pub event_type:       u8,
	/// https://docs.discord.food/resources/auto-moderation#automod-trigger-type
	pub trigger_type:     u8,
	/// see the trigger types in https://docs.discord.food/resources/auto-moderation#automod-trigger-metadata for when to include this
	pub trigger_metadata: AutomodTriggerMetadata,
	pub actions:          Vec<AutomodAction>,
	pub enabled:          bool,
	pub exempt_roles:     Vec<Snowflake>,
	pub exempt_channels:  Vec<Snowflake>,
}

pub type CreateGuildAutomodRuleResponse = AutomodRule;

/// Type: patch
///
/// requires MANAGE_GUILD permission
///
/// supports the X-Audit-Log-Reason header
pub fn MODIFY_GUILD_AUTOMOD_RULE_ENDPOINT(
	guild_id: Snowflake,
	rule_id: Snowflake,
) -> String {
	format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id)
}

pub type ModifyGuildAutomodRuleRequest = CreateGuildAutomodRuleRequest;

pub type ModifyGuildAutomodRuleResponse = AutomodRule;

/// Type: delete
///
/// requires MANAGE_GUILD permission
///
/// supports the X-Audit-Log-Reason header
pub fn DELETE_GUILD_AUTOMOD_RULE_ENDPOINT(
	guild_id: Snowflake,
	rule_id: Snowflake,
) -> String {
	format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id)
}

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn VALIDATE_GUILD_AUTOMOD_RULE_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/rules/validate", guild_id)
}

#[derive(Serialize)]
pub struct ValidateGuildAutomodRuleRequest {
	/// see the trigger types in https://docs.discord.food/resources/auto-moderation#automod-trigger-metadata for when to include this
	pub trigger_metadata: AutomodTriggerMetadata,
}

#[derive(Deserialize)]
pub struct ValidateGuildAutomodRuleResponse {
	pub trigger_metadata: AutomodTriggerMetadata,
}

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn EXECUTE_AUTOMOD_ALERT_ACTION_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/alert-action", guild_id)
}

#[derive(Serialize)]
pub struct ExecuteAutomodAlertActionRequest {
	pub channel_id:        Snowflake,
	pub message_id:        Snowflake,
	/// https://docs.discord.food/resources/auto-moderation#automod-alert-action-type
	pub alert_action_type: u8,
}

/// Type: put
///
/// requires MANAGE_GUILD permission
pub fn MODIFY_AUTOMOD_INCIDENTS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/incident-actions", guild_id)
}

#[derive(Serialize)]
pub struct ModifyGuildAutomodIncidentsRequest {
	pub invites_disabled_until: Timestamp,
	pub dms_disabled_until:     Timestamp,
}

#[derive(Deserialize)]
pub struct ModifyGuildAutomodIncidentsResponse {
	pub invites_disabled_until: Timestamp,
	pub dms_disabled_until:     Timestamp,
}

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn RESOLVE_AUTOMOD_INCIDENT_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/false-alarm", guild_id)
}

#[derive(Serialize)]
pub struct ResolveAutomodIncidentRequest {
	pub alert_message_id: Snowflake,
	pub reason:           String,
}

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn REPORT_AUTOMOD_INCIDENT_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/report-raid", guild_id)
}

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn CLEAR_MENTION_RAID_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/auto-moderation/clear-mention-raid", guild_id)
}
