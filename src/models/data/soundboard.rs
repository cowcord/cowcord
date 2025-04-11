use serde::Deserialize;

use super::types::Snowflake;
use super::user::User;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SoundboardSound {
	pub sound_id:   Snowflake,
	pub name:       String,
	pub volume:     f64,
	pub emoji_id:   Option<Snowflake>,
	pub emoji_name: Option<String>,
	pub guild_id:   Snowflake,
	pub available:  bool,
	pub user:       User,
	pub user_id:    Snowflake,
}
