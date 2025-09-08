use std::collections::HashMap;
use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::api::integrations::{
	Gif,
	GifCategory,
	GifMediaFormat,
	Integration,
	IntegrationExpireBehavior,
	IntegrationType,
	TenorGif,
};
use crate::common::id::{ChannelId, GuildId, IntegrationId};
use crate::common::locale::Locale;
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Requires the MANAGE_GUILD permission.
///
/// This endpoint returns a maximum of 50 integrations. If a guild has more integrations, they cannot be accessed.
///
/// Returns a list of [integration](https://docs.discord.food/resources/integration#integration-object) objects for the guild.
pub fn GET_GUILD_INTEGRATIONS(
	query: &GetGuildIntegrationsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/integrations{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildIntegrationsQueryParams {
	/// Whether to only include Discord application integrations with registered commands (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_commands: Option<bool>,
	/// Whether to include integration role connection metadata (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_role_connections_metadata: Option<bool>,
}

pub type GetGuildIntegrationsResponse = Vec<Integration>;

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Enables an integration for the guild.
///
/// Returns a 204 empty response on success.
/// Fires [Guild Integrations Update](https://docs.discord.food/topics/gateway-events#guild-integrations-update)
/// and [Integration Create](https://docs.discord.food/topics/gateway-events#integration-create) Gateway events.
pub fn CREATE_GUILD_INTEGRATION(guild_id: &GuildId) -> String {
	format!("/guilds/{}/integrations", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildIntegration {
	/// The type of integration to enable
	/// (only `twitch` and `youtube` are supported)
	pub r#type: IntegrationType,
	/// The ID of the integration account to enable
	pub id: IntegrationId,
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Syncs an integration for the guild.
///
/// Returns a 204 empty response on success.
/// Fires [Guild Integrations Update](https://docs.discord.food/topics/gateway-events#guild-integrations-update)
/// and [Integration Update](https://docs.discord.food/topics/gateway-events#integration-update) Gateway events.
pub fn SYNC_GUILD_INTEGRATION(
	guild_id: &GuildId,
	integration_id: &IntegrationId,
) -> String {
	format!("/guilds/{}/integrations/{}/sync", guild_id, integration_id)
}

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Modifies the behavior and settings of the integration in the guild.
///
/// Returns a 204 empty response on success.
/// Fires [Guild Integrations Update](https://docs.discord.food/resources/integration#integration-object)
/// and [Integration Update](https://docs.discord.food/resources/integration#integration-object) Gateway events.
pub fn MODIFY_GUILD_INTEGRATION(
	guild_id: &GuildId,
	integration_id: &IntegrationId,
) -> String {
	format!("/guilds/{}/integrations/{}", guild_id, integration_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildIntegrationRequest {
	/// The behavior of expiring subscribers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expire_behavior: Option<IntegrationExpireBehavior>,
	/// The grace period before expiring subscribers
	/// (one of 1, 3, 7, 14, 30, in days)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expire_grace_period: Option<u8>,
	/// Whether emoticons should be synced for this integration
	/// (Twitch only)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enable_emoticons: Option<bool>,
}

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the MANAGE_GUILD permission.
///
/// Removes the given integration ID from the guild.
/// Deletes any associated webhooks and kicks the associated bot (if there is one).
///
/// Returns a 204 empty response on success.
/// Fires [Guild Integrations Update](https://docs.discord.food/topics/gateway-events#guild-integrations-update),
/// [Integration Delete](https://docs.discord.food/topics/gateway-events#integration-delete),
/// and optionally [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove)
/// and [Webhooks Update](https://docs.discord.food/topics/gateway-events#webhooks-update) Gateway events.
pub fn DELETE_GUILD_INTEGRATION(
	guild_id: &GuildId,
	integration_id: &IntegrationId,
) -> String {
	format!("/guilds/{}/integrations/{}", guild_id, integration_id)
}

/// Method: `POST`
///
/// Requires the MANAGE_GUILD permission.
///
/// Migrates all Discord application integrations in the guild to the `applications.commands` [OAuth2 scope](https://docs.discord.food/topics/oauth2#oauth2-scopes).
///
/// Fires a [Guild Integrations Update](https://docs.discord.food/topics/gateway-events#guild-integrations-update)
/// and multiple [Integration Update](https://docs.discord.food/topics/gateway-events#integration-update) Gateway events.
pub fn MIGRATE_GUILD_COMMAND_SCOPE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/migrate-command-scope", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct MigrateGuildCommandScopeResponse {
	/// The IDs of migrated integrations that now have application commands registered
	pub integration_ids_with_app_commands: Vec<IntegrationId>,
}

/// Method: `GET`
///
/// Returns a mapping of guild IDs to lists of application IDs attached to the integrations in the current user's guilds.
pub const GET_GUILD_INTEGRATION_APPLICATION_IDS: &str =
	"/users/@me/guilds/integration-application-ids";

pub type GetGuildIntegrationApplicationIds = HashMap<GuildId, Vec<IntegrationId>>;

/// Method: `GET`
///
/// This endpoint returns a maximum of 50 integrations.
/// If a channel has more integrations, they cannot be accessed.
///
/// Returns a list of [integration](https://docs.discord.food/resources/integration#integration-object) objects for the private channel.
pub fn GET_CHANNEL_INTEGRATIONS(channel_id: &ChannelId) -> String {
	format!("/channels/{}/integrations", channel_id)
}

pub type GetChannelIntegrationsResponse = Vec<Integration>;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Removes the given integration ID from the channel.
///
/// Returns a 204 empty response on success.
/// Fires an [Integration Delete](https://docs.discord.food/resources/integration#integration-object) Gateway event.
pub fn DELETE_CHANNEL_INTEGRATION(
	channel_id: &ChannelId,
	integration_id: &IntegrationId,
) -> String {
	format!("/channels/{}/integrations/{}", channel_id, integration_id)
}

/// Method: `POST`
///
/// Joins the user to the given integration ID's guild.
///
/// This endpoint is only usable with [integrations found on the user's connections](https://docs.discord.food/resources/connected-accounts#connection-object).
///
/// Returns a 204 empty response on success.
/// Fires a [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create) Gateway event.
pub fn JOIN_INTEGRATION_GUILD(integration_id: &IntegrationId) -> String {
	format!("/integrations/{}/join", integration_id)
}

/// Method: `GET`
///
/// Returns a list of up to 10 [GIFs sourced from Tenor](https://docs.discord.food/resources/integration#tenor-gif-structure) based on the provided query.
pub fn SEARCH_TENOR_GIFS(query: &SearchTenorGifsQueryParams) -> String {
	format!("/integrations/tenor/search{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct SearchTenorGifsQueryParams {
	/// The search query to use
	pub q: String,
}

pub type SearchTenorGifsResponse = Vec<TenorGif>;

/// Method: `GET`
///
/// Returns a list of the top trending search terms.
pub fn GET_TRENDING_GIF_SEARCH_TERMS(query: &GetTrendingGifSearchTermsQueryParams) -> String {
	format!("/gifs/trending-search{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetTrendingGifSearchTermsQueryParams {
	/// The maximum number of search terms to return
	/// (1-50, default 5)
	///
	/// The limit is only a suggestion, the API may return fewer GIFs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU8>,
	/// The locale to use in search results
	/// (default en-US)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
}

pub type GetTrendingGifSearchTermsResponse = Vec<String>;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a list of recommended search terms based on the provided query.
pub fn GET_SUGGESTED_GIF_SEARCH_TERMS(query: &GetSuggestedGifSearchTermsQueryParams) -> String {
	format!("/gifs/suggest{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetSuggestedGifSearchTermsQueryParams {
	/// The search query to use
	pub q: String,
	/// The maximum number of search terms to return
	/// (1-50, default 20)
	///
	/// The limit is only a suggestion; the API may return fewer GIFs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU8>,
	/// The locale to use in search results
	/// (default en-US)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
}

pub type GetSuggestedGifSearchTermsResponse = Vec<String>;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a list of [GIF](https://docs.discord.food/resources/integration#gif-structure) objects based on the provided query.
pub fn SEARCH_GIFS(query: &SearchGifsQueryParams) -> String {
	format!("/gifs/search{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct SearchGifsQueryParams {
	/// The search query to use
	pub q: String,
	/// The maximum number of GIFs to return
	/// (20-500)
	///
	/// The limit is only a suggestion; the API may return fewer GIFs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// The media format to use
	pub media_format: GifMediaFormat,
	/// The locale to use in search results (default en-US)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
}

pub type SearchGifsResponse = Vec<Gif>;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns trending GIF categories and their associated preview GIFs.
pub fn GET_TRENDING_GIF_CATEGORIES(query: &GetTrendingGifCategoriesQueryParams) -> String {
	format!("/gifs/trending{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetTrendingGifCategoriesQueryParams {
	/// The media format to use
	pub media_format: GifMediaFormat,
	/// The locale to use in search results
	/// (default en-US)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTrendingGifCategoriesResponse {
	/// The trending GIF categories
	pub categories: Vec<GifCategory>,
	/// A trending GIF to use as a placeholder
	pub gifs: Vec<Gif>,
}

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a list of [GIF](https://docs.discord.food/resources/integration#gif-structure) objects that are currently trending.
pub fn GET_TRENDING_GIFS(query: &GetTrendingGifsQueryParams) -> String {
	format!("/gifs/trending-gifs{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetTrendingGifsQueryParams {
	/// The maximum number of GIFs to return
	/// (20-500)
	///
	/// The limit is only a suggestion; the API may return fewer GIFs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// The media format to use
	pub media_format: GifMediaFormat,
	/// The locale to use in search results
	/// (default en-US)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
}

pub type GetTrendingGifs = Vec<Gif>;

/// Method: `POST`
///
/// Tracks the selection of a GIF by the user.
///
/// Returns a 204 empty response on success.
pub const TRACK_SELECTED_GIF: &str = "/gifs/select";

#[derive(Serialize, Deserialize)]
pub struct JsonParams {
	/// The ID of the selected GIF
	pub id: String,
	/// The search query used to find the GIF
	pub q: String,
}
