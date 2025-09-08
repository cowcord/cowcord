use arrayvec::{ArrayString, ArrayVec};
use hex::Hex;
use serde::{Deserialize, Serialize};

use crate::api::discovery::{
	DiscoverableGuild,
	DiscoveryCategory,
	DiscoveryMetadata,
	DiscoveryReason,
	DiscoveryRequirements,
	GuildBadgeType,
	GuildProfile,
	GuildTrait,
	GuildVisibility,
	MonetizationStorePage,
};
use crate::common::id::{ApplicationId, DiscoveryCategoryId, GuildId};
use crate::common::image::ImageHash;
use crate::common::locale::Locale;
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// When this endpoint is paginated normally, only discoverable guilds that are verified or partnered are returned.
///
/// To get other discoverable guilds, you must either use the guild_ids query parameter or [search discovery](https://docs.discord.food/resources/discovery#searching-discovery).
///
/// Returns a list of [discoverable guild](https://docs.discord.food/resources/discovery#discoverable-guild-object) objects representing the guilds that are available for the current user to discover.
pub fn GET_DISCOVERABLE_GUILDS(query: &GetDiscoverableGuildsQueryParams) -> String {
	format!("/discoverable-guilds{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetDiscoverableGuildsQueryParams {
	/// The IDs of the discoverable guilds to return (max 48)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_ids: Option<Vec<GuildId>>,
	/// The IDs of the applications to return matching discoverable guilds for (max 48)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_ids: Option<Vec<ApplicationId>>,
	/// The IDs of the discovery categories to filter results by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub categories: Option<Vec<u32>>,
	/// The maximum number of guilds to return
	/// (max 48, default 48)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Number of guilds to skip before returning guilds
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDiscoverableGuildsResponse {
	/// The guilds that match the query
	pub guilds: Vec<DiscoverableGuild>,
	/// The total number of guilds that match the query
	pub total: u32,
	/// The number of guilds returned in the response
	pub limit: u16,
	/// The number of guilds skipped before returning guilds
	pub offset: u16,
}

/// Method: `GET`
///
/// This endpoint has the following immutable filters set:
///
/// `approximate_member_count > 200`
///
/// `approximate_presence_count > 0`
///
/// `auto_removed: false`
///
/// Returns a list of [discoverable guild](https://docs.discord.food/resources/discovery#discoverable-guild-object) objects that match the query.
pub fn SEARCH_DISCOVERABLE_GUILDS(query: &SearchDiscoverableGuildsQueryParams) -> String {
	format!("/discoverable-guilds/search{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct SearchDiscoverableGuildsQueryParams {
	/// The query to match (max 100 characters)
	pub query: ArrayString<100>,
	/// The maximum number of guilds to return
	/// (max 48, default 24)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Number of guilds to skip before returning guilds (max 2999)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u16>,
	/// The ID of the discovery category to filter results by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_id: Option<u32>,
}

pub type SearchDiscoverableGuildsResponse = Vec<DiscoverableGuild>;

/// Method: `GET`
///
/// Does not require authentication
///
/// This endpoint has the following immutable filters set:
///
/// `approximate_member_count > 200`
///
/// `approximate_presence_count > 0`
///
/// `auto_removed: false`
///
/// `is_published: true`
///
/// Returns a list of [discoverable guild](https://docs.discord.food/resources/discovery#discoverable-guild-object) objects that have a landing web page and match the query. This endpoint is a proxy for [searching using the Algolia API](https://docs.discord.food/resources/discovery#searching-discovery). See the [Algolia Search documentation](https://www.algolia.com/doc/rest-api/search/) for more information.
pub fn SEARCH_PUBLISHED_GUILDS(query: &SearchPublishedGuildsQueryParams) -> String {
	format!("/discovery/search{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct SearchPublishedGuildsQueryParams {
	/// The query to match
	#[serde(skip_serializing_if = "Option::is_none")]
	pub query: Option<String>,
	/// The maximum number of guilds to return
	/// (1-48, default 48)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Number of guilds to skip before returning guilds (max 2999)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u16>,
}

pub type SearchPublishedGuildsResponse = Vec<DiscoverableGuild>;

/// Method: `GET`
///
/// Does not require authentication
///
/// If a guild has both a landing web page and a monetization store page, the store page is prioritized.
///
/// This endpoint requires the guild to either be discoverable and [published](https://docs.discord.food/resources/discovery#discovery-metadata-object) or have the [`CREATOR_STORE_PAGE` guild feature](https://docs.discord.food/resources/guild#guild-features).
///
/// Returns information about a guild's landing web page or monetization store page.
pub fn GET_DISCOVERY_SLUG(guild_id: &GuildId) -> String {
	format!("/discovery/{}", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetDiscoverySlugResponse {
	/// The guild's discovery slug; can be appended to https://discord.com/servers/ to get the guild's discovery page
	pub slug: String,
	/// The guild information, if the guild is discoverable
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild: Option<DiscoverableGuild>,
	/// The guild's monetization store page, if enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store_page: Option<MonetizationStorePage>,
}

/// Method: `GET`
///
/// Returns a list of [discovery category](https://docs.discord.food/resources/discovery#discovery-category-object) objects representing the available discovery categories.
pub fn GET_DISCOVERY_CATEGORIES(query: &GetDiscoveryCategoriesQueryParams) -> String {
	format!("/discovery/categories{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetDiscoveryCategoriesQueryParams {
	/// The language to return category names in
	/// (default "en-US")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
	/// Whether to only return categories that can be set as a guild's primary category
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_only: Option<bool>,
}

pub type GetDiscoveryCategoriesResponse = Vec<DiscoveryCategory>;

/// Method: `GET`
///
/// Checks if a discovery search term is allowed.
pub fn VALIDATE_DISCOVERY_SEARCH_TERM(query: &ValidateDiscoverySearchTermQueryParams) -> String {
	format!("/discovery/valid-term{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct ValidateDiscoverySearchTermQueryParams {
	/// The search term to validate
	pub term: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateDiscoverySearchTermResponse {
	/// Whether the provided term is valid
	pub valid: bool,
}

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns the [discovery requirements](https://docs.discord.food/resources/discovery#discovery-requirements-object) object for the guild.
pub fn GET_GUILD_DISCOVERY_REQUIREMENTS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/discovery-requirements", guild_id)
}

pub type GetGuildDiscoveryRequirementsResponse = DiscoveryRequirements;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns the [discovery metadata](https://docs.discord.food/resources/discovery#discovery-metadata-object) object for the guild.
pub fn GET_GUILD_DISCOVERY_METADATA(guild_id: &GuildId) -> String {
	format!("/guilds/{}/discovery-metadata", guild_id)
}

pub type GetGuildDiscoveryMetadataResponse = DiscoveryMetadata;

/// Method: `PATCH`
///
/// Replaces the discovery metadata for the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// All parameters to this endpoint are optional and nullable.
/// Omitting or setting a null value will set it to default.
///
/// Returns the updated [discovery metadata](https://docs.discord.food/resources/discovery#discovery-metadata-object) object on success.
pub fn MODIFY_GUILD_DISCOVERY_METADATA(guild_id: &GuildId) -> String {
	format!("/guilds/{}/discovery-metadata", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildDiscoveryMetadataRequest {
	/// The ID of the primary discovery category set for the guild
	/// (default 0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_category_id: Option<Option<u32>>,
	/// The discovery search keywords for the guild
	/// (max 10)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keywords: Option<Option<ArrayVec<String, 10>>>,
	/// Whether the guild is shown as a source through custom emojis and stickers
	/// (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_discoverability_enabled: Option<Option<bool>>,
	/// Whether the guild's landing web page is currently published
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_published: Option<Option<bool>>,
	/// The reasons to join the guild shown in the discovery web page
	/// (max 4)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasons_to_join: Option<Option<ArrayVec<DiscoveryReason, 4>>>,
	/// The guild's social media links shown in the discovery web page
	/// (max 256 characters, max 9)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub social_links: Option<Option<ArrayVec<ArrayString<256>, 9>>>,
	/// The guild's long description shown in the discovery web page
	/// (max 2400 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub about: Option<Option<ArrayString<2400>>>,
}

/// Method: `PUT`
///
/// Adds a [discovery subcategory](https://docs.discord.food/resources/discovery#discovery-category-object) to the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a `204` empty response on success.
pub fn ADD_GUILD_DISCOVERY_SUBCATEGORY(
	guild_id: &GuildId,
	category_id: &DiscoveryCategoryId,
) -> String {
	format!("/guilds/{}/discovery-categories/{}", guild_id, category_id)
}

/// Method: `DELETE`
///
/// Removes a [discovery subcategory](https://docs.discord.food/resources/discovery#discovery-category-object) from the guild.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a `204` empty response on success.
pub fn REMOVE_GUILD_DISCOVERY_SUBCATEGORY(
	guild_id: &GuildId,
	category_id: &DiscoveryCategoryId,
) -> String {
	format!("/guilds/{}/discovery-categories/{}", guild_id, category_id)
}

/// Method: `GET`
///
/// User must be a member of the guild or the guild must be discoverable or have a [`PUBLIC` or `PUBLIC_WITH_RECRUITMENT` visibility](https://docs.discord.food/resources/discovery#guild-visibility).
///
/// Returns a [guild profile](https://docs.discord.food/resources/discovery#guild-profile-object) object for the given guild ID.
pub fn GET_GUILD_PROFILE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/profile", guild_id)
}

pub type GetGuildProfileResponse = GuildProfile;

/// Method: `PATCH`
///
/// Modifies the [guild profile](https://docs.discord.food/resources/discovery#guild-profile-object) for the given guild ID.
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns the updated [guild profile](https://docs.discord.food/resources/discovery#guild-profile-object) object on success.
pub fn MODIFY_GUILD_PROFILE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/profile", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct JsonParams {
	/// The name of the guild
	/// (2-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<100>>,
	/// The guild's icon
	///
	/// animated icons are only shown when the guild has the `ANIMATED_ICON` feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The description for the guild
	/// (max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<300>>>,
	/// The guild's accent color as a hexadecimal color string
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "hex::as_str")]
	pub brand_color_primary: Option<Hex>,
	/// The IDs of the applications representing the games the guild plays
	/// (max 20)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game_application_ids: Option<ArrayVec<ApplicationId, 20>>,
	/// The tag of the guild
	/// (2-4 characters)
	///
	/// Requires the `GUILD_TAGS` guild feature.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag: Option<Option<ArrayString<4>>>,
	/// The badge shown on the guild's tag
	///
	/// Requires the `GUILD_TAGS` guild feature.
	pub badge: GuildBadgeType,
	/// The primary color of the badge as a hexadecimal color string
	///
	/// Requires the `GUILD_TAGS` guild feature.
	#[serde(with = "hex::as_str")]
	pub badge_color_primary: Option<Hex>,
	/// The secondary color of the badge as a hexadecimal color string
	///
	/// Requires the `GUILD_TAGS` guild feature.
	#[serde(with = "hex::as_str")]
	pub badge_color_secondary: Option<Hex>,
	/// Terms used to describe the guild's interest and personality
	/// (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traits: Option<ArrayVec<GuildTrait, 5>>,
	/// The visibility level of the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub visibility: Option<GuildVisibility>,
	/// The guild's discovery splash
	pub custom_banner: Option<ImageHash>,
}
