use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::{quests::QuestRewardCode, users::PartialUser}, common::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct Entitlement {
	/// The ID of the entitlement
	pub id: Snowflake,
	/// The type of entitlement
	pub r#type: EntitlementType,
	/// The ID of the SKU granted
	pub sku_id: Snowflake,
	/// The ID of the application that owns the SKU
	pub application_id: Snowflake,
	/// The ID of the user that is granted access to the SKU
	pub user_id: Snowflake,
	/// The ID of the guild that is granted access to the SKU
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// The ID of the parent entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<Snowflake>,
	/// Whether the entitlement is deleted
	pub deleted: bool,
	/// For consumable items, whether the entitlement has been consumed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub consumed: Option<bool>,
	/// The IDs of the application branches granted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub branches: Option<Vec<Snowflake>>,
	/// When the entitlement validity period starts
	pub starts_at: Option<Timestamp>,
	/// When the entitlement validity period ends
	pub ends_at: Option<Timestamp>,
	/// The ID of the promotion the entitlement is from
	pub promotion_id: Option<Snowflake>,
	/// The ID of the subscription the entitlement is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_id: Option<Snowflake>,
	/// The flags for the gift code the entitlement is attached to
	pub gift_code_flags: GiftCodeFlags,
	/// The ID of the batch the gift code attached to the entitlement is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_batch_id: Option<Snowflake>,
	/// The ID of the user that gifted the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gifter_user_id: Option<Snowflake>,
	/// The style of the gift attached to the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_style: Option<GiftStyle>,
	/// The tenant fulfillment status of the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fulfillment_status: Option<EntitlementFulfillmentStatus>,
	/// When the entitlement was fulfilled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fulfilled_at: Option<Timestamp>,
	/// The special source type of the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source_type: Option<EntitlementSourceType>,
	/// Tenant metadata for the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tenant_metadata: Option<TenantMetadata>,
	/// The SKU granted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sku: Option<Value>,
	/// The subscription plan granted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_plan: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct TenantMetadata {
	/// Metadata about the quest rewards granted by the entitlement
	pub quest_rewards: QuestRewardsMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct QuestRewardsMetadata {
	/// The reward type of the entitlement
	pub tag: i64,
	/// The reward granted by the entitlement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reward_code: Option<QuestRewardCode>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntitlementType {
	/// Entitlement was purchased by a user
	PURCHASE = 1,
	/// Entitlement is for a premium (Nitro) subscription
	PREMIUM_SUBSCRIPTION = 2,
	/// Entitlement was gifted by a developer
	DEVELOPER_GIFT = 3,
	/// Entitlement was purchased by a developer in application test mode
	TEST_MODE_PURCHASE = 4,
	/// Entitlement was granted when the SKU was free
	FREE_PURCHASE = 5,
	/// Entitlement was gifted by another user
	USER_GIFT = 6,
	/// Entitlement was claimed for free through a premium subscription
	PREMIUM_PURCHASE = 7,
	/// Entitlement is for an application subscription
	APPLICATION_SUBSCRIPTION = 8,
	/// Entitlement was claimed for free by a Discord employee
	FREE_STAFF_PURCHASE = 9,
	/// Entitlement was granted as a reward for completing a quest
	QUEST_REWARD = 10,
	/// Entitlement is for a fractional premium subscription
	FRACTIONAL_REDEMPTION = 11,
	/// Entitlement was purchased with virtual currency (Orbs)
	VIRTUAL_CURRENCY_REDEMPTION = 12,
	/// Entitlement was purchased with premium guild subscriptions (boosts)
	GUILD_POWERUP = 13,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntitlementFulfillmentStatus {
	/// Unknown fulfillment status
	UNKNOWN = 0,
	/// Fulfillment is not needed for this entitlement
	FULFILLMENT_NOT_NEEDED = 1,
	/// Fulfillment is needed for this entitlement
	FULFILLMENT_NEEDED = 2,
	/// Entitlement has been fulfilled
	FULFILLED = 3,
	/// Fulfillment of the entitlement has failed
	FULFILLMENT_FAILED = 4,
	/// Unfulfillment is needed for this entitlement
	UNFULFILLMENT_NEEDED = 5,
	/// Entitlement has been unfulfilled
	UNFULFILLED = 6,
	/// Unfulfillment of the entitlement has failed
	UNFULFILLMENT_FAILED = 7,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntitlementSourceType {
	/// Entitlement was granted as a reward for completing a quest
	QUEST_REWARD = 1,
	/// Entitlement was gifted by a developer
	DEVELOPER_GIFT = 2,
	/// Entitlement was granted via an invoice
	INVOICE = 3,
	/// Entitlement was granted as part of a reverse trial
	REVERSE_TRIAL = 4,
	/// Entitlement was gifted by another user
	USER_GIFT = 5,
	/// Entitlement was granted via the guild powerups feature
	GUILD_POWERUP = 6,
	/// Entitlement was granted as part of a first-party promotion
	HOLIDAY_PROMOTION = 7,
	/// Unknown
	FRACTIONAL_PREMIUM_GIVEBACK = 8,
}

#[derive(Serialize, Deserialize)]
pub struct GiftCode {
	/// The gift code
	pub code: String,
	/// The ID of the SKU that the gift code grants
	pub sku_id: Snowflake,
	/// The ID of the application that owns the SKU
	pub application_id: Snowflake,
	/// The flags for the gift code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<GiftCodeFlags>,
	/// The number of times the gift code has been used
	pub uses: i64,
	/// The maximum number of times the gift code can be used
	pub max_uses: i64,
	/// Whether the gift code has been redeemed by the current user
	pub redeemed: bool,
	/// When the gift code expires
	pub expires_at: Option<Timestamp>,
	/// The ID of the batch the gift code is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_id: Option<Snowflake>,
	/// The IDs of the application branches granted by the gift code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entitlement_branches: Option<Vec<Snowflake>>,
	/// The style of the gift code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_style: Option<Option<GiftStyle>>,
	/// The user that created the gift code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// The store listing for the SKU the gift code grants
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store_listing: Option<Value>,
	/// The ID of the subscription plan the gift code grants
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_plan_id: Option<Snowflake>,
	/// The subscription plan the gift code grants
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_plan: Option<Value>,
	/// The subscription trial the gift code is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_trial: Option<Value>,
	/// The promotion the gift code is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub promotion: Option<Value>,
}

bitflags! {
	pub struct GiftCodeFlags: u64 {
		/// Gift requires a payment source to redeem
		const PAYMENT_SOURCE_REQUIRED = 1 << 0;
		/// Gift cannot be redeemed by users with existing premium subscriptions
		const EXISTING_SUBSCRIPTION_DISALLOWED = 1 << 1;
		/// Gift cannot be redeemed by the gifter
		const NOT_SELF_REDEEMABLE = 1 << 2;
		/// Gift is from a promotion
		const PROMOTION = 1 << 3;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GiftStyle {
	/// Snowglobe style gift code
	SNOWGLOBE = 1,
	/// Box style gift code
	BOX = 2,
	/// Cup style gift code
	CUP = 3,
	/// Standard box style gift code
	STANDARD_BOX = 4,
	/// Cake style gift code
	CAKE = 5,
	/// Chest style gift code
	CHEST = 6,
	/// Coffee style gift code
	COFFEE = 7,
	/// Seasonal standard box style gift code
	SEASONAL_STANDARD_BOX = 8,
	/// Seasonal cake style gift code
	SEASONAL_CAKE = 9,
	/// Seasonal chest style gift code
	SEASONAL_CHEST = 10,
	/// Seasonal coffee style gift code
	SEASONAL_COFFEE = 11,
	/// Nitroween standard style gift code
	NITROWEEN_STANDARD = 12,
}
