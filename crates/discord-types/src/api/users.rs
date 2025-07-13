use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::{emoji::Emoji, family_center::LinkedUser}, common::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct User {
	/// The ID of the user
	pub id: Snowflake,
	/// The user's username, may be unique across the platform (2-32 characters)
	pub username: String,
	/// The user's stringified 4-digit Discord tag
	pub discriminator: String,
	/// The user's display name (1-32 characters)
	pub global_name: Option<String>,
	/// The user's avatar hash
	pub avatar: Option<String>,
	/// The user's avatar decoration
	pub avatar_decoration_data: Option<AvatarDecorationData>,
	/// The user's equipped collectibles
	pub collectibles: Option<Collectibles>,
	/// The primary guild of the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_guild: Option<Option<PrimaryGuild>>,
	/// The linked users connected to the account via Family Center
	pub linked_users: Vec<LinkedUser>,
	/// Whether the user is a bot account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<bool>,
	/// Whether the user is an official Discord System user (part of the urgent message system)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system: Option<bool>,
	/// Whether the user has multi-factor authentication enabled on their account
	pub mfa_enabled: bool,
	/// Whether the user is allowed to see NSFW content, null if not yet known
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nsfw_allowed: Option<Option<bool>>,
	/// The age verification status of the user
	pub age_verification_status: AgeVerificationStatus,
	/// The user's pronouns (max 40 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pronouns: Option<String>,
	/// The user's bio (max 190 characters)
	pub bio: String,
	/// The user's banner hash
	pub banner: Option<String>,
	/// The user's banner color encoded as an integer representation of a hexadecimal color code
	pub accent_color: Option<i64>,
	/// The language option chosen by the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locale: Option<Locale>,
	/// Whether the email on this account has been verified
	pub verified: bool,
	/// The user's email address
	pub email: Option<String>,
	/// The user's E.164-formatted phone number
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<Option<String>>,
	/// Whether the user is subscribed to Nitro
	#[deprecated]
	pub premium: bool,
	/// The type of premium (Nitro) subscription on a user's account
	pub premium_type: PremiumType,
	/// The ID of the user's personal, non-employee user account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub personal_connection_id: Option<Snowflake>,
	/// The flags on a user's account
	pub flags: UserFlags,
	/// The public flags on a user's account
	pub public_flags: UserFlags,
	/// The purchased flags on a user's account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub purchased_flags: Option<PurchasedFlags>,
	/// The premium usage flags on a user's account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub premium_usage_flags: Option<PremiumUsageFlags>,
	/// Whether the user has used the desktop client before
	#[serde(skip_serializing_if = "Option::is_none")]
	pub desktop: Option<bool>,
	/// Whether the user has used the mobile client before
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<bool>,
	/// Whether the user's email has failed to deliver and is no longer valid
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_bounced_email: Option<bool>,
	/// The types of multi-factor authenticators the user has enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authenticator_types: Option<Vec<AuthenticatorType>>,
}

#[derive(Serialize, Deserialize)]
pub struct PartialUser {
	/// The ID of the user
	pub id: Snowflake,
	/// The user's username, may be unique across the platform (2-32 characters)
	pub username: String,
	/// The user's stringified 4-digit Discord tag
	pub discriminator: String,
	/// The user's display name (1-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub global_name: Option<Option<String>>,
	/// The user's avatar hash
	pub avatar: Option<String>,
	/// The user's avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_data: Option<Option<AvatarDecorationData>>,
	/// The user's equipped collectibles
	pub collectibles: Option<Collectibles>,
	/// The primary guild of the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_guild: Option<Option<PrimaryGuild>>,
	/// Whether the user is a bot account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<bool>,
	/// Whether the user is an official Discord System user (part of the urgent message system)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system: Option<bool>,
	/// The user's banner hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<String>>,
	/// The user's banner color encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accent_color: Option<Option<i64>>,
	/// The public flags on a user's account
	#[serde(skip_serializing_if = "Option::is_none")]
	pub public_flags: Option<UserFlags>,
}

#[derive(Serialize, Deserialize)]
pub struct PrimaryGuild {
	/// Whether the user is displaying their guild tag
	pub identity_enabled: Option<bool>,
	/// The ID of the guild
	pub identity_guild_id: Option<Snowflake>,
	/// The user's guild tag (max 4 characters)
	pub tag: Option<String>,
	/// The guild tag badge hash
	pub badge: Option<String>,
}

bitflags! {
	pub struct UserFlags: u64 {
		/// Discord Staff
		const STAFF = 1 << 0;
		/// Partnered Server Owner
		const PARTNER = 1 << 1;
		/// HypeSquad Events
		const HYPESQUAD = 1 << 2;
		/// Level 1 Discord Bug Hunter
		const BUG_HUNTER_LEVEL_1 = 1 << 3;
		/// SMS enabled as a multi-factor authentication backup
		const MFA_SMS = 1 << 4;
		/// User has dismissed the current premium (Nitro) promotion
		const PREMIUM_PROMO_DISMISSED = 1 << 5;
		/// HypeSquad Bravery
		const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
		/// HypeSquad Brilliance
		const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
		/// HypeSquad Balance
		const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
		/// Early Premium (Nitro) Supporter
		const PREMIUM_EARLY_SUPPORTER = 1 << 9;
		/// User is a Team
		const TEAM_PSEUDO_USER = 1 << 10;
		/// User is registered on Discord's HubSpot customer platform, used for official Discord programs (e.g. partner)
		const IS_HUBSPOT_CONTACT = 1 << 11;
		/// User has unread urgent system messages; an urgent message is one sent from Trust and Safety
		const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
		/// Level 2 Discord Bug Hunter
		const BUG_HUNTER_LEVEL_2 = 1 << 14;
		/// User is scheduled for deletion for being under the minimum required age
		const UNDERAGE_DELETED = 1 << 15;
		/// Verified Bot
		const VERIFIED_BOT = 1 << 16;
		/// Early Verified Bot Developer
		const VERIFIED_DEVELOPER = 1 << 17;
		/// Moderator Programs Alumni
		const CERTIFIED_MODERATOR = 1 << 18;
		/// Bot uses only HTTP interactions and is shown in the online member list
		const BOT_HTTP_INTERACTIONS = 1 << 19;
		/// User is marked as a spammer and has their messages collapsed in the UI
		const SPAMMER = 1 << 20;
		/// Active Developer
		const ACTIVE_DEVELOPER = 1 << 22;
		/// User is a provisional account used with the social layer integration
		const PROVISIONAL_ACCOUNT = 1 << 23;
		/// User has their global ratelimit raised to 1,200 requests per second
		const HIGH_GLOBAL_RATE_LIMIT = 1 << 33;
		/// User's account is deleted
		const DELETED = 1 << 34;
		/// User's account is disabled for suspicious activity and must reset their password to regain access
		const DISABLED_SUSPICIOUS_ACTIVITY = 1 << 35;
		/// User deleted their own account
		const SELF_DELETED = 1 << 36;
		/// User has a premium (Nitro) custom discriminator
		const PREMIUM_DISCRIMINATOR = 1 << 37;
		/// User has used the desktop client
		const USED_DESKTOP_CLIENT = 1 << 38;
		/// User has used the web client
		const USED_WEB_CLIENT = 1 << 39;
		/// User has used the mobile client
		const USED_MOBILE_CLIENT = 1 << 40;
		/// User's account is disabled
		const DISABLED = 1 << 41;
		/// User has started at least one Gateway session and is now eligible to send messages
		const HAS_SESSION_STARTED = 1 << 43;
		/// User is quarantined and cannot create DMs or accept invites
		const QUARANTINED = 1 << 44;
		/// User is eligible for early access to unique usernames
		const PREMIUM_ELIGIBLE_FOR_UNIQUE_USERNAME = 1 << 47;
		/// User is a collaborator and is considered staff
		const COLLABORATOR = 1 << 50;
		/// User is a restricted collaborator and is considered staff
		const RESTRICTED_COLLABORATOR = 1 << 51;
	}
}

bitflags! {
    /// Purchased flags denote what premium items a user has ever purchased. Visit the [Nitro](https://discord.com/nitro) page to learn more about the premium plans currently offered.
	pub struct PurchasedFlags: u64 {
		/// User has purchased Nitro classic
		const NITRO_CLASSIC = 1 << 0;
		/// User has purchased regular Nitro
		const NITRO = 1 << 1;
		/// User has purchased a guild boost
		const GUILD_BOOST = 1 << 2;
		/// User has purchased Nitro basic
		const NITRO_BASIC = 1 << 3;
		/// User has a reverse trial active
		const ON_REVERSE_TRIAL = 1 << 4;
	}
}

bitflags! {
	/// Premium usage flags denote what premium (Nitro) features a user has utilized.
	pub struct PremiumUsageFlags: u64 {
		/// User has utilized premium discriminators
		const PREMIUM_DISCRIMINATOR = 1 << 0;
		/// User has utilized animated avatars
		const ANIMATED_AVATAR = 1 << 1;
		/// User has utilized profile banners
		const PROFILE_BANNER = 1 << 2;
	}
}

/// Premium types denote the level of premium a user has. Visit the [Nitro](https://discord.com/nitro) page to learn more about the premium plans currently offered.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumType {
	/// No Nitro
	#[deprecated]
	NONE = 0,
	/// Nitro Classic
	TIER_1 = 1,
	/// Nitro
	TIER_2 = 2,
	/// Nitro Basic
	TIER_3 = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AgeVerificationStatus {
	/// User has not verified their age
	UNVERIFIED = 1,
	/// User is a verified teenager
	VERIFIED_TEEN = 2,
	/// User is a verified adult
	VERIFIED_ADULT = 3,
}

/// Denotes an action Discord requires the user to take before they can continue using the platform. In some cases, multiple actions may be required, and the user must complete all of them before they can continue using Discord.
#[derive(Serialize, Deserialize)]
pub enum RequiredActionType {
	/// The user must re-indicate their agreement of Discord's terms of service and privacy policy; this does not limit the user from using Discord
	AGREEMENTS,
	/// The user must add and verify an email address to their account
	REQUIRE_VERIFIED_EMAIL,
	/// The user must reverify their existing email address
	REQUIRE_REVERIFIED_EMAIL,
	/// The user must add a phone number to their account
	REQUIRE_VERIFIED_PHONE,
	/// The user must reverify their existing phone number
	REQUIRE_REVERIFIED_PHONE,
	/// The user must add and verify an email address to their account or add a phone number to their account
	REQUIRE_VERIFIED_EMAIL_OR_VERIFIED_PHONE,
	/// The user must reverify their existing email address or add a phone number to their account
	REQUIRE_REVERIFIED_EMAIL_OR_VERIFIED_PHONE,
	/// The user must add and verify an email address to their account or reverify their existing phone number
	REQUIRE_VERIFIED_EMAIL_OR_REVERIFIED_PHONE,
	/// The user must reverify their existing email address or reverify their existing phone number
	REQUIRE_REVERIFIED_EMAIL_OR_REVERIFIED_PHONE,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarDecorationData {
	/// The avatar decoration hash
	pub asset: String,
	/// The ID of the avatar decoration's SKU
	pub sku_id: Snowflake,
	/// Unix timestamp of when the current avatar decoration expires
	pub expires_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Collectibles {
	/// The user's nameplate
	pub nameplate: NameplateData,
}

#[derive(Serialize, Deserialize)]
pub struct NameplateData {
	/// The nameplate asset path
	pub asset: String,
	/// The ID of the nameplate's SKU
	pub sku_id: Snowflake,
	/// The nameplate's accessibility description
	pub label: String,
	/// The nameplate's color palette
	pub palette: String,
	/// Unix timestamp of when the current nameplate expires
	pub expires_at: Option<i64>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NameplateColorPalette {
	none,
	crimson,
	berry,
	sky,
	teal,
	forest,
	bubble_gum,
	violet,
	cobalt,
	clover,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileMetadata {
	/// The guild ID this profile applies to, if it is a guild profile
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// The user's pronouns (max 40 characters)
	pub pronouns: String,
	/// The user's bio (max 190 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<String>,
	/// The user's banner hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<String>>,
	/// The user's banner color encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accent_color: Option<Option<i64>>,
	/// The user's two theme colors encoded as an array of integers representing hexadecimal color codes
	#[serde(skip_serializing_if = "Option::is_none")]
	pub theme_colors: Option<Option<(i64, i64)>>,
	/// The user's profile popout animation particle type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub popout_animation_particle_type: Option<Option<Snowflake>>,
	/// The user's profile emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Option<Emoji>>,
	/// The user's profile effect
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile_effect: Option<Option<ProfileEffect>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileEffect {
	/// The ID of the profile effect
	pub id: Snowflake,
	/// Unix timestamp of when the current profile effect expires
	pub expires_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Authenticator {
	/// The ID of the authenticator
	pub id: String,
	/// The type of authenticator
	pub r#type: AuthenticatorType,
	/// The name of the authenticator
	pub name: String,
}

/// Authenticator types represent enabled multi-factor authentication methods. See the [MFA verification documentation](https://docs.discord.food/authentication#mfa-verification) for more information.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AuthenticatorType {
	/// WebAuthn credentials
	WEBAUTHN = 1,
	/// Time-based One-Time Password code
	TOTP = 2,
	/// SMS code
	SMS = 3,
}

#[derive(Serialize, Deserialize)]
pub struct BackupCode {
	/// The ID of the user
	pub user_id: Snowflake,
	/// The backup code
	pub code: String,
	/// Whether the backup code has been used
	pub consumed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Harvest {
	/// The ID of the harvest
	pub harvest_id: Snowflake,
	/// The ID of the user being harvested
	pub user_id: Snowflake,
	/// The email the harvest will be sent to
	pub email: String,
	/// The state of the harvest
	pub state: String,
	/// The status of the harvest
	pub status: i64,
	/// When the harvest was created
	pub created_at: Timestamp,
	/// When the harvest was completed
	pub completed_at: Option<Timestamp>,
	/// When the harvest was last polled
	pub polled_at: Option<Timestamp>,
	/// The state of each backend being harvested
	pub backends: HashMap<HarvestBackendInternalType, HarvestBackendState>,
	/// When the harvest was last updated
	pub updated_at: Timestamp,
	/// Whether the harvest is a shadow run
	pub shadow_run: bool,
	/// Additional metadata about the harvest
	pub harvest_metadata: HarvestMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct HarvestMetadata {
	/// Whether the user being harvested is a Discord employee
	pub user_is_staff: bool,
	/// Whether an email has been sent informing the user that the archive is taking longer than expected
	pub sla_email_sent: bool,
	/// Whether the harvest bypasses the cooldown period for requesting harvests
	pub bypass_cooldown: bool,
	/// Whether the user being harvested is a provisional account
	pub is_provisional: bool,
}

#[derive(Serialize, Deserialize)]
pub enum HarvestState {
	/// The harvest is not yet complete
	INCOMPLETE,
	/// The harvest has been delivered to the user
	DELIVERED,
	/// The harvest has been cancelled
	CANCELLED,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HarvestStatus {
	/// The harvest is queued and has not been started
	QUEUED = 0,
	/// The harvest is currently running
	RUNNING = 1,
	/// The harvest has failed
	FAILED = 2,
	/// The harvest has completed successfully
	COMPLETED = 3,
	/// The harvest has been cancelled
	CANCELLED = 4,
}

#[derive(Serialize, Deserialize)]
pub enum HarvestBackendInternalType {
	/// All account information
	users,
	/// Actions the user has taken in Discord
	analytics,
	/// First-party embedded activity information
	activities_e,
	/// First-party embedded activity information
	activities_w,
	/// All user messages
	messages,
	/// Discord's HubSpot contact data, used for official Discord programs (e.g. partner)
	hubspot,
	/// All guilds the user is currently a member of
	guilds,
	/// Quest data
	ads,
}

#[derive(Serialize, Deserialize)]
pub enum HarvestBackendState {
	/// The backend has not been processed
	INITIAL,
	/// The backend is currently processing
	RUNNING,
	/// The backend has been processed
	EXTRACTED,
}

#[derive(Serialize, Deserialize)]
pub struct UserSurvey {
	/// The ID of the survey
	pub id: Snowflake,
	/// The ID of the survey
	pub key: Snowflake,
	/// The title of the survey
	pub prompt: String,
	/// The call-to-action text
	pub cta: String,
	/// The URL to the survey
	pub url: String,
	/// User requirements for the survey to be shown
	pub guild_requirements: Vec<SurveyRequirementType>,
	/// The guild member count requirements (min, max)
	pub guild_size: (Option<i64>, Option<i64>),
	/// The guild permissions bitwise value requirements
	pub guild_permissions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum SurveyRequirementType {
	/// The user must be the owner of a guild
	IS_OWNER,
	/// The user must have the ADMINISTRATOR permission in any guild
	IS_ADMIN,
	/// The user must be in a guild with the COMMUNITY feature
	IS_COMMUNITY,
	/// The user must be in a guild with a member count in a given range
	GUILD_SIZE,
	/// All guilds the user is in must have a member count in a given range
	GUILD_SIZE_ALL,
	/// The user must be in a guild with the HUB feature
	IS_HUB,
	/// The user must be currently viewing a guild
	IS_VIEWING,
	/// The user must have the given permissions in any guild
	GUILD_PERMISSIONS,
}

