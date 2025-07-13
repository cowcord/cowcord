#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::api::directory::DirectoryEntry;
use crate::api::types::Snowflake;
use crate::utils::request::to_string_query;

/// Type: get
/// requires VIEW_CHANNEL permission
pub fn GET_DIRECTORY_COUNTS_ENDPOINT(channel_id: Snowflake) -> String {
	format!("/channels/{}/directory-entries/counts", channel_id)
}

// returns a mapping of https://docs.discord.food/resources/directory#directory-category ill do that later

/// Type: get
/// requires VIEW_CHANNEL permission
pub fn GET_DIRECTORY_ENTRIES_ENDPOINT(
	channel_id: Snowflake,
	query: GetDirectoryEntriesRequest,
) -> String {
	format!(
		"/channels/{}/directory-entries{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetDirectoryEntriesRequest {
	/// https://docs.discord.food/resources/directory#directory-entry-type
	pub r#type: u8,
	/// https://docs.discord.food/resources/directory#directory-category
	pub category_id: u8,
}

pub type GetDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Type: get
/// requires VIEW_CHANNEL permission
pub fn GET_PARTIAL_DIRECTORY_ENTRIES_ENDPOINT(
	channel_id: Snowflake,
	query: GetPartialDirectoryEntriesRequest,
) -> String {
	format!(
		"/channels/{}/directory-entries/list{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetPartialDirectoryEntriesRequest {
	pub entity_ids: Vec<Snowflake>,
}

pub type GetPartialDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Type: get
/// requires VIEW_CHANNEL permission
pub fn SEARCH_DIRECTORY_ENTRIES_ENDPOINT(
	channel_id: Snowflake,
	query: SearchDirectoryEntriesRequest,
) -> String {
	format!(
		"/channels/{}/directory-entries/search{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct SearchDirectoryEntriesRequest {
	pub query: String,
	/// https://docs.discord.food/resources/directory#directory-entry-type
	pub r#type: u8,
	/// https://docs.discord.food/resources/directory#directory-category
	pub category_id: u8,
}

pub type SearchDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Type: get
/// requires VIEW_CHANNEL permission
pub fn GET_DIRECTORY_ENTRY_ENDPOINT(
	channel_id: Snowflake,
	entity_id: Snowflake,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

pub type GetDirectoryEntryResponse = DirectoryEntry;

/// Type: post
/// requires VIEW_CHANNEL and MANAGE_GUILD permission
pub fn CREATE_DIRECTORY_ENTRY_ENDPOINT(
	channel_id: Snowflake,
	entity_id: Snowflake,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateDirectoryEntryRequest {
	/// https://docs.discord.food/resources/directory#directory-entry-type
	pub r#type: u8,
	/// https://docs.discord.food/resources/directory#directory-category
	pub primary_category_id: u8,
	pub description: Option<String>,
}

pub type CreateDirectoryEntryResponse = DirectoryEntry;

/// Type: patch
/// requires VIEW_CHANNEL and MANAGE_GUILD permission
pub fn MODIFY_DIRECTORY_ENTRY_ENDPOINT(
	channel_id: Snowflake,
	entity_id: Snowflake,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyDirectoryEntryRequest {
	/// https://docs.discord.food/resources/directory#directory-category
	pub primary_category_id: u8,
	pub description: String,
}

pub type ModifyDirectoryEntryResponse = DirectoryEntry;

/// Type: delete
/// requires VIEW_CHANNEL and MANAGE_GUILD permission
pub fn DELETE_DIRECTORY_ENTRY_ENDPOINT(
	channel_id: Snowflake,
	entity_id: Snowflake,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

/// Type: delete
pub fn GET_DIRECTORY_BROADCAST_INFO_ENDPOINT(
	channel_id: Snowflake,
	query: GetDirectoryBroadcastInfoRequest,
) -> String {
	format!(
		"/channels/{}/directory-entry/broadcast{}",
		channel_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetDirectoryBroadcastInfoRequest {
	/// https://docs.discord.food/resources/directory#directory-entry-type
	pub r#type: u8,
	pub entity_id: Snowflake,
}

#[derive(Deserialize)]
pub struct GetDirectoryBroadcastInfoResponse {
	can_broadcast: bool,
	has_broadcast: bool,
}
