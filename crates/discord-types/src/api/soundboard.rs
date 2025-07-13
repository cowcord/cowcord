use serde::{Deserialize, Serialize};

use crate::api::users::PartialUser;

#[derive(Serialize, Deserialize)]
pub struct SoundboardSound {
	/// The ID of the soundboard sound
	pub sound_id: Snowflake,
	/// The name of the soundboard sound (2-32 characters)
	pub name: String,
	/// The volume of the soundboard sound (represented as a float from 0 to 1)
	pub volume: f64,
	/// The ID of the sound's custom emoji
	pub emoji_id: Option<Snowflake>,
	/// The unicode character of the sound's emoji
	pub emoji_name: Option<String>,
	/// The ID of the source guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// Whether this guild sound can be used; may be false due to loss of premium subscriptions (boosts)
	pub available: bool,
	/// The user who created this sound
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// The ID of the user who created this sound
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<Snowflake>,
}
