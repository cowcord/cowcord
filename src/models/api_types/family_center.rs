use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{api_types::{guild::Guild, users::PartialUser}, types::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct FamilyCenter {
	/// List of linked users
	pub linked_users: Vec<LinkedUser>,
	/// Audit log of the linked users activity
	pub teen_audit_log: TeenAuditLog,
	/// List of requestors the linked user is connected to
	pub users: Vec<PartialUser>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedUser {
	/// When the link request was sent
	pub created_at: Timestamp,
	/// When the link status was last updated
	pub updated_at: Timestamp,
	/// The link status of the linked user
	pub link_status: i64,
	/// The link type
	pub link_type: i64,
	/// The ID of the account the linked user is connected to
	pub requestor_id: Snowflake,
	/// The ID of the linked user
	pub user_id: Snowflake,
}

/// Represents the current state of the link.
#[derive(Serialize, Deserialize)]
pub enum LinkStatus {
	/// The Family Center link request has been sent, but not accepted
	SENT = 1,
	/// The linked user is currently connected to the requestor
	CONNECTED = 2,
	/// The link has been disconnected
	DISCONNECTED = 3,
	/// The link request was rejected
	REJECTED = 4,
}

/// Represents what part each user played in the connection.
#[derive(Serialize, Deserialize)]
pub enum LinkType {
	/// The current user accepted the request and is the linked user of the link
	RECEIVER = 1,
	/// The current user sent the request and is the requestor of the link
	SENDER = 2,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedUsers {
	/// List of linked users
	pub linked_users: Vec<LinkedUser>,
	/// List of requestors the linked user is connected to
	pub users: Vec<PartialUser>,
}

#[derive(Serialize, Deserialize)]
pub struct TeenAuditLog {
	/// The ID of the linked user
	pub teen_user_id: Option<Snowflake>,
	/// A snowflake representing the start time of the current 7-day track range
	pub range_start_id: Option<Snowflake>,
	/// List of actions the linked user has done
	pub actions: Vec<Action>,
	/// Users referenced in the audit log
	pub users: Vec<PartialUser>,
	/// Guilds referenced in the audit log
	pub guilds: Vec<Guild>,
	/// Object keyed by action types with their totals
	pub totals: HashMap<i64, ActionType>,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
	/// The ID of the event action
	pub event_id: Snowflake,
	/// The ID of the linked user
	pub user_id: Snowflake,
	/// The ID of the entity the action relates to (user, guild, or group DM) based off the action type
	pub entity_id: Snowflake,
	/// The type of the action of the action, detailing what this action involved
	pub display_type: i64,
}

/// Represents what kind of action the linked user engaged in.
#[derive(Serialize, Deserialize)]
pub enum ActionType {
	/// Users added within the last 7 days
	USERS_ADDED = 1,
	/// Guild joined within the last 7 days
	GUILDS_JOINED = 2,
	/// Users messaged within the last 7 days
	USERS_MESSAGED = 3,
	/// Guilds the linked user has sent messages in within the last 7 days
	GUILDS_MESSAGED = 4,
	/// Users called within the last 7 days
	USERS_CALLED = 5,
}


