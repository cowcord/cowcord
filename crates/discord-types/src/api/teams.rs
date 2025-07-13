use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{api_types::users::PartialUser, types::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct Team {
	/// The ID of the team
	pub id: Snowflake,
	/// The name of the team
	pub name: String,
	/// The team's icon hash
	pub icon: Option<String>,
	/// The ID of the team's owner
	pub owner_user_id: Snowflake,
	/// The members in the team
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<Vec<TeamMember>>,
	/// The status of the team's primary payout account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payout_account_status: Option<Option<TeamPayoutAccountStatus>>,
	/// The statuses of the team's payout accounts
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payout_account_statuses: Option<Vec<TeamPayoutAccount>>,
	/// The ID of the team's Stripe Connect account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stripe_connect_account_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamPayoutAccount {
	/// The payout gateway used
	pub gateway: i64,
	/// The status of the payout account
	pub status: TeamPayoutAccountStatus,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TeamPayoutGateway {
	/// Stripe Top-Up
	STRIPE_TOPUP = 1,
	/// Tipalti
	TIPALTI = 2,
	/// Stripe
	STRIPE_PRIMARY = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TeamPayoutAccountStatus {
	/// Team has not submitted a payout account application
	UNSUBMITTED = 1,
	/// Team's payout account application is pending approval
	PENDING = 2,
	/// Team's payout account requires action to receive payouts
	ACTION_REQUIRED = 3,
	/// Team's payout account is active and can receive payouts
	ACTIVE = 4,
	/// Team's payout account is blocked and cannot receive payouts
	BLOCKED = 5,
	/// Team's payout account is suspended and cannot receive payouts
	SUSPENDED = 6,
}

#[derive(Serialize, Deserialize)]
pub struct TeamMember {
	/// The user this team member represents
	pub user: PartialUser,
	/// The ID of the team the user is a member of
	pub team_id: Snowflake,
	/// The user's team membership state
	pub membership_state: i64,
	/// The user's role on the team
	pub role: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TeamMembershipState {
	/// The user is invited
	INVITED = 1,
	/// The user has accepted the invite
	ACCEPTED = 2,
}

#[derive(Serialize, Deserialize)]
pub enum TeamMemberRoleTypes {
	/// Admins have similar access to owners, except they cannot take destructive actions on the team or team-owned apps.
	admin,
	/// Developers can access information about team-owned apps, like the client secret or public key. They can also take limited actions on team-owned apps, like configuring interaction endpoints or resetting the bot token. Members with the Developer role cannot manage the team or its members, or take destructive actions on team-owned apps.
	developer,
	/// Read-only members can access information about a team and any team-owned apps. Some examples include getting the IDs of applications and exporting payout records.
	read_only,
}

#[derive(Serialize, Deserialize)]
pub struct TeamPayout {
	/// The ID of the payout
	pub id: Snowflake,
	/// The ID of the user who receives the payout
	pub user_id: Snowflake,
	/// The amount of the payout
	pub amount: i64,
	/// The status of the payout
	pub status: i64,
	/// When the payout period started
	pub period_start: Timestamp,
	/// When the payout period ended
	pub period_end: Option<Timestamp>,
	/// When the payout was made
	pub payout_date: Option<Timestamp>,
	/// The latest response from Tipalti
	#[serde(skip_serializing_if = "Option::is_none")]
	pub latest_tipalti_submission_response: Option<Value>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TeamPayoutStatus {
	/// The payout is open
	OPEN = 1,
	/// The payout has been paid out
	PAID = 2,
	/// The payout is pending completion
	PENDING = 3,
	/// The payout has been manually made
	MANUAL = 4,
	/// The payout has been cancelled
	CANCELLED = 5,
	/// The payout has been deferred
	DEFERRED = 6,
	/// The payout has been deferred internally
	DEFERRED_INTERNAL = 7,
	/// The payout is processing
	PROCESSING = 8,
	/// The payout has errored
	ERROR = 9,
	/// The payout has been rejected
	REJECTED = 10,
	/// The payout is under risk review
	RISK_REVIEW = 11,
	/// The payout has been submitted for completion
	SUBMITTED = 12,
	/// The payout is pending sufficient funds
	PENDING_FUNDS = 13,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
	/// The ID of the company
	pub id: Snowflake,
	/// The name of the company
	pub name: String,
}
