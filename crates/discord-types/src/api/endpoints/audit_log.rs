use serde::{Deserialize, Serialize};

use crate::api::audit_log::{AuditLogActionType, AuditLogEntry, PartialIntegration};
use crate::api::auto_moderation::AutomodRule;
use crate::api::channel::Channel;
use crate::api::guild_scheduled_event::GuildScheduledEvent;
use crate::api::users::PartialUser;
use crate::api::webhooks::Webhook;
use crate::common::id::{AuditLogEntryId, GuildId, UserId};
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Requires the `VIEW_AUDIT_LOG` permission.
///
/// Returns the audit log for the guild.
pub fn GET_GUILD_AUDIT_LOG(
	query: &GetGuildAuditLogQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/audit-logs{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildAuditLogQueryParams {
	/// Get entries before this entry ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<AuditLogEntryId>,
	/// Get entries after this entry ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<AuditLogEntryId>,
	/// Max number of entries to return (1-100, default 50)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Get actions made by a specific user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<UserId>,
	/// Get actions affecting a specific entity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_id: Option<AuditLogEntryId>,
	/// The type of audit log event to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action_type: Option<AuditLogActionType>,
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildAuditLogResponse {
	/// Audit log entries
	pub audit_log_entries: Vec<AuditLogEntry>,
	/// Application commands referenced in the audit log
	pub application_commands: Vec<ApplicationCommand>,
	/// AutoMod rules referenced in the audit log
	pub auto_moderation_rules: Vec<AutomodRule>,
	/// Guild scheduled events referenced in the audit log
	pub guild_scheduled_events: Vec<GuildScheduledEvent>,
	/// Partial integrations referenced in the audit log
	pub integrations: Vec<PartialIntegration>,
	/// Threads referenced in the audit log
	pub threads: Vec<Channel>,
	/// Users referenced in the audit log
	pub users: Vec<PartialUser>,
	/// Webhooks referenced in the audit log
	pub webhooks: Vec<Webhook>,
}
