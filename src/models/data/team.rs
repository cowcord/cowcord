#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use super::user::User;
use crate::models::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Team {
	pub id:                        Snowflake,
	pub name:                      String,
	pub icon:                      Option<String>,
	pub owner_user_id:             Snowflake,
	pub members:                   Vec<Member>,
	/// https://docs.discord.sex/resources/team#team-payout-account-status
	pub payout_account_status:     Option<u8>,
	pub stripe_connect_account_id: String,
}

pub enum PayoutAccountStatus {
	UNSUBMITTED = 1,
	PENDING = 2,
	ACTION_REQUIRED = 3,
	ACTIVE = 4,
	BLOCKED = 5,
	SUSPENDED = 6,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Member {
	pub user:             User,
	pub team_id:          Snowflake,
	/// https://docs.discord.sex/resources/team#membership-state
	pub membership_state: u8,
	/// https://docs.discord.sex/resources/team#team-member-role-types
	pub role:             String,
}

pub enum MembershipState {
	INVITED = 1,
	ACCEPTED = 2,
}

pub enum MemberRoleType {
	admin,
	developer,
	read_only,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Payout {
	pub id:                                 Snowflake,
	pub user_id:                            Snowflake,
	pub amount:                             u32,
	/// https://docs.discord.sex/resources/team#team-payout-status
	pub status:                             u8,
	pub period_start:                       Timestamp,
	pub period_end:                         Option<Timestamp>,
	pub payout_date:                        Option<Timestamp>,
	pub latest_tipalti_submission_response: String, // no clue about the type for this
}

pub enum PayoutStatus {
	OPEN = 1,
	PAID = 2,
	PENDING = 3,
	MANUAL = 4,
	CANCELLED = 5,
	DEFERRED = 6,
	DEFERRED_INTERNAL = 7,
	PROCESSING = 8,
	ERROR = 9,
	REJECTED = 10,
	RISK_REVIEW = 11,
	SUBMITTED = 12,
	PENDING_FUNDS = 13,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Company {
	pub id:   Snowflake,
	pub name: String,
}
