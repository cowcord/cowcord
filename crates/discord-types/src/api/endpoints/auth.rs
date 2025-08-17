use serde::{Deserialize, Serialize};

use crate::api::users::PartialUser;

pub mod login {
	use arrayvec::ArrayString;
	use serde::{Deserialize, Serialize};

	use crate::api::auth::{
		AuthenticatorType,
		LoginRequiredActionType,
		LoginSettings,
		LoginSource,
	};
	use crate::common::id::{SkuId, UserId};

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
	/// Invalidates the given authentication session and unregisters the provided push notification token.
	pub const LOGOUT: &str = "/auth/logout";

	#[derive(Serialize, Deserialize)]
	pub struct LogoutRequest {
		/// The push notification provider to revoke
		#[serde(skip_serializing_if = "Option::is_none")]
		pub provider: Option<PushNotificationProvider>,
		/// The push notification token to unregister
		#[serde(skip_serializing_if = "Option::is_none")]
		pub token: Option<String>,
		/// The VOIP push notification provider to revoke the token from
		///
		/// VOIP-specific push notification tokens are only used with PushKit on iOS.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub voip_provider: Option<String>,
		/// The VOIP push notification token to unregister
		///
		/// VOIP-specific push notification tokens are only used with PushKit on iOS.
		#[serde(skip_serializing_if = "Option::is_none")]
		pub voip_token: Option<String>,
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

pub mod register {
	use arrayvec::ArrayString;
	use serde::{Deserialize, Serialize};

	use crate::api::auth::{PasswordStrength, PromotionalEmailMetadata};
	use crate::common::id::SkuId;
	use crate::common::timestamp::Timestamp;
	use crate::utils::url::ToStringQuery;

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
	pub struct RegisterAccountRequest {
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

	#[derive(Serialize, Deserialize)]
	pub struct RegisterAccountResponse {
		/// The authentication token
		pub token: String,
		/// Whether the user should be shown the joined guild's member verification form
		#[serde(skip_serializing_if = "Option::is_none")]
		pub show_verification_form: Option<bool>,
	}

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Sends a verification code to the user's phone number to register an account.
	///
	/// The verification code should be first used to [verify the phone number](https://ee085bcf.discord-userdoccers.pages.dev/topics/phone-verification#verify-phone-number) before [completing the registration](https://ee085bcf.discord-userdoccers.pages.dev/authentication#register-account).
	///
	/// Returns a 204 empty response on success.
	pub const REGISTER_ACCOUNT_WITH_PHONE_NUMBER: &str = "/auth/register/phone";

	#[derive(Serialize, Deserialize)]
	pub struct RegisterAccountWithPhoneNumberRequest {
		/// The user's E.164-formatted phone number
		pub phone: String,
	}

	/// Method: `GET`
	///
	/// Does not require authentication
	///
	/// Returns the location metadata for the user's IP address.
	pub const GET_LOCATION_METADATA: &str = "/auth/location-metadata";

	#[derive(Serialize, Deserialize)]
	pub struct GetLocationMetadataResponse {
		/// The ISO 3166-1 alpha-2 country code of the user's IP address
		pub country_code: String,
		/// Whether the user must explicitly agree to Discord's Terms of Service and Privacy Policy in order to register
		pub consent_required: bool,
		/// Promotional email consent metadata
		pub promotional_email_opt_in: PromotionalEmailMetadata,
	}

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Validates the strength of a password and returns a score based on its complexity.
	/// This is used to ensure that the user's password meets Discord's security requirements.
	pub const GET_PASSWORD_STRENGTH: &str = "/auth/password/validate";

	#[derive(Serialize, Deserialize)]
	pub struct GetPasswordStrengthRequest {
		/// The password to validate (8-72 characters)
		pub password: ArrayString<72>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct GetPasswordStrengthResponse {
		/// Whether the password is valid according to Discord's security requirements
		pub valid: bool,
		/// The password strength score (0-4)
		pub password_strength: PasswordStrength,
	}

	/// Method: `GET`
	///
	/// Does not require authentication
	///
	/// Returns a suggested unique username string for the user to register with.
	pub fn GET_UNIQUE_USERNAME_SUGGESTIONS(
		query: &GetUniqueUsernameSuggestionsQueryParams
	) -> String {
		format!(
			"/unique-username/username-suggestions-unauthed?{}",
			query.to_string_query()
		)
	}

	#[derive(Serialize, Deserialize)]
	pub struct GetUniqueUsernameSuggestionsQueryParams {
		/// The global name to base the username suggestions on (default random)
		#[serde(skip_serializing_if = "Option::is_none")]
		pub global_name: Option<String>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct GetUniqueUsernameSuggestionsResponse {
		/// The suggested username
		pub username: String,
	}

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Checks whether a unique username is available for the user to register with.
	///
	/// See the [Usernames and Nicknames section](https://ee085bcf.discord-userdoccers.pages.dev/resources/user#usernames-and-nicknames) for more information on username restrictions.
	pub const GET_UNIQUE_USERNAME_ELIGIBILITY: &str = "/unique-username/username-attempt-unauthed";

	#[derive(Serialize, Deserialize)]
	pub struct GetUniqueUsernameEligibilityRequest {
		/// The username to check
		pub username: String,
	}

	#[derive(Serialize, Deserialize)]
	pub struct GetUniqueUsernameEligibilityResponse {
		/// Whether the username is taken
		pub taken: Option<bool>,
	}
}

pub mod recovery {
	use arrayvec::ArrayString;
	use serde::{Deserialize, Serialize};

	use crate::api::users::AuthenticatorType;
	use crate::common::id::UserId;

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Initiates the password reset process for the given email or phone number.
	///
	/// If providing an email, the user will receive a link that redirects to the official Discord client with a verification token present in the URL's fragment (e.g. https://discord.com/reset#token=eyJpZCI6ODUyODkyMjk3NjYxOTA2OTkzLCJlbWFpbCI6Im5lbGx5QGRpc2NvcmRhcHAuY29tIn0.Z6pQDg.pKCZBaaiodflO6FZhdttm6B_z74).
	///
	/// If providing a phone number, the request will fail with a [70007 JSON error code](https://ee085bcf.discord-userdoccers.pages.dev/topics/opcodes-and-status-codes#json-error-codes), and the user will receive a verification code via SMS.
	/// A verification token should be retrieved by [verifying the phone number](https://ee085bcf.discord-userdoccers.pages.dev/topics/phone-verification#verify-phone-number).
	///
	/// If the user is ineligible to reset their password via phone number, the phone number verification request will fail with a [70009 JSON error code](https://ee085bcf.discord-userdoccers.pages.dev/topics/opcodes-and-status-codes#json-error-codes) and the user will receive a link to reset their password via email.
	///
	/// Returns a 204 empty response on success.
	pub const FORGOT_PASSWORD: &str = "/auth/forgot";

	#[derive(Serialize, Deserialize)]
	pub struct ForgotPasswordRequest {
		/// The user's email or E.164-formatted phone number
		pub login: String,
	}

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Resets the user's password and retrieves an authentication token.
	///
	/// When attempting to reset the password of a user with multi-factor authentication enabled, the request will return a response similar to the [MFA Required response](https://ee085bcf.discord-userdoccers.pages.dev/authentication#mfa-verification).
	///
	/// To complete the password reset, the client must retry the request with the ticket and code parameters specified.
	pub const RESET_PASSWORD: &str = "/auth/reset";

	#[derive(Serialize, Deserialize)]
	pub struct ResetPasswordRequest {
		/// The password reset token received from the email or phone number verification
		pub token: String,
		/// The user's new password (8-72 characters)
		pub password: ArrayString<72>,
		/// The source path the password reset was initiated from (e.g. /reset )
		#[serde(skip_serializing_if = "Option::is_none")]
		pub source: Option<String>,
		/// The authenticator type to use for MFA verification
		#[serde(skip_serializing_if = "Option::is_none")]
		pub method: Option<AuthenticatorType>,
		/// The MFA ticket received from the previous request
		#[serde(skip_serializing_if = "Option::is_none")]
		pub ticket: Option<String>,
		/// The MFA code (TOTP, SMS, backup, or WebAuthn) to be verified
		#[serde(skip_serializing_if = "Option::is_none")]
		pub code: Option<String>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct ResetPasswordResponse {
		/// The authentication token, if the password reset was completed
		#[serde(skip_serializing_if = "Option::is_none")]
		pub token: Option<String>,
		/// The ID of the user whose password was reset, if MFA verification is required
		#[serde(skip_serializing_if = "Option::is_none")]
		pub user_id: Option<UserId>,
		/// A ticket to be used when retrying the request with multi-factor authentication
		#[serde(skip_serializing_if = "Option::is_none")]
		pub ticket: Option<String>,
		/// Whether multi-factor authentication is required to reset the password (default false)
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
	/// Recovers an account after account takeover. This endpoint will:
	///
	/// - Invalidate the revert token used
	///
	/// - Invalidate all authorization tokens and active Gateway sessions associated with the account
	///
	/// - Set the account's email back to the original
	///
	/// - Set the account's password to the one provided
	///
	/// - Remove MFA from the account
	///
	/// - Remove and blacklist all phone numbers previously associated with the account
	///
	/// This process will fail if the account was deleted beforehand (e.g. by anti-spam or a Discord employee) or the original email was assigned to a different account.
	pub const REVERT_ACCOUNT: &str = "/auth/revert";

	#[derive(Serialize, Deserialize)]
	pub struct RevertAccountRequest {
		/// The revert token from the recovery email
		pub token: String,
		/// The new password for the account
		pub password: ArrayString<72>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct RevertAccountResponse {
		/// The account's restored email
		pub email: String,
	}
}

pub mod mfa {
	use serde::{Deserialize, Serialize};

	use crate::api::auth::{AuthenticatorType, MfaMethod};

	#[derive(Serialize, Deserialize)]
	pub struct MfaRequiredResponse {
		/// A message saying that multi-factor authentication is required for the operation
		pub message: String,
		/// An error code (will always be 60003 )
		pub code: i64,
		/// The multi-factor authentication verification request
		pub mfa: MfaVerificationRequest,
	}

	#[derive(Serialize, Deserialize)]
	pub struct MfaVerificationRequest {
		/// The MFA ticket
		pub ticket: String,
		/// An array of MFA methods that can be used to verify the user's identity
		pub methods: Vec<MfaMethod>,
	}

	/// Method: `POST`
	///
	/// Verifies a user's identity using multi-factor authentication. On success, returns a cookie that can be used to bypass MFA for the next 5 minutes.
	pub const VERIFY_MFA: &str = "/mfa/finish";

	#[derive(Serialize, Deserialize)]
	pub struct VerifyMfaRequest {
		/// The MFA ticket received from the MFA required response
		pub ticket: String,
		/// The authenticator type used to verify the user's identity
		pub mfa_type: AuthenticatorType,
		/// The MFA data (TOTP, SMS, backup, WebAuthn, or password) to be verified
		pub data: String,
	}

	#[derive(Serialize, Deserialize)]
	pub struct VerifyMfaResponse {
		/// The MFA verification JWT
		/// (expires after 5 minutes)
		pub token: String,
	}

	/// Method: `POST`
	///
	/// Does not require authentication
	///
	/// Sends a multi-factor authentication code to the user's phone number for verification.
	pub const SEND_MFA_SMS: &str = "/auth/mfa/sms/send";

	#[derive(Serialize, Deserialize)]
	pub struct SendMfaSmsRequest {
		/// The MFA ticket received from the login request or MFA required response
		pub ticket: String,
	}

	#[derive(Serialize, Deserialize)]
	pub struct SendMfaSmsResponse {
		/// The redacted phone number the SMS was sent to
		pub phone: String,
	}
}
