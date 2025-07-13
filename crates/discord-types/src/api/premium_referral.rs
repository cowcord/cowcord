use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::users::PartialUser, common::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct PremiumReferral {
	/// The ID of the referral
	pub id: Snowflake,
	/// The user who the referral was sent to
	pub user_id: Snowflake,
	/// The trial ID associated with the referral
	pub trial_id: Snowflake,
	/// The subscription trial associated with the referral
	pub subscription_trial: Value,
	/// The expiry date of the referral link
	pub expires_at: Timestamp,
	/// The ID of the user who created the referral
	pub referrer_id: Snowflake,
	/// The user who created the referral
	pub referrer: PartialUser,
	/// When the referral was redeemed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redeemed_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct PremiumReferralEligibility {
	/// The amount of referrals remaining
	pub referrals_remaining: i64,
	/// The IDs of users that have been referred
	pub sent_user_ids: Vec<Snowflake>,
	/// When the referral count will refresh
	pub refresh_at: Option<Timestamp>,
	/// Whether the user has friends that are eligible for a referral
	pub has_eligible_friends: bool,
	/// The redemption status of each referred user ID
	pub recipient_status: HashMap<Snowflake, PremiumReferralRecipientStatus>,
	/// Whether the user is eligible for a personal discount upon referral redemption
	pub is_eligible_for_incentive: bool,
	/// Whether the user will receive an incentivized discount on their next Nitro purchase
	pub is_qualified_for_incentive: bool,
	/// The incentive status of the user
	pub referral_incentive_status: PremiumReferralIncentiveStatus,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumReferralRecipientStatus {
	/// The recipient has redeemed the referral
	REDEEMED = 1,
	/// The recipient has yet to redeem the referral
	PENDING = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumReferralIncentiveStatus {
	/// The user is not eligible for an incentive
	NOT_ELIGIBLE = 0,
	/// The user is eligible for an incentive
	ELIGIBLE = 1,
	/// The user will receive an incentivized discount on their next Nitro purchase
	QUALIFIED = 2,
	/// The user is on cooldown and cannot receive an incentive
	COOLDOWN = 3,
	/// The user has not applied their incentive yet
	UNAPPLIED = 4,
}

