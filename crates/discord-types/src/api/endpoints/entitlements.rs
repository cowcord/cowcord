use serde::{Deserialize, Serialize};

use crate::api::entitlements::{
	Entitlement,
	EntitlementType,
	GatewayCheckoutContext,
	GiftCode,
	GiftStyle,
};
use crate::common::id::{
	ApplicationId,
	ChannelId,
	EntitlementId,
	GuildId,
	SkuId,
	SubscriptionId,
	UserId,
};
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects granted to the current user, both active and expired.
pub fn GET_USER_ENTITLEMENTS(query: &GetUserEntitlementsQueryParams) -> String {
	format!("/users/@me/entitlements{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetUserEntitlementsQueryParams {
	/// Whether to include SKU objects in the response (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_sku: Option<bool>,
	/// Whether to include application objects in the SKUs (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_application: Option<bool>,
	/// Whether ended entitlements should be omitted (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_ended: Option<bool>,
	/// The type of entitlement to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entitlement_type: Option<EntitlementType>,
}

pub type GetUserEntitlementsResponse = Vec<Entitlement>;

/// Method: `GET`
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects that the current user can gift.
pub fn GET_USER_GIFTABLE_ENTITLEMENTS(query: &GetUserGiftableEntitlementsQueryParams) -> String {
	format!("/users/@me/entitlements/gifts{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetUserGiftableEntitlementsQueryParams {
	/// The user's billing country code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country_code: Option<String>,
}

pub type GetUserGiftableEntitlementsResponse = Vec<Entitlement>;

/// Method: `GET`
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects granted to the given guild, both active and expired.
pub fn GET_GUILD_ENTITLEMENTS(
	query: &GetGuildEntitlementsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/entitlements{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildEntitlementsQueryParams {
	/// Whether to include SKU objects in the response (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_sku: Option<bool>,
	/// Whether to include application objects in the SKUs (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_application: Option<bool>,
	/// Whether ended entitlements should be omitted (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_ended: Option<bool>,
	/// Whether deleted entitlements should be omitted (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_deleted: Option<bool>,
	/// The type of entitlement to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entitlement_type: Option<EntitlementType>,
}

pub type GetGuildEntitlementsResponse = Vec<Entitlement>;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `applications.entitlements` scope
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects for the given application, both active and expired.
pub fn GET_APPLICATION_ENTITLEMENTS(
	query: &GetApplicationEntitlementsQueryParams,
	application_id: &ApplicationId,
) -> String {
	format!(
		"/applications/{}/entitlements{}",
		application_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationEntitlementsQueryParams {
	/// The ID of the user to look up entitlements for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<UserId>,
	/// The IDs of the SKUs to look up entitlements for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sku_ids: Option<Vec<SkuId>>,
	/// The ID of the guild to look up entitlements for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// Whether ended entitlements should be omitted (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_ended: Option<bool>,
	/// Whether deleted entitlements should be omitted (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclude_deleted: Option<bool>,
	/// Get entitlements before this entitlement ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<EntitlementId>,
	/// Get entitlements after this entitlement ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<EntitlementId>,
	/// Max number of entitlements to return
	/// (1-100, default 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

pub type GetApplicationEntitlementsResponse = Vec<Entitlement>;

/// Method: `GET`
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects granted to the current user for the given application.
pub fn GET_USER_APPLICATION_ENTITLEMENTS(
	query: &GetUserApplicationEntitlementsQueryParams,
	application_id: &ApplicationId,
) -> String {
	format!(
		"/users/@me/applications/{}/entitlements{}",
		application_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetUserApplicationEntitlementsQueryParams {
	/// The IDs of the SKUs to look up entitlements for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sku_ids: Option<Vec<SkuId>>,
	/// Whether consumed entitlements should be omitted (default true)
	pub exclude_consumed: bool,
}

pub type GetUserApplicationEntitlementsResponse = Vec<Entitlement>;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `applications.entitlements` scope
///
/// Returns an [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) object for the given application and entitlement ID.
pub fn GET_APPLICATION_ENTITLEMENT(
	application_id: &ApplicationId,
	entitlement_id: &EntitlementId,
) -> String {
	format!(
		"/applications/{}/entitlements/{}",
		application_id, entitlement_id
	)
}

pub type GetApplicationEntitlementResponse = Entitlement;

/// Method: `POST`
///
/// Creates a test entitlement to a given subscription SKU for a given guild or user.
///
/// Returns an [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) object on success. Fires an [Entitlement Create](https://docs.discord.food/topics/gateway-events#entitlement-create) Gateway event.
#[cfg(feature = "bot")]
pub fn CREATE_APPLICATION_ENTITLEMENT(application_id: &ApplicationId) -> String {
	format!("/applications/{}/entitlements", application_id)
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "bot")]
pub struct CreateApplicationEntitlementRequest {
	/// The ID of the SKU to grant the entitlement to
	pub sku_id: String,
	/// The ID of the guild or user to grant the entitlement to
	pub owner_id: String,
	/// The type of owner of the entitlement
	pub owner_type: EntitlementOwnerType,
}

/// Method: `POST`
///
/// Supports OAuth2 for authentication with the `applications.entitlements` scope
///
/// For one-time purchase consumable SKUs, marks a given entitlement for the user as consumed.
///
/// Returns a `204` empty response on success.
/// Fires an [Entitlement Update](https://docs.discord.food/topics/gateway-events#entitlement-update) Gateway event.
pub fn CONSUME_APPLICATION_ENTITLEMENT(
	application_id: &ApplicationId,
	entitlement_id: &EntitlementId,
) -> String {
	format!(
		"/applications/{}/entitlements/{}/consume",
		application_id, entitlement_id
	)
}

/// Method: `DELETE`
///
/// Supports OAuth2 for authentication with the `applications.entitlements` scope
///
/// Deletes a currently-active test entitlement.
///
/// Returns a `204` empty response on success.
/// Fires an [Entitlement Delete](https://docs.discord.food/topics/gateway-events#entitlement-delete) Gateway event.
pub fn DELETE_APPLICATION_ENTITLEMENT(
	application_id: &ApplicationId,
	entitlement_id: &EntitlementId,
) -> String {
	format!(
		"/applications/{}/entitlements/{}",
		application_id, entitlement_id
	)
}

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a [gift code](https://docs.discord.food/resources/entitlement#gift-code-object) object for the given code.
pub fn GET_GIFT_CODE(
	query: &GetGiftCodeQueryParams,
	code: &str,
) -> String {
	format!(
		"/entitlements/gift-codes/{}{}",
		code,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGiftCodeQueryParams {
	/// Whether to include the application object in the SKU
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_application: Option<bool>,
	/// Whether to include the subscription plan object in the SKU
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_subscription_plan: Option<bool>,
}

pub type GetGiftCodeResponse = GiftCode;

/// Method: `POST`
///
/// Redeems a gift code for the current user.
///
/// Returns an [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) object on success.
/// Fires an [Entitlement Create](https://docs.discord.food/topics/gateway-events#entitlement-create) and [Gift Code Update](https://docs.discord.food/topics/gateway-events#gift-code-update) Gateway event.
pub fn REDEEM_GIFT_CODE(code: &str) -> String {
	format!("/entitlements/gift-codes/{}/redeem", code)
}

#[derive(Serialize, Deserialize)]
pub struct RedeemGiftCodeRequest {
	/// The ID of the payment source to use for the gift code redemption
	#[serde(skip_serializing_if = "Option::is_none")]
	pub payment_source_id: Option<Option<String>>,
	/// The ID of the channel the gift code is being redeemed in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<Option<ChannelId>>,
	/// The context for the gateway checkout, if applicable
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gateway_checkout_context: Option<Option<GatewayCheckoutContext>>,
}

/// Method: `GET`
///
/// Returns a list of [gift code](https://docs.discord.food/resources/entitlement#gift-code-object) objects that the current user has created.
pub fn GET_USER_GIFT_CODES(query: &GetUserGiftCodesQueryParams) -> String {
	format!(
		"/users/@me/entitlements/gift-codes{}",
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetUserGiftCodesQueryParams {
	/// The IDs of the SKUs to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sku_ids: Option<Vec<SkuId>>,
	/// The ID of the subscription plan to filter by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_plan_id: Option<SubscriptionId>,
}

/// Method: `POST`
///
/// Creates a gift code.
///
/// Requires an eligible giftable entitlement.
///
/// Returns a [gift code](https://docs.discord.food/resources/entitlement#gift-code-object) object on success.
/// Fires a [Gift Code Create](https://docs.discord.food/topics/gateway-events#gift-code-create) Gateway event.
pub const CREATE_USER_GIFT_CODE: &str = "/users/@me/entitlements/gift-codes";

#[derive(Serialize, Deserialize)]
pub struct CreateUserGiftCodeRequest {
	/// The ID of the SKU to create a gift code for
	pub sku_id: SkuId,
	/// The ID of the subscription plan to create a gift code for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_plan_id: Option<SubscriptionId>,
	/// The style of the gift created
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_style: Option<GiftStyle>,
}

/// Method: `DELETE`
///
/// Revokes a gift code created by the current user.
///
/// Returns a `204` empty response on success.
pub fn REVOKE_USER_GIFT_CODE(code: &str) -> String {
	format!("/users/@me/entitlements/gift-codes/{}", code)
}
