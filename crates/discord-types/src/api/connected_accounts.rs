use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::integrations::{IntegrationAccount, IntegrationGuild, IntegrationType};
use crate::common::id::{DeviceId, IntegrationId};

#[derive(Serialize, Deserialize)]
pub struct Connection {
	/// ID of the connection account
	pub id: String,
	/// The type of the connection
	pub r#type: ConnectionType,
	/// The username of the connection account
	pub name: String,
	/// Whether the connection is verified
	pub verified: bool,
	/// Service-specific metadata about the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Value>,
	/// Visibility of the connection's metadata
	pub metadata_visibility: VisibilityType,
	/// whether the connection is revoked
	pub revoked: bool,
	/// The guild integrations attached to the connection
	pub integrations: Vec<ConnectionIntegration>,
	/// Whether friend sync is enabled for this connection
	pub friend_sync: bool,
	/// Whether activities related to this connection will be shown in presence
	pub show_activity: bool,
	/// Whether this connection has a corresponding third party OAuth2 token
	pub two_way_link: bool,
	/// Visibility of the connection
	pub visibility: VisibilityType,
	/// The access token for the connection account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PartialConnection {
	/// ID of the connection account
	pub id: String,
	/// The type of the connection
	pub r#type: ConnectionType,
	/// The username of the connection account
	pub name: String,
	/// Whether the connection is verified
	pub verified: bool,
	/// Service-specific metadata about the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionIntegration {
	/// The ID of the integration
	pub id: IntegrationId,
	/// The type of integration
	pub r#type: IntegrationType,
	/// The integration's account information
	pub account: IntegrationAccount,
	/// The guild that the integration is attached to
	pub guild: IntegrationGuild,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConnectionType {
	#[serde(rename = "amazon-music")]
	amazon_music,
	battlenet,
	bluesky,
	bungie,
	contacts,
	crunchyroll,
	domain,
	ebay,
	epicgames,
	facebook,
	github,
	instagram,
	leagueoflegends,
	mastodon,
	paypal,
	playstation,
	#[serde(rename = "playstation-stg")]
	playstation_stg,
	reddit,
	roblox,
	riotgames,
	samsung,
	soundcloud,
	spotify,
	skype,
	steam,
	tiktok,
	twitch,
	twitter,
	xbox,
	youtube,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VisibilityType {
	/// Invisible to everyone except the user themselves
	NONE = 0,
	/// Visible to everyone
	EVERYONE = 1,
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleDevice {
	/// The ID of the device
	pub id: DeviceId,
	/// The name of the device
	pub name: String,
	/// The console platform (only playstation and playstation-stg are allowed)
	pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub enum TwoWayLinkType {
	/// The connection is linked via web
	web,
	/// The connection is linked via mobile
	mobile,
	/// The connection is linked via desktop
	desktop,
}

#[derive(Serialize, Deserialize)]
pub struct FriendListEntry {
	/// E.164-formatted phone number of the contact
	pub friend_id: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContactSyncSuggestionsSetting {
	/// Users who have contact synced that have the current user as a contact
	MUTUAL_CONTACT_INFO_ONLY = 1,
	/// Users who have contact synced
	ANYONE_WITH_CONTACT_INFO = 2,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectRequestProperties {
	/// The console handoff type
	pub handoff_type: String,
}

#[derive(Serialize, Deserialize)]
pub enum ConsoleHandoffType {
	/// Create a new call on a console device
	CREATE_NEW_CALL,
	/// Transfer an existing call to a console device
	TRANSFER_EXISTING_CALL,
}

#[derive(Serialize, Deserialize)]
pub enum ConsoleCommandType {
	/// Connect to a voice call
	connect_voice,
}
