use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::auth::{LoginRequiredActionType, LoginSettings, LoginSource};
use crate::api::users::{AuthenticatorType, PartialUser};
use crate::common::id::{SkuId, UserId};
use crate::common::timestamp::Timestamp;

/// Method: `POST`
///
/// Does not require authentication
///
/// Retrieves an authentication token for the given credentials.
///
///
/// If this endpoint is requested with a valid authentication token, a success response will be returned irrespective of the request body.
///
///
/// When first logging into an account (without MFA enabled) from a new location, Discord may reject the login request and require the user to verify the login attempt via email or phone.
///
/// If logging in via email, the user will receive a link that redirects to the official Discord client with a verification token present in the URL's fragment (e.g. https://discord.com/authorize-ip#token=Wzg1Mjg5MjI5NzY2MTkwNjk5MywiMTI3LjAuMC4x8J+RvSJd.kdI8zppMIeTZsIBva3zZslaz_58).
///
/// If logging in via phone number, the request will fail with a [70007 JSON error code](https://ee085bcf.discord-userdoccers.pages.dev/topics/opcodes-and-status-codes#json-error-codes), and the user will receive a verification code via SMS. A verification token should be retrieved by [verifying the phone number](https://ee085bcf.discord-userdoccers.pages.dev/topics/phone-verification#verify-phone-number).
///
///
/// After [IP authorization](https://ee085bcf.discord-userdoccers.pages.dev/authentication#authorize-ip-address) with this verification token, the login request should be retried. A new CAPTCHA challenge should not be required.
///
/// If the user's account is suspended by Discord, the login request will fail with a 403 forbidden and a special error response body, similar to a success response:
///
///```json
/// {
///   "user_id": "852892297661906993",
///   "suspended_user_token": "ODUyODkyMjk3NjYxOTA2OTkz.Hcj0Nl.YEJSsjeq_vJKLpOofd5QMksqw32e"
/// }
/// ```
///
/// Suspended authentication tokens cannot be used as a regular authentication token. They can only be used to view account standing and appeal the account's suspension.
pub const LOGIN_ACCOUNT: &str = "/auth/login";

#[derive(Serialize, Deserialize)]
pub struct LoginAccountRequest {
	/// The user's email or E.164-formatted phone number
	pub login: String,
	/// The user's password (8-72 characters)
	pub password: ArrayString<72>,
	/// Whether to undelete a self-disabled or self-deleted account (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub undelete: Option<bool>,
	/// The source of the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<Option<LoginSource>>,
	/// The SKU ID of the gift code that initiated the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<SkuId>>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginAccountResponse {
	/// The ID of the user that was logged in
	pub user_id: UserId,
	/// The authentication token, if the login was completed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/// The user's partial settings, if the login was completed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_settings: Option<LoginSettings>,
	/// The required actions that must be completed before continuing to use Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_actions: Option<Vec<LoginRequiredActionType>>,
	/// A ticket to be used in the multi-factor authentication flow
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ticket: Option<String>,
	/// Whether multi-factor authentication is required to login (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mfa: Option<bool>,
	/// Whether the user has TOTP-based multi-factor authentication enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub totp: Option<bool>,
	/// Whether the user has SMS-based multi-factor authentication enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sms: Option<bool>,
	/// Whether backup codes can be used for multi-factor authentication
	#[serde(skip_serializing_if = "Option::is_none")]
	pub backup: Option<bool>,
	/// The stringified JSON public key credential request options challenge for WebAuthn
	#[serde(skip_serializing_if = "Option::is_none")]
	pub webauthn: Option<Option<String>>,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Verifies a multi-factor login and retrieves an authentication token using the specified [authenticator type](https://ee085bcf.discord-userdoccers.pages.dev/authentication#authenticator-type).
///
/// To verify using SMS MFA, you must first send a code to the user's phone number using the [Send MFA SMS](https://ee085bcf.discord-userdoccers.pages.dev/authentication#send-mfa-sms) endpoint.
///
/// If the user's account is suspended by Discord, the login request will fail with a 403 forbidden and a special error response body, similar to a success response:
///
///```json
/// { "suspended_user_token": "ODUyODkyMjk3NjYxOTA2OTkz.Hcj0Nl.YEJSsjeq_vJKLpOofd5QMksqw32e" }
/// ```
///
/// Suspended authentication tokens cannot be used as a regular authentication token. They can only be used to view account standing and appeal the account's suspension.
pub fn VERIFY_MFA_LOGIN(authenticator_type: &AuthenticatorType) -> String {
	format!("/auth/mfa/{:?}", authenticator_type)
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMfaLoginRequest {
	/// The MFA ticket received from the login request
	pub ticket: String,
	/// The MFA code (TOTP, SMS, backup, or WebAuthn) to be verified
	pub code: String,
	/// The source of the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<Option<LoginSource>>,
	/// The SKU ID of the gift code that initiated the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<SkuId>>,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMfaLoginResponse {
	/// The authentication token
	pub token: String,
	/// The user's partial settings
	pub user_settings: LoginSettings,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Generates a challenge to start the conditional UI flow for WebAuthn login.
///
/// If this endpoint is requested with a valid authentication token, an [authentication token](https://ee085bcf.discord-userdoccers.pages.dev/authentication#example-response-(completed)) will be returned irrespective of the request body.
pub const START_PASSWORDLESS_LOGIN: &str = "/auth/conditional/start";

#[derive(Serialize, Deserialize)]
pub struct StartPasswordlessLoginResponse {
	/// The WebAuthn login ticket
	pub ticket: String,
	/// The stringified JSON public key credential request options challenge for WebAuthn
	pub challenge: String,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Retrieves an authentication token for the given WebAuthn credentials.
///
/// If this endpoint is requested with a valid authentication token, a success response will be returned irrespective of the request body.
///
/// If the user's account is suspended by Discord, the login request will fail with a 403 forbidden and a special error response body, similar to a success response:
///
///```json
/// { "suspended_user_token": "ODUyODkyMjk3NjYxOTA2OTkz.Hcj0Nl.YEJSsjeq_vJKLpOofd5QMksqw32e" }
/// ```
///
/// Suspended authentication tokens cannot be used as a regular authentication token. They can only be used to view account standing and appeal the account's suspension.
pub const FINISH_PASSWORDLESS_LOGIN: &str = "/auth/conditional/finish";

#[derive(Serialize, Deserialize)]
pub struct FinishPasswordlessLoginRequest {
	/// The WebAuthn login ticket received from the Start Passwordless Login endpoint
	pub ticket: String,
	/// The stringified JSON public key credential response for WebAuthn
	pub credential: String,
	/// The source of the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<Option<LoginSource>>,
	/// The SKU ID of the gift code that initiated the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<SkuId>>,
}

#[derive(Serialize, Deserialize)]
pub struct FinishPasswordlessLoginResponse {
	/// The ID of the user that was logged in
	pub user_id: UserId,
	/// The authentication token
	pub token: String,
	/// The user's partial settings
	pub user_settings: LoginSettings,
	/// The required actions that must be completed before continuing to use Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_actions: Option<Vec<LoginRequiredActionType>>,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Authorizes the client's IP address for login. Returns a 204 empty response on success.
pub const AUTHORIZE_IP_ADDRESS: &str = "/auth/authorize-ip";

#[derive(Serialize, Deserialize)]
pub struct AuthorizeIpAddressRequest {
	/// The verification token received from the email or phone number verification process
	pub token: String,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Creates a handoff token to transfer the user's authentication state to another client.
pub const CREATE_HANDOFF_TOKEN: &str = "/auth/handoff";

#[derive(Serialize, Deserialize)]
pub struct CreateHandoffTokenRequest {
	/// A unique key to identify the handoff request (e.g. a random UUID)
	pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateHandoffTokenResponse {
	/// The handoff token to pass to the target
	///
	/// This is not an authentication token
	pub handoff_token: String,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Exchanges a handoff token for an authentication token. Handoff tokens can only be exchanged once and are only valid from the same IP address they were created from.
pub const EXCHANGE_HANDOFF_TOKEN: &str = "/auth/handoff/exchange";

#[derive(Serialize, Deserialize)]
pub struct ExchangeHandoffTokenRequest {
	/// The unique key used to create the handoff token
	pub key: String,
	/// The handoff token received from the Create Handoff Token endpoint
	pub handoff_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeHandoffTokenResponse {
	/// The authentication token
	pub token: String,
	/// The user that was authenticated
	pub user: PartialUser,
}

/// Method: `POST`
///
/// Does not require authentication
///
/// Creates a new account and retrieves an authentication token for the given credentials.
///
/// If this endpoint is requested with a valid authentication token, a success response will be returned irrespective of the request body.
///
/// Accounts should only be registered for a user to immediately use. Upon registration, the client should immediately connect to the Gateway using the retrieved authentication token.
/// Suspicious account creations may be flagged by Discord and require [additional verification steps](https://ee085bcf.discord-userdoccers.pages.dev/resources/user#required-action-type) or lead to immediate account termination.
pub const REGISTER_ACCOUNT: &str = "/auth/register";

#[derive(Serialize, Deserialize)]
pub struct JsonParams {
	/// The username to register (default random)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub username: Option<String>,
	/// The display name to register
	#[serde(skip_serializing_if = "Option::is_none")]
	pub global_name: Option<Option<String>>,
	/// The user's email address
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	/// The phone verification token received from the phone registration flow
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_token: Option<String>,
	/// The user's password (8-72 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password: Option<ArrayString<72>>,
	/// The user's date of birth
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_of_birth: Option<Timestamp>,
	/// The fingerprint to use for registration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fingerprint: Option<String>,
	/// The invite code that initiated the registration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite: Option<Option<String>>,
	/// The guild template code that initiated the registration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_template_code: Option<Option<String>>,
	/// The SKU ID of the gift code that initiated the registration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<SkuId>>,
	/// Whether the user agrees to Discord's Terms of Service and Privacy Policy
	///
	/// You can check whether the user needs to provide explicit consent by called the `GET_LOCATION_METADATA` endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub consent: Option<bool>,
	/// Whether the user explicitly opts-in/out to receiving promotional emails from Discord
	///
	/// You can check whether the user needs to provide explicit consent by called the `GET_LOCATION_METADATA` endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub promotional_email_opt_in: Option<bool>,
}
