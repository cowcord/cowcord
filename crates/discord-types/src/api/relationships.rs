use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::{connected_accounts::ConnectionType, users::PartialUser}, common::{id::{ApplicationId, UserId}, timestamp::Timestamp}};

#[derive(Serialize, Deserialize)]
pub struct Relationship {
	/// The ID of the target user
	pub id: String,
	/// The type of relationship
	pub r#type: RelationshipType,
	/// The target user
	pub user: PartialUser,
	/// The nickname of the user in this relationship (1-32 characters)
	pub nickname: Option<String>,
	/// Whether the friend request was flagged as spam (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_spam_request: Option<bool>,
	/// Whether the friend request was sent by a user without a mutual friend or small mutual guild (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stranger_request: Option<bool>,
	/// Whether the target user has been ignored by the current user
	pub user_ignored: bool,
	/// The ID of the application that created the relationship
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_application_id: Option<Option<ApplicationId>>,
	/// When the user requested a relationship
	#[serde(skip_serializing_if = "Option::is_none")]
	pub since: Option<Timestamp>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RelationshipType {
	/// No relationship exists
	NONE = 0,
	/// The user is a friend
	FRIEND = 1,
	/// The user is blocked
	BLOCKED = 2,
	/// The user has sent a friend request to the current user
	INCOMING_REQUEST = 3,
	/// The current user has sent a friend request to the user
	OUTGOING_REQUEST = 4,
	/// The user is an affinity of the current user
	IMPLICIT = 5,
}

#[derive(Serialize, Deserialize)]
pub struct GameRelationship {
	/// The ID of the target user
	pub id: String,
	/// The ID of the application whose game the relationship originated from
	pub application_id: ApplicationId,
	/// The type of relationship
	pub r#type: RelationshipType,
	/// The target user
	pub user: PartialUser,
	/// When the user requested a relationship
	pub since: Timestamp,
	/// The DM access level for the relationship
	pub dm_access_type: Value,
	/// The ID of the current user
	pub user_id: UserId,
}

#[derive(Serialize, Deserialize)]
pub struct FriendSuggestion {
	/// The suggested user
	pub suggested_user: PartialUser,
	/// The sources of the suggestion
	pub reasons: Vec<FriendSuggestionReason>,
	/// Whether the suggested user has the current user in their contacts
	#[serde(skip_serializing_if = "Option::is_none")]
	pub from_suggested_user_contacts: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FriendSuggestionReason {
	/// The type of reason
	pub r#type: FriendSuggestionReasonType,
	/// The platform that the suggestion originated from
	pub platform: ConnectionType,
	/// The user's name on the platform
	pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FriendSuggestionReasonType {
	/// The user is a friend on another platform
	EXTERNAL_FRIEND = 1,
}

