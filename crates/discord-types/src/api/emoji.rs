use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::guild::{GuildFeatures, PremiumTier};
use crate::api::users::PartialUser;
use crate::common::id::{ApplicationId, EmojiId, GuildId, RoleId};
use crate::common::image::ImageHash;

#[derive(Serialize, Deserialize)]
pub struct Emoji {
	/// The ID of the emoji
	pub id: Option<EmojiId>,
	/// The name of the emoji (2-32 characters)
	pub name: ArrayString<32>,
	/// The roles allowed to use the emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<RoleId>>,
	/// The user that uploaded the emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// Whether this emoji must be wrapped in colons
	#[serde(skip_serializing_if = "Option::is_none")]
	pub require_colons: Option<bool>,
	/// Whether this emoji is managed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub managed: Option<bool>,
	/// Whether this emoji is animated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub animated: Option<bool>,
	/// Whether this emoji can be used; may be false due to loss of premium subscriptions (boosts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TopEmoji {
	/// The ID of the emoji
	pub emoji_id: EmojiId,
	/// The overall rank of the emoji
	pub emoji_rank: u8,
}

#[derive(Serialize, Deserialize)]
pub enum EmojiSourceType {
	/// The emoji is uploaded to a guild
	GUILD,
	/// The emoji is uploaded to an application
	APPLICATION,
}

#[derive(Serialize, Deserialize)]
pub struct EmojiGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild
	/// (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<ImageHash>,
	/// The description for the guild
	/// (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// Custom guild emojis
	pub emojis: Vec<Emoji>,
	/// The guild's premium tier (boost level)
	pub premium_tier: PremiumTier,
	/// The number of premium subscriptions (boosts) the guild currently has
	pub premium_subscription_count: u32,
	/// Approximate number of total members in the guild
	pub approximate_member_count: u32,
	/// Approximate number of non-offline members in the guild
	pub approximate_presence_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EmojiApplication {
	/// The ID of the application
	pub id: ApplicationId,
	/// The name of the application
	pub name: String,
}
