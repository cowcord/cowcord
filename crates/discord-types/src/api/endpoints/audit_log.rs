#![allow(non_snake_case)]

use serde::Serialize;

use crate::api::audit_log::AuditLog;
use crate::api::types::Snowflake;

/// Type: get
pub fn GET_GUILD_AUDIT_LOG_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/audit-logs", guild_id)
}

#[derive(Serialize)]
pub struct GetGuildAuditLogRequest {
	pub before:      Snowflake,
	pub after:       Snowflake,
	pub limit:       u8,
	pub user_id:     Snowflake,
	pub target_id:   Snowflake,
	/// https://docs.discord.food/resources/audit-log#audit-log-events
	pub action_type: u8,
}

pub type GetGuildAuditLogResponse = AuditLog;
