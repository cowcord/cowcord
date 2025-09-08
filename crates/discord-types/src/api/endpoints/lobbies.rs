use std::collections::HashMap;
#[cfg(feature = "non-user")]
use std::num::NonZeroU32;

use arrayvec::ArrayString;
#[cfg(feature = "non-user")]
use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::api::lobbies::Lobby;
#[cfg(feature = "non-user")]
use crate::api::lobbies::{LobbyMember, LobbyMemberFlags};
use crate::api::messages::Message;
#[cfg(feature = "non-user")]
use crate::common::id::UserId;
use crate::common::id::{ChannelId, LobbyId};

/// Method: `POST`
///
/// Clients will not be able to join or leave a lobby created using this API.
///
/// Creates a new lobby. Returns a [lobby](https://docs.discord.food/resources/lobby#lobby-object) object on success.
/// Fires a [Lobby Create](https://docs.discord.food/topics/gateway-events#lobby-create) Gateway event.
#[cfg(feature = "non-user")]
#[deprecated(note = "usez JOIN_OR_CREATE_LOBBY")]
pub const CREATE_LOBBY: &str = "/lobbies";

#[derive(Serialize, Deserialize)]
#[cfg(feature = "non-user")]
pub struct CreateLobbyRequest {
	/// The metadata of the lobby
	/// (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<ArrayString<1024>, ArrayString<1024>>>>,
	/// The members of the lobby
	/// (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<ArrayVec<LobbyMember, 25>>,
	/// How long to wait (in seconds) before shutting down a lobby after it becomes idle
	/// (min 5, max 604800, default 300)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub idle_timeout_seconds: Option<NonZeroU32>,
}

#[cfg(feature = "non-user")]
pub type CreateLobbyResponse = Lobby;

/// Method: `PUT`
///
/// Supports OAuth2 for authentication with the `lobbies.write` scope
///
/// Joins an existing lobby or creates a new lobby.
///
/// Returns a [lobby](https://docs.discord.food/resources/lobby#lobby-object) object on success.
/// May fire a [Lobby Create](https://docs.discord.food/topics/gateway-events#lobby-create)
/// or [Lobby Member Add](https://docs.discord.food/topics/gateway-events#lobby-member-add) Gateway event.
#[cfg(feature = "non-user")]
pub const JOIN_OR_CREATE_LOBBY: &str = "/lobbies";

#[derive(Serialize, Deserialize)]
#[cfg(feature = "non-user")]
pub struct JoinOrCreateLobbyRequest {
	/// The lobby secret
	/// (max 250 characters)
	///
	///  Secret values expire after 30 days. After this time period, the lobby will still exist, but new users won't be able to join.
	pub secret: ArrayString<250>,
	/// The metadata of the lobby
	/// (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<ArrayString<1024>, ArrayString<1024>>>>,
	/// The members of the lobby
	/// (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<ArrayVec<LobbyMember, 25>>,
	/// How long to wait (in seconds) before shutting down a lobby after it becomes idle
	/// (min 5, max 604800)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub idle_timeout_seconds: Option<NonZeroU32>,
}

#[cfg(feature = "non-user")]
pub type JoinOrCreateLobbyResponse = Lobby;

/// Method: `GET`
///
/// User must be a member of the lobby.
///
/// Returns a [lobby](https://docs.discord.food/resources/lobby#lobby-object) object for the given lobby ID.
#[cfg(feature = "non-user")]
pub fn GET_LOBBY(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}", lobby_id)
}

#[cfg(feature = "non-user")]
pub type GetLobbyResponse = Lobby;

/// Method: `PATCH`
///
/// Application must be the creator of the lobby.
///
/// Modifies a lobby's settings.
///
/// Returns the updated [lobby](https://docs.discord.food/resources/lobby#lobby-object) object on success.
/// Fires a [Lobby Update](https://docs.discord.food/topics/gateway-events#lobby-update) Gateway event.
#[cfg(feature = "non-user")]
pub fn MODIFY_LOBBY(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}", lobby_id)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "non-user")]
pub struct ModifyLobbyRequest {
	/// The metadata of the lobby
	/// (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<ArrayString<1024>, ArrayString<1024>>>>,
	/// The members of the lobby
	/// (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<ArrayVec<LobbyMember, 25>>,
	/// How long to wait (in seconds) before shutting down a lobby after it becomes idle
	/// (min 5, max 604800, default 300)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub idle_timeout_seconds: Option<NonZeroU32>,
}

#[cfg(feature = "non-user")]
pub type ModifyLobbyResponse = Lobby;

/// Method: `DELETE`
///
/// Application must be the creator of the lobby.
///
/// Deletes a lobby.
///
/// Returns a 204 empty response on success.
/// Fires a [Lobby Delete](https://docs.discord.food/topics/gateway-events#lobby-delete) Gateway event.
#[cfg(feature = "non-user")]
pub fn DELETE_LOBBY(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}", lobby_id)
}

/// Method: `PUT`
///
/// Adds a member to the lobby.
///
/// If the user is already a member of the lobby, this will update the member and fire a [Lobby Member Update](https://docs.discord.food/topics/gateway-events#lobby-member-update) Gateway event instead.
///
/// Returns the [lobby member](https://docs.discord.food/resources/lobby#lobby-member-object) object.
/// Fires a [Lobby Member Add](https://docs.discord.food/topics/gateway-events#lobby-member-add)
/// or [Lobby Member Update](https://docs.discord.food/topics/gateway-events#lobby-member-update) Gateway event.
#[cfg(feature = "non-user")]
pub fn ADD_LOBBY_MEMBER(
	lobby_id: &LobbyId,
	user_id: &UserId,
) -> String {
	format!("/lobbies/{}/members/{}", lobby_id, user_id)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "non-user")]
pub struct AddLobbyMemberRequest {
	/// The metadata of the lobby member
	/// (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<ArrayString<1024>, ArrayString<1024>>>>,
	/// The lobby member's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<LobbyMemberFlags>,
}

#[cfg(feature = "non-user")]
pub type AddLobbyMemberResponse = Lobby;

/// Method: `DELETE`
///
/// Removes a member from the lobby.
///
/// Returns a 204 empty response on success.
/// Fires a [Lobby Member Remove](https://docs.discord.food/topics/gateway-events#lobby-member-remove) Gateway event.
#[cfg(feature = "non-user")]
pub fn REMOVE_LOBBY_MEMBER(
	lobby_id: &LobbyId,
	user_id: &UserId,
) -> String {
	format!("/lobbies/{}/members/{}", lobby_id, user_id)
}

/// Method: `DELETE`
///
/// This endpoint is only usable with the `lobbies.write` OAuth2 scope
///
/// Removes the current user from the lobby.
///
/// Returns a 204 empty response on success.
/// Fires a [Lobby Delete](https://docs.discord.food/topics/gateway-events#lobby-delete)
/// and [Lobby Member Remove](https://docs.discord.food/topics/gateway-events#lobby-member-remove) Gateway event.
pub fn LEAVE_LOBBY(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}/members/@me", lobby_id)
}

/// Method: `PATCH`
///
/// This endpoint is only usable with the `lobbies.write` OAuth2 scope
///
/// Links or unlinks a channel to the lobby.
///
/// The application must be the creator of the lobby or the member must have the [`CAN_LINK_LOBBY` flag](https://docs.discord.food/resources/lobby#lobby-member-flags).
///
/// Returns a [lobby](https://docs.discord.food/resources/lobby#lobby-object) object on success.
/// Fires a [Lobby Update](https://docs.discord.food/topics/gateway-events#lobby-update)
/// and [Channel Update](https://docs.discord.food/topics/gateway-events#channel-update) Gateway event.
pub fn MODIFY_LOBBY_LINKED_CHANNEL(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}/channel-linking", lobby_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyLobbyLinkedChannelRequest {
	/// The ID of the channel to link to the lobby
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<Option<ChannelId>>,
}

pub type ModifyLobbyLinkedChannelResponse = Lobby;

/// Method: `POST`
///
/// This endpoint is only usable with the `lobbies.write` OAuth2 scope
///
/// Posts a message to a lobby.
///
/// Returns a partial [message](https://docs.discord.food/resources/message#message-object) object on success.
/// Fires a [Lobby Message Create](https://docs.discord.food/topics/gateway-events#lobby-message-create)
/// and [Message Create](https://docs.discord.food/topics/gateway-events#message-create) Gateway event.
/// Functionally identical to the [Create Message](https://docs.discord.food/resources/message#create-message) endpoint,
/// but is used for lobbies in an OAuth2 context and has some additional parameters. Check there for more information.
pub fn CREATE_LOBBY_MESSAGE(lobby_id: &LobbyId) -> String {
	format!("/lobbies/{}/messages", lobby_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateLobbyMessageRequest {
	/// Custom metadata for the message
	/// (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<ArrayString<1024>, ArrayString<1024>>>>,
}

pub type CreateLobbyMessageResponse = Message;
