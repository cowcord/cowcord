use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::users::Theme;
use crate::common::locale::Locale;

/// Where a login is initiated from outside of the normal login flow.
#[derive(Serialize, Deserialize)]
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

/// A partial settings object to bootstrap the client with.
#[derive(Serialize, Deserialize)]
pub struct LoginSettings {
	/// The language option chosen by the user
	pub locale: Locale,
	/// The client theme selected by the user (only dark or light are returned)
	pub theme: Theme,
}

/// Actions the user must complete after a successful login.
#[derive(Serialize, Deserialize)]
pub enum LoginRequiredActionType {
	/// The user must change their password to meet Discord's new password requirements
	update_password,
}

#[derive(Serialize, Deserialize)]
pub struct PromotionalEmailMetadata {
	/// Whether the user must explicitly agree to receive promotional emails from Discord
	pub required: bool,
	/// Whether the promotional email consent checkbox should be pre-checked, if explicit consent is required
	pub pre_checked: bool,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PasswordStrength {
	VERY_WEAK = 0,
	WEAK = 1,
	MEDIUM = 2,
	STRONG = 3,
	VERY_STRONG = 4,
}

#[derive(Serialize, Deserialize)]
pub struct MfaMethod {
	/// The type of MFA method that can be used to verify the user's identity
	pub r#type: AuthenticatorType,
	/// The stringified JSON public key credential request options challenge for WebAuthn
	#[serde(skip_serializing_if = "Option::is_none")]
	pub challenge: Option<String>,
	/// Whether backup codes can be used in addition to TOTP codes
	#[serde(skip_serializing_if = "Option::is_none")]
	pub backup_codes_allowed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorType {
	/// Verification using a TOTP code or backup code
	totp,
	/// Verification using a code sent to the user's phone number via SMS
	sms,
	/// Verification using a backup code
	backup,
	/// Verification using a WebAuthn device
	webauthn,
	/// Verification using the user's password
	password,
}
