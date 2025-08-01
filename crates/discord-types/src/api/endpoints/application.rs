use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};

use crate::api::application::{
	ActivityLink,
	Application,
	ApplicationAsset,
	ApplicationDisclosureType,
	ApplicationDiscoverabilityState,
	ApplicationDiscoveryEligibilityFlags,
	ApplicationDistributor,
	ApplicationFlags,
	ApplicationInstallParams,
	ApplicationIntegrationType,
	ApplicationIntegrationTypeConfiguration,
	ApplicationInteractionsVersion,
	ApplicationMissingDataType,
	ApplicationMonetizationState,
	ApplicationProxyConfig,
	ApplicationRoleConnection,
	ApplicationType,
	DetectableApplication,
	EmbeddedActivityConfig,
	EmbeddedActivityOrientationLockStateType,
	EmbeddedActivityPlatformConfig,
	EmbeddedActivityPlatformType,
	ExplicitContentFilterLevel,
	ExternalAsset,
	PartialApplication,
	WhitelistedUser,
};
use crate::api::integrations::IntegrationType;
use crate::api::messages::Attachment;
use crate::api::presences::OperatingSystemDesktopType;
use crate::api::users::User;
use crate::common::id::{
	ActivityLinkId,
	ApplicationAssetId,
	ApplicationId,
	ChannelId,
	DeveloperId,
	GuildId,
	PublisherId,
	TeamId,
	UserId,
};
use crate::common::image::ImageHash;
use crate::common::timestamp::Timestamp;
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Returns a list of [application](https://docs.discord.food/resources/application#application-object) objects that the current user has.
pub fn GET_APPLICATIONS(query: &GetApplicationsQueryParams) -> String {
	format!("/applications{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationsQueryParams {
	/// Whether to include applications that a team the user is a part of owns
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_team_applications: Option<bool>,
}

pub type GetApplicationsResponse = Vec<Application>;

/// Method: `GET`
///
/// Returns a list of [application](https://docs.discord.food/resources/application#application-object) objects that the current user has, additionally including the application's assets.
pub fn GET_APPLICATIONS_WITH_ASSETS(query: &GetApplicationsWithAssetsQueryParams) -> String {
	format!("/applications-with-assets{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationsWithAssetsQueryParams {
	/// Whether to include applications that a team the user is a part of owns
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_team_applications: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationsWithAssetsResponse {
	/// The applications the user has
	pub applications: Vec<Application>,
	/// The assets for each application
	pub assets: HashMap<ApplicationId, Vec<ApplicationAsset>>,
}

/// Method: `POST`
///
/// Creates a new application.
///
/// Users can have a maximum of 50 applications, with each team able to have a maximum of 25.
///
/// Returns an [application](https://docs.discord.food/resources/application#application-object) object on success.
pub const CREATE_APPLICATION: &str = "/applications";

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationRequest {
	/// The name of the application
	pub name: String,
	/// The type of the application (only CREATOR_MONETIZATION is supported)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ApplicationType>,
	/// The ID of the team to create this application under
	#[serde(skip_serializing_if = "Option::is_none")]
	pub team_id: Option<TeamId>,
	/// The description of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<String>>,
	/// The application's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The application's default rich presence invite cover image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<Option<ImageHash>>,
	/// the application's flags (only GATEWAY_GUILD_MEMBERS_LIMITED , GATEWAY_PRESENCE_LIMITED , and GATEWAY_MESSAGE_CONTENT_LIMITED can be set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ApplicationFlags>,
	/// The ID of the guild linked to the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Option<GuildId>>,
	/// The whitelisted URLs for redirecting to during OAuth2 authorization (max 10)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redirect_uris: Option<Option<Vec<String>>>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<Option<String>>,
}

pub type CreateApplicationResponse = Application;

/// Method: `GET`
///
/// User must be the owner of the application or member of the current team.
///
/// Returns an [application](https://docs.discord.food/resources/application#application-object) object for the given ID.
pub fn GET_APPLICATION(application_id: &ApplicationId) -> String {
	format!("/applications/{}", application_id)
}

pub type GetApplicationResponse = Application;

/// Method: `GET`
///
/// Returns the [application](https://docs.discord.food/resources/application#application-object) object associated with the requestor.
#[deprecated(note = "This endpoint is not usable by user accounts.")]
pub const GET_CURRENT_APPLICATION: &str = "/applications/@me";

pub type GetCurrentApplicationResponse = Application;

/// Method: `PATCH`
///
/// Modifies an application.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns the updated [application](https://docs.discord.food/resources/application#application-object) object on success.
pub fn MODIFY_APPLICATION(application_id: &ApplicationId) -> String {
	format!("/applications/{}", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyApplicationResponseRequest {
	/// The name of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The description of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<String>>,
	/// The application's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The application's default rich presence invite cover image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<Option<ImageHash>>,
	/// The application's flags (only PUBLIC_OAUTH2_CLIENT , GATEWAY_GUILD_MEMBERS_LIMITED , GATEWAY_PRESENCE_LIMITED , and GATEWAY_MESSAGE_CONTENT_LIMITED can be set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ApplicationFlags>,
	/// The ID of the guild linked to the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Option<GuildId>>,
	/// The IDs of the companies that developed the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub developer_ids: Option<Option<Vec<DeveloperId>>>,
	/// The IDs of the companies that published the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publisher_ids: Option<Option<Vec<PublisherId>>>,
	/// The whitelisted RPC origin URLs for the application, if RPC is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Option<Vec<String>>>,
	/// The whitelisted URLs for redirecting to during OAuth2 authorization (max 10)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redirect_uris: Option<Option<Vec<String>>>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<Option<String>>,
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
	/// The role connection verification entry point of the integration; when configured, this will render the application as a verification method in guild role verification configuration
	pub role_connections_verification_url: Option<String>,
	/// The URL of the application's interactions endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_endpoint_url: Option<String>,
	/// The version of the application's interactions endpoint implementation
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_version: Option<u8>,
	/// The enabled Gateway events to send to the interaction endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_event_types: Option<Option<Vec<String>>>,
	/// Whether uploaded media content used in application commands is scanned and deleted for explicit content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
	/// Tags describing the content and functionality of the application (max 20 characters, max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Option<Vec<String>>>,
	/// The default in-app authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<Option<ApplicationInstallParams>>,
	/// The default custom authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<Option<String>>,
	/// The configuration for each integration type supported by the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_types_config: Option<
		HashMap<ApplicationIntegrationType, Option<ApplicationIntegrationTypeConfiguration>>,
	>,
	/// The current application directory discoverability state of the application (only NOT_DISCOVERABLE and DISCOVERABLE is supported)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discoverability_state: Option<ApplicationDiscoverabilityState>,
	/// The current application monetization state of the application (only NONE and ENABLED is supported)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub monetization_state: Option<ApplicationMonetizationState>,
	/// The maximum possible participants in the application's embedded activity (-1 for no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<Option<i32>>,
}

pub type ModifyApplicationResponse = Application;

/// Method: `PATCH`
///
/// Modifies the requestor's application information. Returns the updated [application](https://docs.discord.food/resources/application#application-object) object on success.
#[deprecated(note = "This endpoint is not usable by user accounts.")]
pub const MODIFY_CURRENT_APPLICATION: &str = "/applications/@me";

#[derive(Serialize, Deserialize)]
pub struct ModifyCurrentApplicationRequest {
	/// The description of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<String>>,
	/// The application's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The application's default rich presence invite cover image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<Option<ImageHash>>,
	/// The application's flags (only GATEWAY_GUILD_MEMBERS_LIMITED , GATEWAY_PRESENCE_LIMITED , and GATEWAY_MESSAGE_CONTENT_LIMITED can be set)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ApplicationFlags>,
	/// The whitelisted RPC origin URLs for the application, if RPC is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Option<Vec<String>>>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<Option<String>>,
	/// The role connection verification entry point of the integration; when configured, this will render the application as a verification method in guild role verification configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_connections_verification_url: Option<Option<String>>,
	/// The URL of the application's interactions endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_endpoint_url: Option<Option<String>>,
	/// The version of the application's interactions endpoint implementation
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_version: Option<ApplicationInteractionsVersion>,
	/// The enabled Gateway events to send to the interaction endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_event_types: Option<Option<Vec<String>>>,
	/// Whether uploaded media content used in application commands is scanned and deleted for explicit content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
	/// Tags describing the content and functionality of the application (max 20 characters, max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Option<ArrayVec<ArrayString<20>, 5>>>,
	/// The default in-app authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<Option<ApplicationInstallParams>>,
	/// The default custom authorization link for the integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<Option<String>>,
	/// The configuration for each integration type supported by the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_types_config:
		Option<HashMap<IntegrationType, Option<ApplicationIntegrationTypeConfiguration>>>,
	/// The maximum possible participants in the application's embedded activity (-1 for no limit)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<Option<i32>>,
}

pub type ModifyCurrentApplicationResponse = Application;

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Deletes an application permanently.
///
/// User must be the owner of the application or current team.
///
/// Returns a 204 empty response on success.
pub fn DELETE_APPLICATION(application_id: &ApplicationId) -> String {
	format!("/applications/{}/delete", application_id)
}

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Transfers ownership of an application to a [team](https://docs.discord.food/resources/team).
///
/// User must be the owner of the application or current team.
///
/// Returns an [application](https://docs.discord.food/resources/application#application-object) object on success.
pub fn TRANSFER_APPLICATION(application_id: &ApplicationId) -> String {
	format!("/applications/{}/transfer", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct TransferApplicationRequest {
	/// The ID of the team to transfer ownership to
	pub team_id: TeamId,
}

pub type TransferApplicationResponse = Application;

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Resets the application's client secret.
///
/// This revokes all previous secrets and returns a new secret.
///
/// User must be the owner of the application or developer of the current team.
pub fn RESET_APPLICATION_SECRET(application_id: &ApplicationId) -> String {
	format!("/applications/{}/reset", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ResetApplicationSecretResponse {
	/// The client secret key of the application
	pub secret: String,
}

/// Method: `GET`
///
/// User must be the owner of the application or member of the current team.
///
/// Returns a list of [whitelisted user](https://docs.discord.food/resources/application#whitelisted-user-structure) objects representing the invited testers for the given application ID.
pub fn GET_APPLICATION_TESTERS(application_id: &ApplicationId) -> String {
	format!("/oauth2/applications/{}/allowlist", application_id)
}

pub type GetApplicationTestersResponse = Vec<WhitelistedUser>;

/// Method: `POST`
///
/// Adds a user to the application's list of testers.
///
/// User must be the owner of the application or developer of the current team.
///
/// User must be friends with the user you are inviting.
///
/// Applications may have a maximum of 50 whitelisted users.
///
/// Returns a [whitelisted user](https://docs.discord.food/resources/application#whitelisted-user-structure) object on success.
pub fn ADD_APPLICATION_TESTER(application_id: &ApplicationId) -> String {
	format!("/oauth2/applications/{}/allowlist", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct AddApplicationTesterRequest {
	/// The username of the user to add
	pub username: String,
	/// The discriminator of the user to add
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discriminator: Option<Option<String>>,
}

pub type AddApplicationTesterResponse = WhitelistedUser;

/// Method: `POST`
///
/// Accepts an application tester invitation received via email.
///
/// Invited users will receive an email with a link that redirects to the official Discord client with a verification token present in the URL's query (e.g. https://discord.com/oauth2/allowlist/accept?token=h9sYyrafnMhhObX4nGi9VOugCa9CSt).
///
/// Returns a 204 empty response on success.
pub fn ACCEPT_APPLICATION_TESTER_INVITATION(
	query: &AcceptApplicationTesterInvitationQueryParams
) -> String {
	format!("/oauth2/allowlist/accept{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct AcceptApplicationTesterInvitationQueryParams {
	/// The verification token from the URL
	pub token: String,
}

/// Method: `DELETE`
///
/// Removes a user from the application's list of testers.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn REMOVE_APPLICATION_TESTER(
	application_id: &ApplicationId,
	user_id: &UserId,
) -> String {
	format!(
		"/oauth2/applications/{}/allowlist/{}",
		application_id, user_id
	)
}

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Creates and attaches a bot to the given application ID. User must be the owner of the application or developer of the current team.
///
/// All newly-created applications have a bot attached by default, so this endpoint is only useful for older applications.
#[deprecated]
pub fn CREATE_APPLICATION_BOT(application_id: &ApplicationId) -> String {
	format!("/applications/{}/bot", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationBotResponse {
	/// The token of the bot, if a bot was newly created
	pub token: Option<String>,
}

/// Method: `PATCH`
///
/// Modifies the application's bot.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns the updated [user](https://docs.discord.food/resources/user#user-object) object on success.
pub fn MODIFY_APPLICATION_BOT(application_id: &ApplicationId) -> String {
	format!("/applications/{}/bot", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyApplicationBotRequest {
	/// The user's username (2-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub username: Option<String>,
	/// The user's avatar
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<Option<ImageHash>>,
	/// The user's banner
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<ImageHash>>,
}

pub type ModifyApplicationBotResponse = User;

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Resets the application's bot token.
///
/// This revokes all previous tokens and returns a new token.
///
/// User must be the owner of the application or developer of the current team.
pub fn RESET_APPLICATION_BOT_TOKEN(application_id: &ApplicationId) -> String {
	format!("/applications/{}/bot/reset", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ResetApplicationBotTokenResponse {
	/// The token of the bot
	pub token: String,
}

/// Method: `POST`
///
/// Submits a request for Gateway intents for a verified bot.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn REQUEST_APPLICATION_GATEWAY_INTENTS(application_id: &ApplicationId) -> String {
	format!(
		"/applications/{}/request-additional-intents",
		application_id
	)
}

#[derive(Serialize, Deserialize)]
pub struct RequestApplicationGatewayIntentsRequest {
	/// The description of the application (50-2000 characters)
	pub application_description: String,
	/// The application flags representing the requested Gateway intents (only GATEWAY_PRESENCE , GATEWAY_GUILD_MEMBERS , and GATEWAY_MESSAGE_CONTENT are supported)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_flags_requested: Option<ApplicationFlags>,
	/// The use case for requesting the presence intent (50-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_use_case_description: Option<Option<ArrayString<2000>>>,
	/// The supplemental material for the requested Gateway presence intent (5-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_use_case_supplemental_material_description:
		Option<Option<ArrayString<2000>>>,
	/// Whether the application stores presence data off-platform
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_store_off_platform: Option<Option<bool>>,
	/// Whether the application retains presence data for 30 days or less
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_retention: Option<Option<bool>>,
	/// Whether the application encrypts stored presence data at rest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_encrypted: Option<Option<bool>>,
	/// Whether application users can opt out of having their presence data stored
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_opt_out_stored: Option<Option<bool>>,
	/// How application users can request the deletion of their presence data (25-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_contact_deletion: Option<Option<ArrayString<2000>>>,
	/// The use case for requesting the guild members intent (50-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_use_case_description: Option<Option<ArrayString<2000>>>,
	/// The supplemental material for the requested Gateway guild members intent (5-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_use_case_supplemental_material_description:
		Option<Option<ArrayString<2000>>>,
	/// Whether the application stores guild member data off-platform
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_store_off_platform: Option<Option<bool>>,
	/// Whether the application retains guild member datafor 30 days or less
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_retention: Option<Option<bool>>,
	/// Whether the application encrypts stored guild member data at rest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_encrypted: Option<Option<bool>>,
	/// How application users can request the deletion of their guild member data (25-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_contact_deletion: Option<Option<ArrayString<2000>>>,
	/// The use case for requesting the message content intent (50-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_use_case_description: Option<Option<ArrayString<2000>>>,
	/// The supplemental material for the requested Gateway message content intent (5-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_use_case_supplemental_material_description:
		Option<Option<ArrayString<2000>>>,
	/// Whether the application stores message content data off-platform
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_store_off_platform: Option<Option<bool>>,
	/// Whether the application retains message content data for 30 days or less
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_retention: Option<Option<bool>>,
	/// Whether the application encrypts stored message content data at rest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_encrypted: Option<Option<bool>>,
	/// Whether application users can opt out of having their message content data stored
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_opt_out_stored: Option<Option<bool>>,
	/// Whether the application uses message content data for AI training
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_ai_training: Option<Option<bool>>,
	/// Whether the application has a public privacy policy detailing how message content data is used
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_public: Option<Option<bool>>,
	/// Where the application's privacy policy can be found (25-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_location: Option<Option<ArrayString<2000>>>,
	/// A link to or screenshots of the application's privacy policy (25-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_example: Option<Option<ArrayString<2000>>>,
	/// How application users can request the deletion of their message content data (25-2000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_contact_deletion: Option<Option<ArrayString<2000>>>,
}

/// Method: `GET`
///
/// Returns information about the application's eligibility for application directory.
///
/// User must be the owner of the application or member of the current team.
pub fn GET_APPLICATION_DISCOVERABILITY_STATE(application_id: &ApplicationId) -> String {
	format!("/applications/{}/discoverability-state", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationDiscoverabilityStateResponse {
	/// The current application directory discoverability state of the application
	pub discoverability_state: ApplicationDiscoverabilityState,
	/// The current application directory eligibility flags for the application
	pub discovery_eligibility_flags: ApplicationDiscoveryEligibilityFlags,
	/// Not safe for work commands that are not allowed in the application directory
	pub bad_commands: Vec<ApplicationCommand>,
}

/// Method: `GET`
///
/// Queries whether the user can use test mode for the application.
///
/// Test mode allows completing purchases without payment.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn QUERY_APPLICATION_TEST_MODE(application_id: &ApplicationId) -> String {
	format!("/activities/{}/test-mode", application_id)
}

/// Method: `GET`
///
/// Returns the embedded activities available globally or in a particular guild.
pub fn GET_EMBEDDED_ACTIVITIES(query: &GetEmbeddedActivitiesQueryParams) -> String {
	format!("/activities/shelf{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetEmbeddedActivitiesQueryParams {
	/// The ID to return embedded activities for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct GetEmbeddedActivitiesResponse {
	/// The available embedded activities
	pub activities: Vec<EmbeddedActivityConfig>,
	/// Applications representing the available embedded activities
	pub applications: Vec<PartialApplication>,
	/// The assets for each application
	pub assets: HashMap<ApplicationId, Vec<ApplicationAsset>>,
}

/// Method: `POST`
///
/// Modifies whether the application is an embedded activity or not (determined by the [EMBEDDED flag](https://docs.discord.food/resources/application#application-flags)).
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn SET_APPLICATION_EMBEDDABILITY(application_id: &ApplicationId) -> String {
	format!("/applications/{}/set-embedded", application_id)
}

/// Method: `GET`
///
/// User must be the owner of the application or member of the current team.
///
/// Returns the [embedded activity config](https://docs.discord.food/resources/application#embedded-activity-config-object) object for the given application ID.
pub fn GET_APPLICATION_EMBEDDED_ACTIVITY_CONFIG(application_id: &ApplicationId) -> String {
	format!("/applications/{}/embedded-activity-config", application_id)
}

pub type GetApplicationEmbeddedActivityConfigResponse = EmbeddedActivityConfig;

/// Method: `PATCH`
///
/// Modifies the embedded activity config for the given application ID.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns the updated [embedded activity config](https://docs.discord.food/resources/application#embedded-activity-config-object) object on success.
pub fn MODIFY_APPLICATION_EMBEDDED_ACTIVITY_CONFIG(application_id: &ApplicationId) -> String {
	format!("/applications/{}/embedded-activity-config", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyApplicationEmbeddedActivityConfigRequest {
	/// The ID of the application asset to preview the activity with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub activity_preview_video_asset_id: Option<Option<ApplicationAssetId>>,
	/// The platforms this activity is supported on
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_platforms: Option<Option<Vec<EmbeddedActivityPlatformType>>>,
	/// The default orientation lock state for the activity on mobile
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_orientation_lock_state: Option<EmbeddedActivityOrientationLockStateType>,
	/// The default orientation lock state for the activity on tablets
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tablet_default_orientation_lock_state: Option<EmbeddedActivityOrientationLockStateType>,
	/// Whether the activity is age gated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub requires_age_gate: Option<bool>,
	/// When the current free period for the activity starts, if any
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub free_period_starts_at: Option<Option<Timestamp>>,
	/// When the current free period for the activity ends, if any
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub free_period_ends_at: Option<Option<Timestamp>>,
	/// The release configuration for the activity on each platform
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_platform_config: Option<HashMap<String, EmbeddedActivityPlatformConfig>>,
	/// The rank of the activity in the activity shelf sort order
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shelf_rank: Option<i64>,
}

pub type ModifyApplicationEmbeddedActivityConfigResponse = EmbeddedActivityConfig;

/// Method: `GET`
///
/// User must be the owner of the application or member of the current team.
///
/// Returns the application's [activity proxy config](https://docs.discord.food/resources/application#application-proxy-config-object) object for the given application ID.
pub fn GET_APPLICATION_PROXY_CONFIG(application_id: &ApplicationId) -> String {
	format!("/applications/{}/proxy-config", application_id)
}

pub type GetApplicationProxyConfigResponse = ApplicationProxyConfig;

/// Method: `POST`
///
/// Replaces the activity proxy config for the given application ID. User must be the owner of the application or developer of the current team.
///
/// URL mappings can utilize any protocol, so the protocol should be omitted from the target field.
///
/// Parameter matching is supported in both the prefix and target fields. For example, you can map /server/{id} to server-{id}.example.com.
///
/// Because of how URL globbing works, the order of the mappings is important. The most specific mappings should be at the top of the list as the first match is used. For example, if you have /foo and /foo/bar, you must place the URL /foo/bar before /foo or else the mapping for /foo/bar will never be reached.
///
/// Returns the updated [application proxy config](https://docs.discord.food/resources/application#application-proxy-config-object) object on success.
pub fn MODIFY_APPLICATION_PROXY_CONFIG(application_id: &ApplicationId) -> String {
	format!("/applications/{}/proxy-config", application_id)
}

pub type ModifyApplicationProxyConfigRequest = ApplicationProxyConfig;

pub type ModifyApplicationProxyConfigResponse = ApplicationProxyConfig;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a list of [application assets](https://docs.discord.food/resources/application#application-asset-object) for the given application ID.
pub fn GET_APPLICATION_ASSETS(
	query: &GetApplicationAssetsQueryParams,
	application_id: &ApplicationId,
) -> String {
	format!(
		"/oauth2/applications/{}/assets{}",
		application_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationAssetsQueryParams {
	/// Whether to bypass the Cloudflare cache for the response (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nocache: Option<bool>,
}

pub type GetApplicationAssetsResponse = Vec<ApplicationAsset>;

/// Method: `POST`
///
/// Creates a new application asset for the given application ID.
///
/// User must be the owner of the application or developer of the current team.
///
/// Due to caching, it may take a while for the asset to be retrievable after creation.
///
/// Returns an [application asset](https://docs.discord.food/resources/application#application-asset-object) object on success.
pub fn CREATE_APPLICATION_ASSET(application_id: &ApplicationId) -> String {
	format!("/oauth2/applications/{}/assets", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationAssetRequest {
	/// The name of the asset
	pub name: String,
	/// The type of the asset
	pub r#type: i64,
	/// The asset's image
	pub image: ImageHash,
}

pub type CreateApplicationAssetResponse = ApplicationAsset;

/// Method: `DELETE`
///
/// Deletes an application asset permanently.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn DELETE_APPLICATION_ASSET(
	application_id: &ApplicationId,
	asset_id: &ApplicationAssetId,
) -> String {
	format!(
		"/oauth2/applications/{}/assets/{}",
		application_id, asset_id
	)
}

/// Method: `POST`
///
/// Proxies a list of URLs for the given application ID.
///
/// Returns a list of [external asset](https://docs.discord.food/resources/application#external-asset-structure) objects on success.
pub fn PROXY_APPLICATION_ASSETS(application_id: &ApplicationId) -> String {
	format!("/applications/{}/external-assets", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ProxyApplicationAssetsRequest {
	/// The URLs of the assets to proxy (max 256 characters, 1-2)
	pub urls: ArrayVec<ArrayString<256>, 2>,
}

pub type ProxyApplicationAssetsResponse = Vec<ExternalAsset>;

/// Method: `POST`
///
/// Supports OAuth2 for authentication
///
/// Uploads an ephemeral attachment to the application.
///
/// Must be a multipart/form-data body.
///
/// Requires the [EMBEDDED application flag](https://docs.discord.food/resources/application#application-flags).
pub fn CREATE_APPLICATION_ATTACHMENT(application_id: &ApplicationId) -> String {
	format!("/applications/{}/attachment", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationAttachmentRequest {
	/// The image file to upload, must be a JPEG, PNG, or GIF file
	pub file: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationAttachmentResponse {
	/// The created ephemeral attachment
	pub attachment: Attachment,
}

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a list of [detectable application](https://docs.discord.food/resources/application#detectable-application-structure) objects representing games that can be detected by Discord for rich presence.
pub const GET_DETECTABLE_APPLICATIONS: &str = "/applications/detectable";

pub type GetDetectableApplicationsResponse = Vec<DetectableApplication>;

/// Method: `GET`
///
/// Returns a list of partial [application](https://docs.discord.food/resources/application#application-object) objects for the given IDs.
pub fn GET_PARTIAL_APPLICATIONS(query: &GetPartialApplicationsQueryParams) -> String {
	format!("/applications/public{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetPartialApplicationsQueryParams {
	/// The IDs of the applications to fetch; unknown IDs are ignored
	pub application_ids: Vec<ApplicationId>,
}

pub type GetPartialApplicationsResponse = Vec<PartialApplication>;

/// Method: `GET`
///
/// Supports OAuth2 for authentication
///
/// Returns a partial [application](https://docs.discord.food/resources/application#application-object) object for the given ID with all public application fields.
pub fn GET_PARTIAL_APPLICATION(
	query: &GetPartialApplicationQueryParams,
	application_id: &ApplicationId,
) -> String {
	format!(
		"/applications/{}/public{}",
		application_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetPartialApplicationQueryParams {
	/// Whether to include the guild object in the response if the guild is discoverable (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_guild: Option<bool>,
}

pub type GetPartialApplicationResponse = PartialApplication;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a partial [application](https://docs.discord.food/resources/application#application-object) object for the given ID with rich presence-related fields.
pub fn GET_RICH_PRESENCE_APPLICATION(application_id: &ApplicationId) -> String {
	format!("/applications/{}/rpc", application_id)
}

pub type GetRichPresenceApplicationResponse = PartialApplication;

/// Method: `GET`
///
/// Returns an object representing additional safety disclosures for the application.
pub fn GET_APPLICATION_DISCLOSURES(application_id: &ApplicationId) -> String {
	format!("/applications/{}/disclosures", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetApplicationDisclosuresResponse {
	/// The disclosures of the application
	pub disclosures: Vec<ApplicationDisclosureType>,
	/// The disclosures that have been acknowledged by the user
	pub acked_disclosures: Vec<ApplicationDisclosureType>,
	/// Whether all disclosures have been acknowledged by the user
	pub all_acked: bool,
}

/// Method: `POST`
///
/// Acknowledges a list of disclosures for the application.
pub fn ACKNOWLEDGE_APPLICATION_DISCLOSURES(application_id: &ApplicationId) -> String {
	format!("/applications/{}/disclosures", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct AcknowledgeApplicationDisclosuresRequest {
	/// The disclosures to acknowledge for the user
	pub disclosures: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AcknowledgeApplicationDisclosuresResponse {
	/// The disclosures that have been acknowledged by the user
	pub disclosures: Vec<i64>,
}

/// Method: `GET`
///
/// Requires the MANAGE_GUILD permission.
///
/// An application is considered attached to a guild if the [application's guild_id field](https://docs.discord.food/resources/application#application-object) is set to the guild's ID.
///
/// Returns a list of [application](https://docs.discord.food/resources/application#application-object) objects attached to the given guild ID.
pub fn GET_GUILD_APPLICATIONS(
	query: &GetGuildApplicationsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/applications{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildApplicationsQueryParams {
	/// The type of applications to return
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ApplicationType>,
	/// Whether to include team information for owned applications (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_team: Option<bool>,
	/// The ID of the channel to filter by (TODO: what the fuck does this do)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<ChannelId>,
}

pub type GetGuildApplicationsResponse = Vec<Application>;

/// Method: `POST`
///
/// Reports a game not detected and tracked to Discord.
///
/// Returns an unverified application object on success.
pub const REPORT_UNVERIFIED_APPLICATION: &str = "/unverified-applications";

#[derive(Serialize, Deserialize)]
pub struct ReportUnverifiedApplicationRequest {
	/// The version of the report (currently 3)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub report_version: Option<u8>,
	/// The name of the application (2-100 characters)
	pub name: ArrayString<100>,
	/// The MD5 hash of the application's icon (32 characters)
	pub icon: ArrayString<32>,
	/// The operating system the application is found on
	pub os: OperatingSystemDesktopType,
	/// The executable of the application (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub executable: Option<ArrayString<1024>>,
	/// The publisher of the application (2-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publisher: Option<ArrayString<100>>,
	/// The distributor of the application SKU
	#[serde(skip_serializing_if = "Option::is_none")]
	pub distributor_application: Option<ApplicationDistributor>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportUnverifiedApplicationResponse {
	/// The name of the application
	pub name: String,
	/// The unique hash of the application
	pub hash: ImageHash,
	/// The missing data for the application
	pub missing_data: Vec<ApplicationMissingDataType>,
}

/// Method: `POST`
///
/// Uploads an unverified application's icon to Discord.
///
/// Returns a 204 empty response on success.
pub const UPLOAD_UNVERIFIED_APPLICATION_ICON: &str = "/unverified-applications/icons";

/// Method: `GET`
///
/// Returns a list of [application role connection](https://docs.discord.food/resources/application#application-role-connection-object) objects for the user.
pub const GET_USER_APPLICATION_ROLE_CONNECTIONS: &str = "/users/@me/applications/role-connections";

pub type GetUserApplicationRoleConnectionsResponse = Vec<ApplicationRoleConnection>;

/// Method: `GET`
///
/// This endpoint is only usable with an OAuth2 access token with the role_connections.write scope for the application specified in the path.
///
/// Returns an [application role connection](https://docs.discord.food/resources/application#application-role-connection-object) object for the user, without optional fields.
pub fn GET_USER_APPLICATION_ROLE_CONNECTION(application_id: &ApplicationId) -> String {
	format!("/users/@me/applications/{}/role-connection", application_id)
}

pub type GetUserApplicationRoleConnectionResponse = ApplicationRoleConnection;

/// Method: `PUT`
///
/// This endpoint is only usable with an OAuth2 access token with the role_connections.write scope for the application specified in the path.
///
/// Updates an application's role connection for the user.
///
/// Returns the updated [application role connection](https://docs.discord.food/resources/application#application-role-connection-object) object on success.
pub fn MODIFY_USER_APPLICATION_ROLE_CONNECTION(application_id: &ApplicationId) -> String {
	format!("/users/@me/applications/{}/role-connection", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyUserApplicationRoleConnectionRequest {
	/// The vanity name of the platform a bot has connected (max 50 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform_name: Option<ArrayString<50>>,
	/// The username on the platform a bot has connected (max 100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform_username: Option<ArrayString<100>>,
	/// Object mapping [application role connection metadata](https://docs.discord.food/resources/application#application-role-connection-metadata-object) keys to their string-ified value (max 100 characters) for the user on the platform a bot has connected
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<HashMap<ArrayString<50>, ArrayString<100>>>,
}

pub type ModifyUserApplicationRoleConnectionResponse = ApplicationRoleConnection;

/// Method: `GET`
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a list of [activity link](https://docs.discord.food/resources/application#activity-link-object) objects for the given application ID.
pub fn GET_APPLICATION_MANAGED_LINKS(application_id: &ApplicationId) -> String {
	format!("/applications/{}/managed-links/", application_id)
}

pub type GetApplicationManagedLinksResponse = Vec<ActivityLink>;

/// Method: `POST`
///
/// Creates a new activity managed link.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns an [activity link](https://docs.discord.food/resources/application#activity-link-object) object on success.
pub fn CREATE_APPLICATION_MANAGED_LINK(application_id: &ApplicationId) -> String {
	format!("/applications/{}/managed-links/", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationManagedLinkRequest {
	/// A custom id for the activity link (1-256 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_id: Option<Option<ArrayString<256>>>,
	/// The description of the activity link (1-64 characters)
	pub description: ArrayString<64>,
	/// The activity link asset
	pub image: ImageHash,
	/// The title of the activity link (1-32 characters)
	pub title: ArrayString<32>,
}

pub type CreateApplicationManagedLinkResponse = ActivityLink;

/// Method: `GET`
///
/// Returns an [activity link](https://docs.discord.food/resources/application#activity-link-object) object for the given ID.
pub fn GET_APPLICATION_MANAGED_LINK(
	application_id: &ApplicationId,
	link_id: &ActivityLinkId,
) -> String {
	format!("/applications/{}/managed-links/{}", application_id, link_id)
}

pub type GetActivityManagedLinkResponse = ActivityLink;

/// Method: `PATCH`
///
/// Updates the specified activity link for the given application ID.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns an [activity link](https://docs.discord.food/resources/application#activity-link-object) object on success.
pub fn UPDATE_APPLICATION_MANAGED_LINK(
	application_id: &ApplicationId,
	link_id: &ActivityLinkId,
) -> String {
	format!("/applications/{}/managed-links/{}", application_id, link_id)
}

pub type UpdateActivityManagedLinkResponse = ActivityLink;

/// Method: `DELETE`
///
/// Deletes the specified activity link for the given application ID.
///
/// User must be the owner of the application or developer of the current team.
///
/// Returns a 204 empty response on success.
pub fn DELETE_APPLICATION_MANAGED_LINK(
	application_id: &ApplicationId,
	link_id: &ActivityLinkId,
) -> String {
	format!("/applications/{}/managed-links/{}", application_id, link_id)
}

/// Method: `POST`
///
/// Supports OAuth2 for authentication
///
/// Creates a new activity quick link.
///
/// When using OAuth2, quick links can be only created for the application that the access token belongs to.
///
/// Returns an [activity link](https://docs.discord.food/resources/application#activity-link-object) object on success.
pub fn CREATE_APPLICATION_QUICK_LINK(application_id: &ApplicationId) -> String {
	format!("/applications/{}/quick-links/", application_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationQuickLinkRequest {
	/// A custom id for the activity link (1-256 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_id: Option<Option<ArrayString<256>>>,
	/// The description of the activity link (1-64 characters)
	pub description: ArrayString<64>,
	/// The activity link asset
	pub image: ImageHash,
	/// The title of the activity link (1-32 characters)
	pub title: ArrayString<32>,
}

pub type CreateApplicationQuickLinkResponse = ActivityLink;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns an [activity link](https://docs.discord.food/resources/application#activity-link-object) object for the given ID.
pub fn GET_APPLICATION_QUICK_LINK(
	application_id: &ApplicationId,
	link_id: &ActivityLinkId,
) -> String {
	format!("/applications/{}/quick-links/{}", application_id, link_id)
}
