use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};

use crate::api::channel::Channel;
use crate::common::id::{ApplicationId, LobbyId, UserId};

#[derive(Serialize, Deserialize)]
pub struct Lobby {
	/// The ID of the lobby
	pub id: LobbyId,
	/// The ID of the application that created the lobby
	pub application_id: ApplicationId,
	/// The metadata of the lobby (max 25 keys, 1024 characters per key and value)
	pub metadata: Option<HashMap<ArrayString<1024>, ArrayString<1024>>>,
	/// The members of the lobby (max 1000)
	pub members: ArrayVec<LobbyMember, 1000>,
	/// The guild channel linked to the lobby
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linked_channel: Option<Channel>,
}

#[derive(Serialize, Deserialize)]
pub struct LobbyMember {
	/// The ID of the user
	pub id: UserId,
	/// The metadata of the lobby member (max 25 keys, 1024 characters per key and value)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Option<HashMap<String, String>>>,
	/// The lobby member's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<LobbyMemberFlags>,
	/// Whether the member is connected to a call in lobby
	pub connected: bool,
}

bitflags! {
	pub struct LobbyMemberFlags: u64 {
		/// Lobby member can link a text channel to the lobby
		const CAN_LINK_LOBBY = 1 << 0;
	}
}
