use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::guild::{PartialGuild, PremiumTier, RoleConnectionOperatorType};
use crate::api::integrations::IntegrationApplication;
use crate::api::teams::{Company, Team};
use crate::api::users::PartialUser;
use crate::common::id::{ApplicationAssetId, ApplicationId, EulaId, GuildId, SkuId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct Application {
	/// The ID of the application
	pub id: ApplicationId,
	/// The name of the application
	pub name: String,
	/// The description of the application
	pub description: String,
	/// The application's icon hash
	pub icon: Option<String>,
	/// The application's default rich presence invite cover image hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<String>,
	/// The application's splash hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub splash: Option<String>,
	/// The type of the application , if any
	pub r#type: Option<ApplicationType>,
	/// The application's flags (including private)
	pub flags: ApplicationFlags,
	/// The ID of the application's primary SKU (game, application subscription, etc.)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_sku_id: Option<SkuId>,
	/// The hex encoded client public key for verification in interactions and the GameSDK's GetTicket
	pub verify_key: String,
	/// The ID of the guild linked to the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// The ID of the EULA required to play the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eula_id: Option<EulaId>,
	/// The URL slug that links to the primary store page of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// Other names the application's game is associated with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub aliases: Option<Vec<String>>,
	/// The unique executables of the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub executables: Option<Vec<ApplicationExecutable>>,
	/// The third party SKUs of the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub third_party_skus: Option<Vec<ApplicationSku>>,
	/// Whether the Discord client is allowed to hook into the application's game directly
	pub hook: bool,
	/// Whether the application's game supports the Discord overlay (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay: Option<bool>,
	/// The methods of overlaying that the application's game supports
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_methods: Option<OverlayMethodFlags>,
	/// Whether the Discord overlay is known to be problematic with this application's game (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_warn: Option<bool>,
	/// Whether to use the compatibility hook for the overlay (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_compatibility_hook: Option<bool>,
	/// The bot attached to this application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<PartialUser>,
	/// The owner of the application
	pub owner: PartialUser,
	/// The team that owns the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub team: Option<Option<Team>>,
	/// The companies that developed the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub developers: Option<Vec<Company>>,
	/// The companies that published the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publishers: Option<Vec<Company>>,
	/// The whitelisted RPC origin URLs for the application, if RPC is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Vec<String>>,
	/// The whitelisted URLs for redirecting to during OAuth2 authorization (max 10)
	pub redirect_uris: Vec<String>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<String>,
	/// Whether only the application owner can add the integration
	pub integration_public: bool,
	/// Whether the integration will only be added upon completion of a full OAuth2 token exchange
	pub integration_require_code_grant: bool,
	/// Whether only the application owner can add the bot
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_public: Option<bool>,
	/// Whether the application's bot will only be added upon completion of a full OAuth2 token exchange
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_require_code_grant: Option<bool>,
	/// Whether the application's bot is disabled by Discord (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_disabled: Option<bool>,
	/// Whether the application's bot is quarantined by Discord; quarantined bots cannot join more guilds or start new direct messages (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_quarantined: Option<bool>,
	/// Approximate count of guilds the application's bot is in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_guild_count: Option<u64>,
	/// Approximate count of users that have authorized the application with the applications.commands scope
	pub approximate_user_install_count: u64,
	/// Approximate count of users that have OAuth2 authorizations for the application
	pub approximate_user_authorization_count: u64,
	/// What guilds the application can be authorized in
	pub internal_guild_restriction: InternalGuildRestriction,
	/// The URL to the application's terms of service
	#[serde(skip_serializing_if = "Option::is_none")]
	pub terms_of_service_url: Option<String>,
	/// The URL to the application's privacy policy
	#[serde(skip_serializing_if = "Option::is_none")]
	pub privacy_policy_url: Option<String>,
	/// The role connection verification entry point of the integration; when configured, this will render the application as a verification method in guild role verification configuration
	pub role_connections_verification_url: Option<String>,
	/// The URL of the application's interactions endpoint
	pub interactions_endpoint_url: String,
	/// The version of the application's interactions endpoint implementation
	pub interactions_version: ApplicationInteractionsVersion,
	/// The enabled event webhook types to send to the interaction endpoint
	pub interactions_event_types: Vec<String>,
	/// Whether event webhooks are enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_webhooks_status: Option<EventWebhooksStatus>,
	/// The URL of the application's event webhooks endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_webhooks_url: Option<String>,
	/// The enabled event webhook types to send to the event webhooks endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_webhooks_types: Option<Vec<EventWebhooksType>>,
	/// Whether uploaded media content used in application commands is scanned and deleted for explicit content
	pub explicit_content_filter: ExplicitContentFilterLevel,
	/// Tags describing the content and functionality of the application (max 20 characters, max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
	/// The default in-app authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<ApplicationInstallParams>,
	/// The default custom authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<String>,
	/// The configuration for each integration type supported by the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_types_config: Option<
		HashMap<ApplicationIntegrationType, Option<ApplicationIntegrationTypeConfiguration>>,
	>,
	/// Whether the application is verified
	pub is_verified: bool,
	/// The current verification state of the application
	pub verification_state: ApplicationVerificationState,
	/// The current store approval state of the commerce application
	pub store_application_state: StoreApplicationState,
	/// The current RPC approval state of the application
	pub rpc_application_state: RpcApplicationState,
	/// The current guild creator monetization state of the application
	pub creator_monetization_state: ApplicationMonetizationState,
	/// Whether the application is discoverable in the application directory
	pub is_discoverable: bool,
	/// The current application directory discoverability state of the application
	pub discoverability_state: ApplicationDiscoverabilityState,
	/// The current application directory eligibility flags for the application
	pub discovery_eligibility_flags: ApplicationDiscoveryEligibilityFlags,
	/// Whether the application has monetization enabled
	pub is_monetized: bool,
	/// Whether the application has public subscriptions or products available for purchase
	pub storefront_available: bool,
	/// The current application monetization state of the application
	pub monetization_state: ApplicationMonetizationState,
	/// The current application monetization eligibility flags for the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub monetization_eligibility_flags: Option<ApplicationMonetizationEligibilityFlags>,
	/// The maximum possible participants in the application's embedded activity (-1 for no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<i64>,
	/// The configuration for the application's embedded activity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embedded_activity_config: Option<EmbeddedActivityConfig>,
}

/// Partial applications may have any combination of fields, depending on the context in which they are provided. Certain data may be included or omitted depending on what data is needed for the given operation. Fields [marked as required](https://docs.discord.food/reference#nullable-and-optional-resource-fields) will always be present.
/// Further complicating things, some optional fields may be omitted if their value is null, even if the specific partial does include this data.
#[derive(Serialize, Deserialize)]
pub struct PartialApplication {
	/// The ID of the application
	pub id: ApplicationId,
	/// The name of the application
	pub name: String,
	/// The description of the application
	pub description: String,
	/// The application's icon hash
	pub icon: Option<String>,
	/// The application's default rich presence invite cover image hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<String>,
	/// The application's splash hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub splash: Option<String>,
	/// The type of the application , if any
	pub r#type: Option<ApplicationType>,
	/// The application's flags (including private)
	pub flags: ApplicationFlags,
	/// The ID of the application's primary SKU (game, application subscription, etc.)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_sku_id: Option<SkuId>,
	/// The hex encoded client public key for verification in interactions and the GameSDK's GetTicket
	pub verify_key: String,
	/// The ID of the guild linked to the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// The guild linked to the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild: Option<PartialGuild>,
	/// The ID of the EULA required to play the application's game {/ todo: link this here /}
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eula_id: Option<EulaId>,
	/// The URL slug that links to the primary store page of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slug: Option<String>,
	/// Other names the application's game is associated with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub aliases: Option<Vec<String>>,
	/// The unique executables of the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub executables: Option<Vec<ApplicationExecutable>>,
	/// The third party SKUs of the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub third_party_skus: Option<Vec<ApplicationSku>>,
	/// Whether the Discord client is allowed to hook into the application's game directly
	pub hook: bool,
	/// Whether the application's game supports the Discord overlay (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay: Option<bool>,
	/// The methods of overlaying that the application's game supports
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_methods: Option<OverlayMethodFlags>,
	/// Whether the Discord overlay is known to be problematic with this application's game (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_warn: Option<bool>,
	/// Whether to use the compatibility hook for the overlay (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub overlay_compatibility_hook: Option<bool>,
	/// The bot attached to this application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<PartialUser>,
	/// The team that owns the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub team: Option<Option<Team>>,
	/// The companies that developed the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub developers: Option<Vec<Company>>,
	/// The companies that published the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publishers: Option<Vec<Company>>,
	/// The whitelisted RPC origin URLs for the application, if RPC is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Vec<String>>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<String>,
	/// Whether only the application owner can add the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_public: Option<bool>,
	/// Whether the integration will only be added upon completion of a full OAuth2 token exchange
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_require_code_grant: Option<bool>,
	/// Whether only the application owner can add the bot
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_public: Option<bool>,
	/// Whether the application's bot will only be added upon completion of a full OAuth2 token exchange
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_require_code_grant: Option<bool>,
	/// The URL to the application's terms of service
	#[serde(skip_serializing_if = "Option::is_none")]
	pub terms_of_service_url: Option<Option<String>>,
	/// The URL to the application's privacy policy
	#[serde(skip_serializing_if = "Option::is_none")]
	pub privacy_policy_url: Option<Option<String>>,
	/// Tags describing the content and functionality of the application (max 20 characters, max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
	/// The default in-app authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<ApplicationInstallParams>,
	/// The default custom authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<String>,
	/// The configuration for each integration type supported by the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_types_config: Option<
		HashMap<ApplicationIntegrationType, Option<ApplicationIntegrationTypeConfiguration>>,
	>,
	/// Whether the application is verified
	pub is_verified: bool,
	/// Whether the application is discoverable in the application directory
	pub is_discoverable: bool,
	/// Whether the application has monetization enabled
	pub is_monetized: bool,
	/// Whether the application has public subscriptions or products available for purchase
	pub storefront_available: bool,
	/// The maximum possible participants in the application's embedded activity (-1 for no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<i64>,
	/// The configuration for the application's embedded activity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embedded_activity_config: Option<EmbeddedActivityConfig>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationType {
	/// A game integrating with Discord
	GAME = 1,
	/// A limited application used for ticketed event SKUs
	TICKETED_EVENTS = 3,
	/// A limited application used for creator monetization (e.g. role subscription) SKUs
	CREATOR_MONETIZATION = 4,
}

bitflags! {
	pub struct ApplicationFlags: u64 {
		/// Embedded application is released to the public (see also release phases )
		const EMBEDDED_RELEASED = 1 << 1;
		/// Application can create managed emoji
		const MANAGED_EMOJI = 1 << 2;
		/// Embedded application can use in-app purchases
		const EMBEDDED_IAP = 1 << 3;
		/// Application can create group DMs without limit
		const GROUP_DM_CREATE = 1 << 4;
		/// Application has created 100+ AutoMod rules
		const AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
		/// Application has its game profile page disabled
		const GAME_PROFILE_DISABLED = 1 << 7;
		/// Application's OAuth2 credentials are considered public and a client secret is not required
		const PUBLIC_OAUTH2_CLIENT = 1 << 8;
		/// Application has limited access to the social layer SDK
		const SOCIAL_LAYER_INTEGRATION_LIMITED = 1 << 10;
		/// Intent required for bots in 100 or more guilds to receive Presence Update Gateway events
		const GATEWAY_PRESENCE = 1 << 12;
		/// Intent required for bots in under 100 guilds to receive Presence Update Gateway events
		const GATEWAY_PRESENCE_LIMITED = 1 << 13;
		/// Intent required for bots in 100 or more guilds to receive guild member-related events like Guild Member Add
		const GATEWAY_GUILD_MEMBERS = 1 << 14;
		/// Intent required for bots in under 100 guilds to receive guild member-related events like Guild Member Add
		const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
		/// Indicates unusual growth of an application that prevents verification
		const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
		/// Application can be embedded within the Discord client
		const EMBEDDED = 1 << 17;
		/// Intent required for bots in 100 or more guilds to receive message content
		const GATEWAY_MESSAGE_CONTENT = 1 << 18;
		/// Intent required for bots in under 100 guilds to receive message content
		const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
		/// Embedded application is created by Discord
		const EMBEDDED_FIRST_PARTY = 1 << 20;
		/// Unknown
		const APPLICATION_COMMAND_MIGRATED = 1 << 21;
		/// Application has registered global application commands
		const APPLICATION_COMMAND_BADGE = 1 << 23;
		/// Application has had at least one global application command used in the last 30 days
		const ACTIVE = 1 << 24;
		/// Application has not had any global application commands used in the last 30 days and has lost the ACTIVE flag
		const ACTIVE_GRACE_PERIOD = 1 << 25;
		/// Application can use IFrames within modals
		const IFRAME_MODAL = 1 << 26;
		/// Application can use the social layer SDK
		const SOCIAL_LAYER_INTEGRATION = 1 << 27;
		/// Application is promoted by Discord in the application directory
		const PROMOTED = 1 << 29;
		/// Application is a Discord partner
		const PARTNER = 1 << 30;
	}
}

bitflags! {
	pub struct OverlayMethodFlags: u64 {
		/// Overlay can be rendered out of process
		const OUT_OF_PROCESS = 1 << 0;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InternalGuildRestriction {
	/// The application can be authorized in any guild
	JOIN_ALL = 1,
	/// The application can only be authorized in guilds without the INTERNAL_EMPLOYEE_ONLY guild feature
	JOIN_EXTERNAL_ONLY = 2,
	/// The application can only be authorized in guilds with the INTERNAL_EMPLOYEE_ONLY guild feature
	JOIN_INTERNAL_ONLY = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationInteractionsVersion {
	/// Only Interaction Create events are sent as documented (default)
	VERSION_1 = 1,
	/// A selection of chosen events are sent
	VERSION_2 = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EventWebhooksStatus {
	/// Event webhooks are disabled
	DISABLED = 1,
	/// Event webhooks are enabled
	ENABLED = 2,
}

#[derive(Serialize, Deserialize)]
pub enum EventWebhooksType {
	/// Sent when a user authorizes the application
	APPLICATION_AUTHORIZED,
	/// Sent when a user creates an entitlement
	ENTITLEMENT_CREATE,
	/// Sent when a user enrolls in a quest
	QUEST_USER_ENROLLMENT,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
	/// Inherits the guild's explicit content filter
	INHERIT = 0,
	/// Media content will always be scanned
	ALWAYS = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationVerificationState {
	/// This application is ineligible for verification
	INELIGIBLE = 1,
	/// This application has not yet been applied for verification
	UNSUBMITTED = 2,
	/// This application has submitted a verification request
	SUBMITTED = 3,
	/// This application has been verified
	SUCCEEDED = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StoreApplicationState {
	/// This application does not have a commerce license
	NONE = 1,
	/// This application has a commerce license but has not yet submitted a store approval request
	PAID = 2,
	/// This application has submitted a store approval request
	SUBMITTED = 3,
	/// This application has been approved for the store
	APPROVED = 4,
	/// This application has been rejected from the store
	REJECTED = 5,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RpcApplicationState {
	/// This application does not have access to RPC
	DISABLED = 0,
	/// This application has not yet been applied for RPC access
	UNSUBMITTED = 1,
	/// This application has submitted a RPC access request
	SUBMITTED = 2,
	/// This application has been approved for RPC access
	APPROVED = 3,
	/// This application has been rejected from RPC access
	REJECTED = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationDiscoverabilityState {
	/// This application is ineligible for the application directory
	INELIGIBLE = 1,
	/// This application is not listed in the application directory
	NOT_DISCOVERABLE = 2,
	/// This application is listed in the application directory
	DISCOVERABLE = 3,
	/// This application is featurable in the application directory
	FEATUREABLE = 4,
	/// This application has been blocked from appearing in the application directory
	BLOCKED = 5,
}

bitflags! {
	pub struct ApplicationDiscoveryEligibilityFlags: u64 {
		/// Application is verified
		const VERIFIED = 1 << 0;
		/// Application has at least one tag set
		const TAG = 1 << 1;
		/// Application has a description
		const DESCRIPTION = 1 << 2;
		/// Application has terms of service set
		const TERMS_OF_SERVICE = 1 << 3;
		/// Application has a privacy policy set
		const PRIVACY_POLICY = 1 << 4;
		/// Application has a custom install URL or install parameters
		const INSTALL_PARAMS = 1 << 5;
		/// Application's name is safe for work
		const SAFE_NAME = 1 << 6;
		/// Application's description is safe for work
		const SAFE_DESCRIPTION = 1 << 7;
		/// Application has the message content intent approved or utilizes application commands
		const APPROVED_COMMANDS = 1 << 8;
		/// Application has a support guild set
		const SUPPORT_GUILD = 1 << 9;
		/// Application's commands are safe for work
		const SAFE_COMMANDS = 1 << 10;
		/// Application's owner has MFA enabled
		const MFA = 1 << 11;
		/// Application's directory long description is safe for work
		const SAFE_DIRECTORY_OVERVIEW = 1 << 12;
		/// Application has at least one supported locale set
		const SUPPORTED_LOCALES = 1 << 13;
		/// Application's directory short description is safe for work
		const SAFE_SHORT_DESCRIPTION = 1 << 14;
		/// Application's role connections metadata is safe for work
		const SAFE_ROLE_CONNECTIONS = 1 << 15;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationMonetizationState {
	/// This application does not have monetization set up
	NONE = 1,
	/// This application has monetization set up
	ENABLED = 2,
	/// This application has been blocked from monetizing
	BLOCKED = 3,
}

bitflags! {
	pub struct ApplicationMonetizationEligibilityFlags: u64 {
		/// Application is verified
		const VERIFIED = 1 << 0;
		/// Application is owned by a team
		const HAS_TEAM = 1 << 1;
		/// Application has the message content intent approved or utilizes application commands
		const APPROVED_COMMANDS = 1 << 2;
		/// Application has terms of service set
		const TERMS_OF_SERVICE = 1 << 3;
		/// Application has a privacy policy set
		const PRIVACY_POLICY = 1 << 4;
		/// Application's name is safe for work
		const SAFE_NAME = 1 << 5;
		/// Application's description is safe for work
		const SAFE_DESCRIPTION = 1 << 6;
		/// Application's role connections metadata is safe for work
		const SAFE_ROLE_CONNECTIONS = 1 << 7;
		/// User is the owner of the team that owns the application
		const USER_IS_TEAM_OWNER = 1 << 8;
		/// Application is not quarantined
		const NOT_QUARANTINED = 1 << 9;
		/// User's locale is supported by monetization
		const USER_LOCALE_SUPPORTED = 1 << 10;
		/// User is old enough to use monetization
		const USER_AGE_SUPPORTED = 1 << 11;
		/// User has a date of birth defined on their account
		const USER_DATE_OF_BIRTH_DEFINED = 1 << 12;
		/// User has MFA enabled
		const USER_MFA_ENABLED = 1 << 13;
		/// User's email is verified
		const USER_EMAIL_VERIFIED = 1 << 14;
		/// All members of the team that owns the application have verified emails
		const TEAM_MEMBERS_EMAIL_VERIFIED = 1 << 15;
		/// All members of the team that owns the application have MFA enabled
		const TEAM_MEMBERS_MFA_ENABLED = 1 << 16;
		/// This application has no issues blocking monetization
		const NO_BLOCKING_ISSUES = 1 << 17;
		/// Owning team has a valid payout status
		const VALID_PAYOUT_STATUS = 1 << 18;
	}
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationExecutable {
	/// The operating system the executable can be found on
	pub os: String,
	/// The name of the executable
	pub name: String,
	/// Whether the executable is for a game launcher
	pub is_launcher: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationSku {
	/// The ID of the game
	pub id: Option<String>,
	/// The SKU of the game
	pub sku: Option<String>,
	/// The distributor of the game
	pub distributor: String,
}

pub enum ApplicationDistributorType {
	/// Discord Store
	discord,
	/// Steam
	steam,
	/// Twitch
	twitch,
	/// Ubisoft Connect
	uplay,
	/// Battle.net
	battlenet,
	/// Origin
	origin,
	/// GOG.com
	gog,
	/// Epic Games Store
	epic,
	/// Google Play Store
	google_play,
	/// NVIDIA Cloud Gaming
	nvidia_gdn_app,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationInstallParams {
	/// The scopes to authorize the integration with
	pub scopes: Vec<String>,
	/// The permissions to request for the application's bot integration role
	pub permissions: String,
}

/// An application's supported installation contexts.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApplicationIntegrationType {
	/// Guild installation context
	GUILD_INSTALL = 0,
	/// User installation context
	USER_INSTALL = 1,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationIntegrationTypeConfiguration {
	/// The default in-app authorization link for the installation context
	#[serde(skip_serializing_if = "Option::is_none")]
	pub oauth2_install_params: Option<ApplicationInstallParams>,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationProxyConfig {
	/// The URLs mapped to the proxy
	pub url_map: Vec<ApplicationProxyMapping>,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationProxyMapping {
	/// The prefix on the proxy
	pub prefix: String,
	/// The domain to proxy
	pub target: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedActivityConfig {
	/// The ID of the application this embedded activity is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<ApplicationId>,
	/// The ID of the application asset to preview the activity with
	pub activity_preview_video_asset_id: Option<ApplicationAssetId>,
	/// The platforms this activity is supported on
	pub supported_platforms: Vec<String>,
	/// The default orientation lock state for the activity on mobile
	pub default_orientation_lock_state: EmbeddedActivityOrientationLockStateType,
	/// The default orientation lock state for the activity on tablets
	pub tablet_default_orientation_lock_state: EmbeddedActivityOrientationLockStateType,
	/// Whether the activity is age gated
	pub requires_age_gate: bool,
	/// Whether the activity uses a responsive aspect ratio instead of a dynamic aspect ratio
	pub legacy_responsive_aspect_ratio: bool,
	/// The minimum guild premium tier required to use the activity, if any
	#[deprecated]
	pub premium_tier_requirement: Option<PremiumTier>,
	/// When the current free period for the activity starts, if any
	#[deprecated]
	pub free_period_starts_at: Option<Timestamp>,
	/// When the current free period for the activity ends, if any
	#[deprecated]
	pub free_period_ends_at: Option<Timestamp>,
	/// The release configuration for the activity on each platform
	pub client_platform_config: HashMap<String, EmbeddedActivityPlatformConfig>,
	/// The rank of the activity in the activity shelf sort order
	pub shelf_rank: i64,
	/// Whether the activity is not routed through the Discord activity proxy
	pub has_csp_exception: bool,
	/// Whether the activity displays advertisements
	pub displays_advertisements: bool,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EmbeddedActivityOrientationLockStateType {
	/// Unrestricted orientation
	UNLOCKED = 1,
	/// Portrait only
	PORTRAIT = 2,
	/// Landscape only
	LANDSCAPE = 3,
}

pub enum EmbeddedActivityPlatformType {
	/// Web
	web,
	/// Android
	android,
	/// iOS
	ios,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedActivityPlatformConfig {
	/// The type of release label for the platform
	pub label_type: EmbeddedActivityLabelType,
	/// When the release label expires
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label_until: Option<Option<Timestamp>>,
	/// The release phase for the platform
	pub release_phase: String,
	/// The surfaces to omit the activity badge from
	pub omit_badge_from_surfaces: Vec<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EmbeddedActivityLabelType {
	/// No special label
	NONE = 0,
	/// The activity is new
	NEW = 1,
	/// The activity has been recently updated
	UPDATED = 2,
}

pub enum EmbeddedActivityReleasePhase {
	/// The activity is still in development
	in_development,
	/// The activity is available to guilds with the ACTIVITIES_ALPHA guild feature
	activities_team,
	/// The activity is available to guilds with the ACTIVITIES_EMPLOYEE guild feature
	employee_release,
	/// The activity is available to all guilds
	soft_launch,
	/// The activity is available to all guilds
	soft_launch_multi_geo,
	/// The activity is available to all guilds
	global_launch,
}

pub enum EmbeddedActivitySurface {
	/// The activity launcher in the voice channel interface
	voice_launcher,
	/// The activity launcher in the text channel interface
	text_launcher,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationAsset {
	/// The ID of the asset
	pub id: String,
	/// The type of the asset
	pub r#type: ApplicationAssetType,
	/// The name of the asset
	pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationAssetType {
	/// Unknown
	ONE = 1,
	/// Unknown
	TWO = 2,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationRoleConnection {
	/// The vanity name of the platform a bot has connected (max 50 characters)
	pub platform_name: Option<String>,
	/// The username on the platform a bot has connected (max 100 characters)
	pub platform_username: Option<String>,
	/// Object mapping application role connection metadata keys to their string-ified value (max 100 characters) for the user on the platform a bot has connected
	pub metadata: Value,
	/// The application that owns the role connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application: Option<IntegrationApplication>,
	/// The metadata that the application has set for the role connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_metadata: Option<Vec<ApplicationRoleConnectionMetadata>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationRoleConnectionMetadata {
	/// The type of metadata value
	pub r#type: RoleConnectionOperatorType,
	/// Key for the metadata field (1-50 characters, must be a-z , 0-9 , or _ )
	pub key: String,
	/// The name of the metadata field (1-100 characters)
	pub name: String,
	/// Translations of the name with keys in available locales
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name_localizations: Option<HashMap<String, String>>,
	/// The description of the metadata field (1-200 characters)
	pub description: String,
	/// Translations of the description with keys in available locales
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityLink {
	/// The application ID
	pub application_id: ApplicationId,
	/// The link ID
	pub link_id: String,
	/// The hash of the application quick link asset
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_path: Option<String>,
	/// The ID of the application asset
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_id: Option<ApplicationAssetId>,
	/// The title of the activity link
	pub title: String,
	/// The description of the activity link
	pub description: String,
	/// A custom ID for the activity link
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_id: Option<Option<String>>,
	/// The primary call to action for the activity link
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_cta: Option<Option<String>>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityLinkType {
	/// Managed by the application and last indefinitely
	MANAGED_LINK = 0,
	/// Made by the user and last for 30 days
	QUICK_LINK = 1,
}
