use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::api_types::{guild::PartialGuild, guild_scheduled_event::GuildScheduledEvent};

#[derive(Serialize, Deserialize)]
pub struct DirectoryEntry {
	/// The type of directory entry
	pub r#type: DirectoryEntryType,
	/// The ID of the directory channel that the entry is in
	pub directory_channel_id: Snowflake,
	/// The ID of the guild or scheduled event
	pub entity_id: Snowflake,
	/// When the entry was createdw
	pub created_at: String,
	/// The primary category of the entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_category_id: Option<i64>,
	/// The description of the entry
	pub description: Option<String>,
	/// The ID of the user that created the entry
	pub author_id: Snowflake,
	/// The guild entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild: Option<Box<DirectoryGuild>>,
	/// The guild scheduled event entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_scheduled_event: Option<DirectoryGuildScheduledEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryGuild {
    #[serde(flatten)]
    pub entry: Box<DirectoryEntry>,
	/// Whether the guild is eligible to be featured in the directory
	pub featurable_in_directory: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryGuildScheduledEvent {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
	/// The guild that the event is for
	pub guild: PartialGuild,
	/// Whether the user has RSVP'd to the event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_rsvp: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DirectoryEntryType {
	/// A guild
	GUILD = 0,
	/// A scheduled event
	GUILD_SCHEDULED_EVENT = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DirectoryCategory {
	/// Uncategorized entry
	UNCATEGORIZED = 0,
	/// School club or organization
	SCHOOL_CLUB = 1,
	/// Class or subject
	CLASS = 2,
	/// Study or social group
	STUDY_SOCIAL = 3,
	/// Miscellaneous entry
	MISC = 5,
}

