use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::connected_accounts::VisibilityType;
use crate::api::emoji::Emoji;
use crate::api::guild::{GuildFeatures, PremiumTier};
use crate::api::stickers::Sticker;
use crate::common::id::{ApplicationId, ChannelId, DiscoveryCategoryId, EmojiId, GuildId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct DiscoverableGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The description for the guild (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// The guild's banner hash
	pub banner: Option<String>,
	/// The guild's splash hash
	pub splash: Option<String>,
	/// The guild's discovery splash hash
	pub discovery_splash: Option<String>,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// The guild's vanity invite code
	pub vanity_url_code: Option<String>,
	/// The preferred locale of the guild; used in discovery and notices from Discord (default "en-US")
	pub preferred_locale: String,
	/// The number of premium subscriptions (boosts) the guild currently has
	pub premium_subscription_count: u64,
	/// Approximate number of members in the guild
	pub approximate_member_count: u64,
	/// Approximate number of online members in the guild
	pub approximate_presence_count: u64,
	/// Custom guild emojis; limited to 30 entries
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emojis: Option<Vec<Emoji>>,
	/// Total number of custom guild emojis
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_count: Option<u16>,
	/// Custom guild stickers; limited to 30 entries
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stickers: Option<Vec<Sticker>>,
	/// Total number of custom guild stickers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticker_count: Option<u16>,
	/// Whether the guild has automatically been removed from discovery for not hitting required targets
	pub auto_removed: bool,
	/// The ID of the primary discovery category set for the guild
	pub primary_category_id: DiscoveryCategoryId,
	/// The primary discovery category set for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_category: Option<DiscoveryCategory>,
	/// The discovery search keywords for the guild (max 30 characters, max 10)
	pub keywords: Option<ArrayVec<ArrayString<30>, 10>>,
	/// Whether the guild's landing web page is currently published
	pub is_published: bool,
	/// The reasons to join the guild shown in the discovery web page (max 4)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasons_to_join: Option<Vec<DiscoveryReason>>,
	/// The guild's social media links shown in the discovery web page (max 256 characters, max 9)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub social_links: Option<Option<Vec<String>>>,
	/// The guild's long description shown in the discovery web page (max 2400 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub about: Option<Option<String>>,
	/// The IDs of discovery subcategories set for the guild (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_ids: Option<Vec<DiscoveryCategoryId>>,
	/// The discovery categories set for the guild (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub categories: Option<Vec<DiscoveryCategory>>,
	/// When the guild was created
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryRequirements {
	/// The ID of the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// Whether the guild has not been flagged by Trust & Safety
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safe_environment: Option<bool>,
	/// Whether the guild meets activity requirements
	#[serde(skip_serializing_if = "Option::is_none")]
	pub healthy: Option<bool>,
	/// Whether the guild's activity metrics have not yet been calculated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub health_score_pending: Option<bool>,
	/// Whether the guild meets the minimum member count requirement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<bool>,
	/// Disallowed terms found in the guild's name, description, and channel names
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw_properties: Option<DiscoveryNsfwProperties>,
	/// Whether the guild has the MFA requirement for moderation actions enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub protected: Option<bool>,
	/// Whether the guild meets the requirements to be in Discovery
	pub sufficient: bool,
	/// Whether the grace period can allow the guild to remain in Discovery
	pub sufficient_without_grace_period: bool,
	/// Whether the guild has a rules channel set
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_rules_channel: Option<bool>,
	/// Whether the guild meets the new member retention requirement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retention_healthy: Option<bool>,
	/// Whether the guild meets the weekly visitor and communicator requirements
	#[serde(skip_serializing_if = "Option::is_none")]
	pub engagement_healthy: Option<bool>,
	/// Whether the guild meets the minimum age requirement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub age: Option<bool>,
	/// The minimum guild age requirement (in days)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_age: Option<Option<u16>>,
	/// The guild's activity metrics
	#[serde(skip_serializing_if = "Option::is_none")]
	pub health_score: Option<DiscoveryHealthScore>,
	/// The minimum guild member count requirement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_size: Option<Option<u16>>,
	/// When the guild's grace period ends
	#[serde(skip_serializing_if = "Option::is_none")]
	pub grace_period_end_date: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryNsfwProperties {
	/// The IDs of the channels with names containing disallowed terms
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channels: Option<Vec<ChannelId>>,
	/// The disallowed terms found in the given channel names
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_banned_keywords: Option<HashMap<ChannelId, Vec<String>>>,
	/// The guild name, if it contains disallowed terms
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The disallowed terms found in the guild name
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name_banned_keywords: Option<Vec<String>>,
	/// The guild description, if it contains disallowed terms
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The disallowed terms found in the guild description
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description_banned_keywords: Option<Vec<String>>,
}

/// Activity metrics are recalculated weekly, as an 8-week rolling average. If they are not yet eligible to be calculated, all fields will be null.
#[derive(Serialize, Deserialize)]
pub struct DiscoveryHealthScore {
	/// Average weekly number of users who talk in the guild and have been on Discord for 8 weeks+
	pub avg_nonnew_communicators: Option<String>,
	/// Average weekly number of users who view the guild and have been on Discord for 8 weeks+
	pub avg_nonnew_participators: Option<String>,
	/// Average number of users who join the guild per week
	pub num_intentful_joiners: Option<String>,
	/// Percentage of new members who remain in the guild for at least a week
	pub perc_ret_w1_intentful: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryMetadata {
	/// The ID of the guild
	pub guild_id: GuildId,
	/// The ID of the primary discovery category set for the guild
	pub primary_category_id: DiscoveryCategoryId,
	/// The discovery search keywords for the guild (max 30 characters, max 10)
	pub keywords: Option<ArrayVec<ArrayString<30>, 10>>,
	/// Whether the guild is shown as a source through custom guild expressions
	pub emoji_discoverability_enabled: bool,
	/// When the guild's partner application was actioned by an employee
	pub partner_actioned_timestamp: Option<Timestamp>,
	/// When the guild applied for partnership
	pub partner_application_timestamp: Option<Timestamp>,
	/// Whether the guild's landing web page is currently published
	pub is_published: bool,
	/// The reasons to join the guild shown in the discovery web page (max 4)
	pub reasons_to_join: ArrayVec<DiscoveryReason, 4>,
	/// The guild's social media links shown in the discovery web page (max 256 characters, max 9)
	pub social_links: Option<ArrayVec<ArrayString<256>, 9>>,
	/// The guild's long description shown in the discovery web page (max 2400 characters)
	pub about: Option<ArrayString<2400>>,
	/// The IDs of discovery subcategories set for the guild (max 5)
	pub category_ids: ArrayVec<DiscoveryCategoryId, 5>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryReason {
	/// The reason to join the guild
	pub reason: String,
	/// The ID of a guild's custom emoji
	pub emoji_id: Option<EmojiId>,
	/// The unicode character of the emoji
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryCategory {
	/// The ID of the category
	pub id: DiscoveryCategoryId,
	/// The name of the category
	pub name: String,
	/// Whether the category can be used as a guild's primary category
	pub is_primary: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GuildProfile {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon_hash: Option<String>,
	/// Approximate count of total members in the guild
	pub member_count: u32,
	/// Approximate count of non-offline members in the guild
	pub online_count: u32,
	/// The description for the guild (max 300 characters)
	pub description: ArrayString<300>,
	/// The guild's accent color as a hexadecimal color string
	pub brand_color_primary: String,
	/// The guild's clan banner hash
	#[deprecated]
	pub banner_hash: Option<String>,
	/// The IDs of the applications representing the games the guild plays (max 20)
	pub game_application_ids: ArrayVec<ApplicationId, 20>,
	/// The activity of the guild in each game
	pub game_activity: HashMap<ApplicationId, GameActivity>,
	/// The tag of the guild (2-4 characters)
	pub tag: Option<ArrayString<4>>,
	/// The badge shown on the guild's tag
	pub badge: GuildBadgeType,
	/// The primary color of the badge as a hexadecimal color string
	pub badge_color_primary: String,
	/// The secondary color of the badge as a hexadecimal color string
	pub badge_color_secondary: String,
	/// The guild tag badge hash
	pub badge_hash: String,
	/// Terms used to describe the guild's interest and personality (max 5)
	pub traits: ArrayVec<GuildTrait, 5>,
	/// Enabled guild features
	pub features: Vec<String>,
	/// The visibility level of the guild
	pub visibility: VisibilityType,
	/// The guild's discovery splash hash
	pub custom_banner_hash: Option<String>,
	/// The number of premium subscriptions (boosts) the guild currently has
	pub premium_subscription_count: u32,
	/// The guild's premium tier (boost level)
	pub premium_tier: PremiumTier,
}

#[derive(Serialize, Deserialize)]
pub struct GameActivity {
	/// The activity level of the guild in the game
	pub activity_level: i64,
	/// The activity score of the guild in the game
	pub activity_score: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GuildTrait {
	/// ID of the emoji associated with the trait
	pub emoji_id: Option<EmojiId>,
	/// Name of the emoji associated with the trait
	pub emoji_name: Option<String>,
	/// Whether the associated emoji is animated
	pub emoji_animated: bool,
	/// Name of the trait
	pub label: String,
	/// Position of the trait in the array for sorting
	pub position: i64,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildBadgeType {
	SWORD = 0,
	WATER_DROP = 1,
	SKULL = 2,
	TOADSTOOL = 3,
	MOON = 4,
	LIGHTNING = 5,
	LEAF = 6,
	HEART = 7,
	FIRE = 8,
	COMPASS = 9,
	CROSSHAIRS = 10,
	FLOWER = 11,
	FORCE = 12,
	GEM = 13,
	LAVA = 14,
	PSYCHIC = 15,
	SMOKE = 16,
	SNOW = 17,
	SOUND = 18,
	SUN = 19,
	WIND = 20,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildVisibility {
	/// This guild is considered public and can be viewed by anyone
	PUBLIC = 1,
	/// This guild is considered private but cannot be viewed and joining requires an invite
	RESTRICTED = 2,
	/// The guild is considered public, allowing anyone to view it and submit a join request
	PUBLIC_WITH_RECRUITMENT = 3,
}
