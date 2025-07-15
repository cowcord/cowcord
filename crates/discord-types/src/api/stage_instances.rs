use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::guild::PrivacyLevel;
use crate::common::id::{ChannelId, GuildId, ScheduledEventId, StageInstanceId};

#[derive(Serialize, Deserialize)]
pub struct StageInstance {
	/// The ID of the stage instance
	pub id: StageInstanceId,
	/// The guild ID of the associated stage channel
	pub guild_id: GuildId,
	/// The ID of the associated stage channel
	pub channel_id: ChannelId,
	/// The topic of the stage instance (1-120 characters)
	pub topic: ArrayString<120>,
	/// The privacy level of the stage instance
	pub privacy_level: PrivacyLevel,
	/// The invite code that can be used to join the stage channel, if the stage instance is public
	pub invite_code: Option<String>,
	/// Whether or not stage discovery is disabled
	#[deprecated]
	pub discoverable_disabled: bool,
	/// The ID of the scheduled event for this stage instance
	pub guild_scheduled_event_id: Option<ScheduledEventId>,
}
