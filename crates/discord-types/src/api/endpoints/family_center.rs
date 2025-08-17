use serde::{Deserialize, Serialize};

use crate::api::family_center::{FamilyCenter, LinkStatus, LinkedUsers};
use crate::common::id::UserId;

/// Method: `GET`
///
/// Returns a [Family Center](https://ee085bcf.discord-userdoccers.pages.dev/resources/family-center#family-center-object) object.
pub const GET_FAMILY_CENTER_OVERVIEW: &str = "/family-center/@me";

pub type GetFamilyCenterOverviewResponse = FamilyCenter;

/// Method: `GET`
///
/// Generates the link code for usage in the generated QR code that a linked user receives to give to a requestor.
///
/// The URL the QR Code represents follows this structure: https://discord.com/feature/family-center/my-family/:linked_user_id/:link_code
///
/// Opening this link on the mobile app prompts the "send connection request" screen.
pub const GET_LINK_CODE: &str = "/family-center/@me/link-code";

#[derive(Serialize, Deserialize)]
pub struct GetLinkCodeResponse {
	/// The code used to connect a requestor to a linked user, appended to the end of the URL the QR code encodes
	pub link_code: String,
}

/// Method: `GET`
///
/// Returns a [linked users](https://ee085bcf.discord-userdoccers.pages.dev/resources/family-center#linked-users-object) object.
pub const GET_LINKED_USERS: &str = "/users/@me/linked-users";

pub type GetLinkedUsersResponse = LinkedUsers;

/// Method: `POST`
///
/// Creates a request that appears in the linked user's Family Center.
///
/// Returns a [linked users](https://ee085bcf.discord-userdoccers.pages.dev/resources/family-center#linked-users-object) object on success.
/// Fires a [User Update](https://ee085bcf.discord-userdoccers.pages.dev/topics/gateway-events#user-update) Gateway event.
pub const CREATE_LINKED_USERS_REQUEST: &str = "/users/@me/linked-users";

#[derive(Serialize, Deserialize)]
pub struct CreateLinkedUsersRequest {
	/// The ID of the user the requestor wants to connect to
	pub recipient_id: UserId,
	/// The link code from the linked user's device
	pub code: String,
}

pub type CreateLinkedUsersResponse = LinkedUsers;

/// Method: `PATCH`
///
/// Modifies the linked user status of a linked user.
///
/// Can be invoked by either the linked user or the requestor if used for removing the link.
///
/// Returns an array of [linked user](https://ee085bcf.discord-userdoccers.pages.dev/resources/family-center#linked-user-object) objects on success.
/// Fires a [User Update](https://ee085bcf.discord-userdoccers.pages.dev/topics/gateway-events#user-update) Gateway event.
pub const MODIFY_LINKED_USERS: &str = "/users/@me/linked-users";

#[derive(Serialize, Deserialize)]
pub struct ModifyLinkedUsersRequest {
	/// The new link status of the linked user
	pub link_status: LinkStatus,
	/// The ID of the user the linked user or requestor is modifying
	pub linked_user_id: UserId,
}
