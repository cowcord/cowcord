use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::users::PartialUser;
use crate::common::id::{EmojiId, RoleId};

#[derive(Serialize, Deserialize)]
pub struct Emoji {
	/// The ID of the emoji
	pub id: Option<EmojiId>,
	/// The name of the emoji (2-32 characters)
	pub name: ArrayString<32>,
	/// The roles allowed to use the emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<RoleId>>,
	/// The user that uploaded the emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// Whether this emoji must be wrapped in colons
	#[serde(skip_serializing_if = "Option::is_none")]
	pub require_colons: Option<bool>,
	/// Whether this emoji is managed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub managed: Option<bool>,
	/// Whether this emoji is animated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub animated: Option<bool>,
	/// Whether this emoji can be used; may be false due to loss of premium subscriptions (boosts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available: Option<bool>,
}
