use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::connected_accounts::{
	ConnectRequestProperties,
	Connection,
	ConnectionType,
	ConsoleCommandType,
	ConsoleDevice,
	ContactSyncSuggestionsSetting,
	FriendListEntry,
	TwoWayLinkType,
	VisibilityType,
};
use crate::api::relationships::FriendSuggestion;
use crate::common::id::{ChannelId, ConsoleCommandId, DeviceId, GuildId};
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns an authorization link that can be used for authorizing a new connection.
pub fn AUTHORIZE_USER_CONNECTION(
	query: &AuthorizeUserConnectionQueryParams,
	connection_type: &ConnectionType,
) -> String {
	format!(
		"/connections/{:?}/authorize{}",
		connection_type,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizeUserConnectionQueryParams {
	/// The type of two-way link to create
	#[serde(skip_serializing_if = "Option::is_none")]
	pub two_way_link_type: Option<Option<TwoWayLinkType>>,
	/// The device code to use for the two-way link
	#[serde(skip_serializing_if = "Option::is_none")]
	pub two_way_user_code: Option<Option<String>>,
	/// Whether this is a continuation of a previous authorization
	#[serde(skip_serializing_if = "Option::is_none")]
	pub continuation: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizeUserConnectionResponse {
	/// The authorization link for the user
	pub url: String,
}

/// Method: `POST`
///
/// Creates a new connection for the current user.
///
/// Returns a [connection](https://docs.discord.food/resources/connected-accounts#connection-object) object on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn CREATE_USER_CONNECTION_CALLBACK(connection_type: &ConnectionType) -> String {
	format!("/connections/{:?}/callback", connection_type)
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserConnectionCallbackRequest {
	/// The authorization code for the connection
	pub code: String,
	/// The state used to authorize the connection
	pub state: String,
	/// The code to use for two-way linking
	#[serde(skip_serializing_if = "Option::is_none")]
	pub two_way_link_code: Option<String>,
	/// Whether the connection is insecure
	#[serde(skip_serializing_if = "Option::is_none")]
	pub insecure: Option<bool>,
	/// Whether to sync friends over the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub friend_sync: Option<bool>,
	/// Additional parameters for OpenID Connect
	#[serde(skip_serializing_if = "Option::is_none")]
	pub openid_params: Option<Value>,
}

pub type CreateUserConnectionCallbackResponse = Connection;

/// Method: `PUT`
///
/// Creates a new contact sync connection for the current user.
///
/// This endpoint is only usable to create contact sync connections (nominally with an ID of @me). For most other connections, use `AUTHORIZE_USER_CONNECTION` and `CREATE_USER_CONNECTION_CALLBACK`.
///
/// Returns a [connection](https://docs.discord.food/resources/connected-accounts#connection-object) object on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn CREATE_CONTACT_SYNC_CONNECTION(connection_id: &str) -> String {
	format!("/users/@me/connections/contacts/{:?}", connection_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateContactSyncConnection {
	/// The username of the connection account
	pub name: String,
	/// Whether to sync friends over the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub friend_sync: Option<bool>,
}

/// Method: `PUT`
///
/// Syncs the user's device contacts to the connection.
///
/// May fire multiple [Friend Suggestion Create](https://docs.discord.food/topics/gateway-events#friend-suggestion-create) Gateway events.
pub fn UPDATE_EXTERNAL_FRIEND_LIST_ENTRIES(connection_id: &str) -> String {
	format!(
		"/users/@me/connections/contacts/{}/external-friend-list-entries",
		connection_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExternalFriendListEntriesRequest {
	/// The phone numbers to sync (max 10000)
	pub friend_list_entries: ArrayVec<FriendListEntry, 10_000>,
	/// Whether the request is a background sync (will not return suggestions)
	pub background: bool,
	/// The contact sync suggestions setting
	pub allowed_in_suggestions: ContactSyncSuggestionsSetting,
	/// Whether to show the mutual friend count of contacts
	pub include_mutual_friends_count: bool,
	/// Whether to add users that have contact synced the current user as friend suggestions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub add_reverse_friend_suggestions: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExternalFriendListEntriesResponse {
	/// Token to be used for bulk adding relationships
	pub bulk_add_token: Option<String>,
	/// Suggested users
	pub friend_suggestions: Vec<FriendSuggestion>,
}

/// Method: `GET`
pub fn CONTACT_SYNC_SETTINGS(connection_id: &str) -> String {
	format!(
		"/users/@me/connections/contacts/{}/external-friend-list-entries/settings",
		connection_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct ContactSyncSettingsResponse {
	/// The contact sync suggestions setting
	pub allowed_in_suggestions: ContactSyncSuggestionsSetting,
}

/// Method: `POST`
///
/// Creates a new domain connection for the current user.
///
/// Discord will verify that the domain is owned by the user. If this verification fails, the endpoint will return an error response:
///
/// ```json
/// { "message": "Unable to validate domain.", "code": 50187, "proof": "dh=dceaca792e3c40fcf356a9297949940af5cfe538" }
/// ```
///
/// The proof provided must be added to the domain's DNS records as a TXT record with the name `_discord.<domain>`.
/// Alternatively, the proof can be served at `https://<domain>/.well-known/discord`.
///
/// After adding the proof, the request should be retried.
///
/// Returns a [connection](https://docs.discord.food/resources/connected-accounts#connection-object) object on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn CREATE_DOMAIN_CONNECTION(connection_id: &str) -> String {
	format!("/users/@me/connections/domain/{}", connection_id)
}

pub type CreateDomainConnectionResponse = Connection;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `connections` scope
///
/// Returns a list of [connection](https://docs.discord.food/resources/connected-accounts#connection-object) objects.
pub const GET_USER_CONNECTIONS: &str = "/users/@me/connections";

pub type GetUserConnectionsResponse = Vec<Connection>;

/// Method: `GET`
///
/// Returns a new access token for the given connection.
///
/// Only available for Twitch, YouTube, and Spotify connections.
///
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn GET_USER_CONNECTION_ACCESS_TOKEN(
	connection_type: &ConnectionType,
	connection_id: &str,
) -> String {
	format!(
		"/users/@me/connections/{:?}/{}/access-token",
		connection_type, connection_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetUserConnectionAccessTokenResponse {
	/// The connection's access token
	pub access_token: String,
}

/// Method: `GET`
///
/// Only available for Reddit connections.
///
/// Returns a list of [subreddits](https://docs.discord.food/resources/connected-accounts#subreddit-structure) the connected account moderates.
pub fn GET_USER_CONNECTION_SUBREDDITS(connection_id: &str) -> String {
	format!("/users/@me/connections/reddit/{}/subreddits", connection_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetUserConnectionSubredditsResponseInner {
	/// The subreddit's ID
	pub id: String,
	/// The number of joined Reddit users
	pub subscribers: u32,
	/// The subreddit's relative URL
	pub url: String,
}

pub type GetUserConnectionSubredditsResponse = Vec<GetUserConnectionSubredditsResponseInner>;

/// Method: `POST`
///
/// Refreshes a connection.
///
/// Returns a 204 empty response on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn REFRESH_USER_CONNECTION(
	connection_type: &ConnectionType,
	connection_id: &str,
) -> String {
	format!(
		"/users/@me/connections/{:?}/{}/refresh",
		connection_type, connection_id
	)
}

/// Method: `PATCH`
///
/// Modifies a connection.
///
/// Not all connection types support all parameters.
///
/// Returns a [connection](https://docs.discord.food/resources/connected-accounts#connection-object) object on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) Gateway event.
pub fn MODIFY_USER_CONNECTION(
	connection_type: &ConnectionType,
	connection_id: &str,
) -> String {
	format!(
		"/users/@me/connections/{:?}/{}",
		connection_type, connection_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyUserConnectionRequest {
	/// The connection's username
	pub name: String,
	/// Whether activities related to this connection will be shown in presence
	pub show_activity: bool,
	/// Whether friend sync is enabled for this connection
	pub friend_sync: bool,
	/// Visibility of the connection's metadata
	pub metadata_visibility: VisibilityType,
	/// Visibility of the connection
	pub visibility: VisibilityType,
}

pub type ModifyUserConnectionResponse = Connection;

/// Method: `DELETE`
///
/// Deletes a connection.
///
/// Deleting a connection will remove you from any guilds you joined via the connection's [integrations](https://docs.discord.food/resources/connected-accounts#connection-integration-structure).
///
/// Returns a 204 empty response on success.
/// Fires a [User Connections Update](https://docs.discord.food/topics/gateway-events#user-connections-update) and optionally a [Guild Delete](https://docs.discord.food/topics/gateway-events#guild-delete) Gateway event.
pub fn DELETE_USER_CONNECTION(
	connection_type: &ConnectionType,
	connection_id: &str,
) -> String {
	format!(
		"/users/@me/connections/{:?}/{}",
		connection_type, connection_id
	)
}

/// Method: `GET`
///
/// This endpoint is only usable with OAuth2 for authentication with the `connections` scope
///
/// Returns a list of [connection](https://docs.discord.food/resources/connected-accounts#connection-object) objects that have a two-way link with the application making the request.
pub const GET_USER_LINKED_CONNECTIONS: &str = "/users/@me/linked-connections";

pub type GetUserLinkedConnectionsResponse = Vec<Connection>;

/// Method: `POST`
///
/// Returns a nonce for connecting to voice on PlayStation consoles.
pub const CREATE_CONSOLE_CONNECTION: &str = "/consoles/connect-request";

#[derive(Serialize, Deserialize)]
pub struct CreateConsoleConnectionRequest {
	/// The properties used for analytics
	#[serde(skip_serializing_if = "Option::is_none")]
	pub analytics_properties: Option<ConnectRequestProperties>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateConsoleConnectionResponse {
	/// The nonce
	pub nonce: String,
}

/// Method: `DELETE`
///
/// Cancels a console connection request.
///
/// Returns a 204 empty response on success.
pub fn CANCEL_CONSOLE_CONNECTION_REQUEST(nonce: &str) -> String {
	format!("/consoles/connect-request/{}", nonce)
}

/// Method: `GET`
///
/// Only supports playstation and playstation-stg connection types.
///
/// Returns the consoles associated with the given connection type.
pub fn GET_CONSOLE_DEVICES(connection_type: &ConnectionType) -> String {
	format!("/consoles/{:?}/devices", connection_type)
}

#[derive(Serialize, Deserialize)]
pub struct GetConsoleDevicesResponse {
	/// The user console devices
	pub devices: Vec<ConsoleDevice>,
}

/// Method: `POST`
///
/// Sends a command to connect to a voice call on a console device.
pub fn SEND_CONSOLE_COMMAND(
	connection_type: &ConnectionType,
	device_id: &DeviceId,
) -> String {
	format!(
		"/consoles/{:?}/devices/{}/commands",
		connection_type, device_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct SendConsoleCommandRequest {
	/// The command type
	pub command: ConsoleCommandType,
	/// The ID of the channel to connect to
	pub channel_id: ChannelId,
	/// The ID of the guild the channel is in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// The nonce obtained from `CREATE_CONSOLE_CONNECTION` endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nonce: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SendConsoleCommandResponse {
	/// The ID of the sent command
	pub id: ConsoleCommandId,
}

/// Method: `DELETE`
///
/// Cancels a console command.
///
/// Returns a 204 empty response on success.
pub fn CANCEL_CONSOLE_COMMAND(
	connection_type: &ConnectionType,
	device_id: &DeviceId,
	command_id: &ConsoleCommandId,
) -> String {
	format!(
		"/consoles/{:?}/devices/{}/commands/{}",
		connection_type, device_id, command_id
	)
}
