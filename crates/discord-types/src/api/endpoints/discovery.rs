#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::api::discovery::{
	DiscoverableGuild,
	DiscoveryMetadata,
	DiscoveryReason,
	DiscoveryRequirements,
	MonetizationStorePage,
};
use crate::api::types::Snowflake;
use crate::utils::request::to_string_query;

/// Type: get
pub fn GET_DISCOVERABLE_GUILDS_ENDPOINT(query: GetDiscoverableGuildsRequest) -> String {
	format!("/discoverable-guilds{}", to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetDiscoverableGuildsRequest {
	pub guild_ids:       Vec<Snowflake>,
	pub application_ids: Vec<Snowflake>,
	pub categories:      Vec<u32>,
	pub limit:           u8,
	pub offset:          u16,
}

#[derive(Deserialize)]
pub struct GetDisoverableGuildsResponse {
	pub guilds: Vec<DiscoverableGuild>,
	pub total:  u16,
	pub limit:  u8,
	pub offset: u16,
}

/// Type: get
pub fn SEARCH_DISCOVERABLE_GUILDS_ENDPOINT(query: SearchDiscoverableGuildsRequest) -> String {
	format!("/discoverable-guilds/search{}", to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct SearchDiscoverableGuildsRequest {
	pub query:       String,
	pub limit:       u8,
	pub offset:      u16,
	pub category_id: u32,
}

/// Type: get
///
/// doesnt require Authentication header
pub fn SEARCH_PUBLISHED_GUILDS_ENDPOINT(query: SearchPublishedGuildsRequest) -> String {
	format!("/discovery/search{}", to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct SearchPublishedGuildsRequest {
	pub query:  String,
	pub limit:  u8,
	pub offset: u16,
}

/// Type: get
///
/// doesnt require Authentication header
pub fn GET_DISCOVERY_SLUG_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/discovery/{}", guild_id)
}

#[derive(Deserialize)]
pub struct GetDiscoverySlugResponse {
	/// can be appended to https://discord.com/servers/ to get the guild's discovery page
	pub slug:       String,
	pub guild:      DiscoverableGuild,
	pub store_page: MonetizationStorePage,
}

/// Type: get
pub fn GET_DISCOVERY_CATEGORIES_ENDPOINT(query: GetDiscoveryCategoriesRequest) -> String {
	format!("/discovery/categories{}", to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetDiscoveryCategoriesRequest {
	pub locale:       String,
	pub primary_only: bool,
}

/// Type: get
pub fn VALIDATE_DISCOVERY_SEARCH_TERM_ENDPOINT(
	query: ValidateDiscoverySearchTermRequest
) -> String {
	format!("/discovery/valid-term{}", to_string_query(&query))
}

#[derive(Serialize)]
pub struct ValidateDiscoverySearchTermRequest {
	pub term: String,
}

#[derive(Deserialize)]
pub struct ValidateDiscoverySearchTermResponse {
	pub valid: bool,
}

/// Type: get
///
/// requires MANAGE_GUILD permission
pub fn GET_GUILD_DISCOVERY_REQUIREMENTS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/discovery-requirements", guild_id)
}

pub type GetGuildDiscoveryRequirementsResponse = DiscoveryRequirements;

/// Type: get
///
/// requires MANAGE_GUILD permission
pub fn GET_GUILD_DISCOVERY_METADATA_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/discovery-metadata", guild_id)
}

pub type GetGuildDiscoveryMetadataResponse = DiscoveryMetadata;

/// Type: patch
///
/// requires MANAGE_GUILD permission
pub fn MODIFY_GUILD_DISCOVERY_METADATA_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/discovery-metadata", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyGuildDiscoveryMetadataRequest {
	pub primary_category_id:           Option<u16>,
	pub keywords:                      Option<Vec<String>>,
	pub emoji_discoverability_enabled: Option<bool>,
	pub is_published:                  Option<bool>,
	pub reasons_to_join:               Option<Vec<DiscoveryReason>>,
	pub social_links:                  Option<Vec<String>>,
	pub about:                         Option<String>,
}

pub type ModifyGuildDiscoveryMetadataResponse = DiscoveryMetadata;

/// Type: put
///
/// requires MANAGE_GUILD permission
pub fn ADD_GUILD_DISCOVERY_CATEGORY_ENDPOINT(
	guild_id: Snowflake,
	category_id: Snowflake,
) -> String {
	format!("/guilds/{}/discovery-categories/{}", guild_id, category_id)
}

/// Type: delete
///
/// requires MANAGE_GUILD permission
pub fn REMOVE_GUILD_DISCOVERY_CATEGORY_ENDPOINT(
	guild_id: Snowflake,
	category_id: Snowflake,
) -> String {
	format!("/guilds/{}/discovery-categories/{}", guild_id, category_id)
}
