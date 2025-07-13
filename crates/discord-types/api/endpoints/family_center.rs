#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::models::api_types::family_center::{FamilyCenter, LinkedUsers};
use crate::models::api_types::types::Snowflake;

/// Type: get
pub const GET_FAMILY_CENTER_OVERVIEW_ENDPOINT: &str = "/family-center/@me";

pub type GetFamilyCenterOverviewResponse = FamilyCenter;

/// Type: get
pub const GET_LINK_CODE_ENDPOINT: &str = "/family-center/@me/link-code";

#[derive(Deserialize)]
pub struct GetLinkCodeResponse {
	pub link_code: String,
}

/// Type: get
pub const GET_LINKED_USERS_ENDPOINT: &str = "/family-center/@me/linked-users";

pub type GetLinkedUsersResponse = LinkedUsers;

/// Type: post
pub const CREATE_LINKED_USERS_ENDPOINT: &str = "/family-center/@me/linked-users";

#[derive(Serialize)]
pub struct CreateLinkedUsersRequest {
	pub recipient_id: Snowflake,
	pub code:         String,
}

pub type CreateLinkedUsersResponse = LinkedUsers;

/// Type: patch
pub const MODIFY_LINKED_USERS_ENDPOINT: &str = "/family-center/@me/linked-users";

#[derive(Serialize)]
pub struct ModifyLinkedUsersRequest {
	/// https://docs.discord.food/resources/family-center#link-status
	pub link_status:    u8,
	pub linked_user_id: String,
}

pub type ModifyLinkedUsersResponse = Vec<LinkedUsers>;
