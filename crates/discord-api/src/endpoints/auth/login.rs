use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::token::Token;

pub const LOGIN_ACCOUNT: &str = "/auth/login";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAccountRequest {
	/// The user's email or E.164-formatted phone number
	pub login: String,
	/// The user's password (8-72 characters)
	pub password: String,
	/// Whether to undelete a self-disabled or self-deleted account (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub undelete: Option<bool>,
	/// The source of the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<Option<LoginSource>>,
	/// The SKU ID of the gift code that initiated the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAccountResponse {
	/// The ID of the user that was logged in
	// pub user_id: UserId,
	pub user_id: String,
	/// The authentication token, if the login was completed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<Token>,
	/// The user's partial settings, if the login was completed
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_settings: Option<LoginSettings>,
	/// The required actions that must be completed before continuing to use Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_actions: Option<Vec<LoginRequiredActionType>>,
	/// A ticket to be used in the multi-factor authentication flow
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ticket: Option<String>,
	/// The instance ID to be used in the multi-factor authentication flow
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_instance_id: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAccountSuspendedResponse {
	/// The ID of the user that was logged in
	// pub user_id: UserId,
	pub user_id: String,
	/// If the user is suspended this will be returned
	///
	/// It is not a regular token that you pass into `Authorization` header.
	/// Instead, for the endpoints it can be used for, its passed in as a json param `token`
	/// on any endpoints under `/safety-hub/suspended`
	pub suspended_user_token: Token,
}

// todo: move to `types` dir, i dont think its endpoint specific
/// A partial settings object to bootstrap the client with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginSettings {
	/// The language option chosen by the user
	pub locale: String,
	/// The client theme selected by the user (only dark or light are returned)
	pub theme: String,
}

/// Where a login is initiated from outside of the normal login flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginSource {
	/// Login request initiated from a gift code
	gift,
	/// Login request initiated from a guild template
	guild_template,
	/// Login request initiated from a guild invite
	guild_invite,
	/// Login request initiated from a group DM invite
	dm_invite,
	/// Login request initiated from a friend invite
	friend_invite,
	/// Login request initiated from a role subscription redirect
	role_subscription,
	/// Login request initiated from a role subscription settings redirect
	role_subscription_setting,
}

/// Actions the user must complete after a successful login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginRequiredActionType {
	/// The user must change their password to meet Discord's new password requirements
	update_password,
}

pub const REMOTE_AUTH_TICKET_EXCHANGE: &str = "/users/@me/remote-auth/login";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuthTicketExchangeRequest {
	/// The ticket obtained from the remote authentication flow
	pub ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuthTicketExchangeResponse {
	/// The authentication token encrypted with the client's public key
	pub encrypted_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticatorType {
	/// Verification using a TOTP code or backup code
	totp,
	/// Verification using a code sent to the user's phone number via SMS
	sms,
	/// Verification using a backup code
	backup,
	/// Verification using a WebAuthn device
	webauthn { credential_request_options: String },
	/// Verification using the user's password
	password,
}

impl Display for AuthenticatorType {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self {
			| AuthenticatorType::totp => write!(f, "Authenticator App"),
			| AuthenticatorType::sms => write!(f, "SMS"),
			| AuthenticatorType::backup => write!(f, "Backup Code"),
			| AuthenticatorType::webauthn {
				credential_request_options: _,
			} => write!(f, "Security Key"),
			| AuthenticatorType::password => write!(f, "Password"),
		}
	}
}

pub fn VERIFY_MFA_LOGIN(authenticator_type: &AuthenticatorType) -> String {
	format!("/auth/mfa/{authenticator_type:?}")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMfaLoginRequest {
	/// The MFA ticket received from the login request
	pub ticket: String,
	/// The login instance ID received from the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_instance_id: Option<String>,
	/// The MFA code (TOTP, SMS, backup, or WebAuthn) to be verified
	///
	/// if WebAuthn, this should be the stringified JSON of the [public key credential response](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)
	pub code: String,
	/// The source of the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_source: Option<Option<String>>,
	/// The SKU ID of the gift code that initiated the login request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_code_sku_id: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMfaLoginResponse {
	/// The authentication token
	pub token: String,
	/// The user's partial settings
	pub user_settings: LoginSettings,
}
