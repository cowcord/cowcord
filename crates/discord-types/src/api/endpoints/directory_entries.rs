use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};

use crate::api::directory_entries::{DirectoryCategory, DirectoryEntry, DirectoryEntryType};
use crate::common::id::{ChannelId, DirectoryEntryId, EntityId, GuildId};
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Requires the `VIEW_CHANNEL` permission.
///
/// Returns a mapping of [directory categories](https://docs.discord.food/resources/directory-entry#directory-category) to their entry count in the given directory channel.
pub fn GET_DIRECTORY_COUNTS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/directory-entries/counts", channel_id)
}

pub type GetDirectoryCountsResponse = HashMap<DirectoryCategory, u32>;

/// Method: `GET`
///
/// Requires the `VIEW_CHANNEL` permission.
///
/// Returns a list of [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) objects in the given directory channel.
pub fn GET_DIRECTORY_ENTRIES(
	query: &GetDirectoryEntriesQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/directory-entries{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetDirectoryEntriesQueryParams {
	/// The type of directory entry to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<DirectoryEntryType>,
	/// The primary category to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_id: Option<DirectoryCategory>,
}

pub type GetDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Method: `GET`
///
/// Requires the VIEW_CHANNEL permission.
///
/// Returns a list of partial [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) objects in the given directory channel.
pub fn GET_PARTIAL_DIRECTORY_ENTRIES(
	query: &GetPartialDirectoryEntriesQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/directory-entries/list{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetPartialDirectoryEntriesQueryParams {
	/// The IDs of the directory entries to retrieve (max 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity_ids: Option<ArrayVec<DirectoryEntryId, 100>>,
}

pub type GetPartialDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Method: `GET`
///
/// Requires the `VIEW_CHANNEL` permission.
///
/// Returns a list of [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) objects in the given directory channel that match the query.
pub fn SEARCH_DIRECTORY_ENTRIES(
	query: &SearchDirectoryEntriesQueryParams,
	channel_id: &ChannelId,
) -> String {
	format!(
		"/channels/{}/directory-entries/search{}",
		channel_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct SearchDirectoryEntriesQueryParams {
	/// The query to search for (1-100 characters)
	pub query: ArrayString<100>,
	/// The type of directory entry to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<DirectoryEntryType>,
	/// The primary category to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_id: Option<DirectoryCategory>,
}

pub type SearchDirectoryEntriesResponse = Vec<DirectoryEntry>;

/// Method: `GET`
///
/// Requires the VIEW_CHANNEL permission.
///
/// Returns a [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) object for the given entity ID in the directory channel.
pub fn GET_DIRECTORY_ENTRY(
	channel_id: &ChannelId,
	entity_id: &EntityId,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

pub type GetDirectoryEntryResponse = DirectoryEntry;

/// Method: `POST`
///
/// Requires the `VIEW_CHANNEL` permission and the `MANAGE_GUILD` permission on the entity being added.
///
/// Returns the new [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) object on success.
/// Fires a [Guild Directory Entry Create](https://docs.discord.food/topics/gateway-events#guild-directory-entry-create) Gateway event.
pub fn CREATE_DIRECTORY_ENTRY(
	channel_id: &ChannelId,
	entity_id: &EntityId,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateDirectoryEntryRequest {
	/// The type of directory entry to create (default `GUILD`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<DirectoryEntryType>,
	/// The primary category of the entry (default `UNCATEGORIZED`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_category_id: Option<DirectoryCategory>,
	/// The description of the entry (max 200 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<200>>>,
}

pub type CreateDirectoryEntryResponse = DirectoryEntry;

/// Method: `PATCH`
///
/// Modifies an existing directory entry in the given directory channel.
///
/// Requires the `VIEW_CHANNEL` permission and the `MANAGE_GUILD` permission on the entity being modified.
///
/// Returns the updated [directory entry](https://docs.discord.food/resources/directory-entry#directory-entry-object) object on success.
/// Fires a [Guild Directory Entry Update](https://docs.discord.food/topics/gateway-events#guild-directory-entry-update) Gateway event.
pub fn MODIFY_DIRECTORY_ENTRY(
	channel_id: &ChannelId,
	entity_id: &EntityId,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyDirectoryEntryRequest {
	/// The primary category of the entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_category_id: Option<DirectoryCategory>,
	/// The description of the entry (max 200 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<ArrayString<200>>,
}

pub type ModifyDirectoryEntryResponse = DirectoryEntry;

/// Method: `DELETE`
///
/// Deletes a directory entry in the given directory channel.
///
/// Requires the `VIEW_CHANNEL` permission and the `MANAGE_GUILD` permission on the entity being deleted.
///
/// Returns a 204 empty response on success.
/// Fires a [Guild Directory Entry Delete](https://docs.discord.food/topics/gateway-events#guild-directory-entry-delete) Gateway event.
pub fn DELETE_DIRECTORY_ENTRY(
	channel_id: &ChannelId,
	entity_id: &EntityId,
) -> String {
	format!("/channels/{}/directory-entry/{}", channel_id, entity_id)
}

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns the broadcast information for the given guild and directory entry type.
pub fn GET_DIRECTORY_BROADCAST_INFO(
	query: &GetDirectoryBroadcastInfoQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/directory-entries/broadcast{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetDirectoryBroadcastInfoQueryParams {
	/// The type of directory entry to get broadcast info for
	pub r#type: DirectoryEntryType,
	/// The ID of the directory entry to get broadcast info for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity_id: Option<DirectoryEntryId>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDirectoryBroadcastInfoResponse {
	/// Whether the user can broadcast in any linked directory channels
	pub can_broadcast: bool,
	/// Whether the entity has been broadcasted in any directory channels
	///
	/// Only included when `entity_id` is provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_broadcast: Option<bool>,
}
