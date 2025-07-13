use serde::{Deserialize, Serialize};

use crate::api::guild::PrivacyLevel;

#[derive(Serialize, Deserialize)]
pub struct StageInstance {
	/// The ID of the stage instance
	pub id: Snowflake,
	/// The guild ID of the associated stage channel
	pub guild_id: Snowflake,
	/// The ID of the associated stage channel
	pub channel_id: Snowflake,
	/// The topic of the stage instance (1-120 characters)
	pub topic: String,
	/// The privacy level of the stage instance
	pub privacy_level: PrivacyLevel,
	/// The invite code that can be used to join the stage channel, if the stage instance is public
	pub invite_code: Option<String>,
	/// Whether or not stage discovery is disabled
	#[deprecated]
	pub discoverable_disabled: bool,
	/// The ID of the scheduled event for this stage instance
	pub guild_scheduled_event_id: Option<Snowflake>,
}
