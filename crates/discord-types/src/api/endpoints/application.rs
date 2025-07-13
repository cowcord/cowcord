#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::application::{
	Application,
	ApplicationAsset,
	ApplicationDistributor,
	ApplicationInstallParams,
	ApplicationIntegrationTypeConfig,
	ApplicationProxyConfig,
	ApplicationProxyMap,
	ApplicationRoleConnection,
	DetectableApplication,
	EmbeddedActivityConfig,
	EmbeddedActivityPlatformConfig,
	ExternalAsset,
};
use crate::api::users::User;
use crate::utils::request::to_string_query;

/// Type: get
pub fn GET_APPLICATIONS_ENDPOINT(with_team_applications: bool) -> String {
	format!(
		"/applications?with_team_applications={}",
		with_team_applications
	)
}

/// Type: get
pub fn GET_APPLICATIONS_WITH_ASSETS_ENDPOINT(with_team_applications: bool) -> String {
	format!(
		"/applications-with-assets?with_team_applications={}",
		with_team_applications
	)
}

pub type GetApplicationsResponse = Vec<Application>;

/// Type: post
pub const CREATE_APPLICATION_ENDPOINT: &str = "/applications";

#[derive(Serialize)]
pub struct CreateApplicationRequest {
	pub name: String,
	/// https://docs.discord.food/resources/application#application-type
	pub r#type: u8,
	pub team_id: Snowflake,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<CdnUri>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<CdnUri>,
	/// https://docs.discord.food/resources/application#application-flags
	pub flags: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redirect_uris: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<String>,
}

pub type CreateApplicationResponse = Application;

/// Type: get
pub fn GET_APPLICATION_BY_ID_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}", application_id)
}

pub type GetApplicationByIdResponse = Application;

/// Type: get
///
/// Not usable by user accounts
pub const GET_CURRENT_APPLICATION_ENDPOINT: &str = "/applications/@me";

pub type GetCurrentApplicationResponse = Application;

/// Type: patch
pub fn MODIFY_APPLICATION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}", application_id)
}

#[derive(Serialize)]
pub struct ModifyApplicationRequest {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<CdnUri>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<CdnUri>,
	pub flags: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub developer_ids: Option<Vec<Snowflake>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publisher_ids: Option<Vec<Snowflake>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redirect_uris: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<String>,
	pub integration_public: bool,
	pub integration_require_code_grant: bool,
	#[deprecated]
	pub bot_public: bool,
	#[deprecated]
	pub bot_require_code_grant: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub terms_of_service_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub privacy_policy_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_connections_verification_url: Option<String>,
	pub interactions_endpoint_url: String,
	/// https://docs.discord.food/resources/application#application-interactions-version
	pub interactions_version: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_event_types: Option<Vec<String>>,
	/// https://docs.discord.food/resources/application#explicit-content-filter-level
	pub explicit_content_filter: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<ApplicationInstallParams>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<String>,
	pub integration_types_config: HashMap<u8, Option<ApplicationIntegrationTypeConfig>>,
	/// https://docs.discord.food/resources/application#application-discoverability-state
	pub discoverability_state: u8,
	/// https://docs.discord.food/resources/application#application-monetization-state
	pub monetization_state: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<i32>,
}

pub type ModifyApplicationResponse = Application;

/// Type: patch
///
/// Not usable by user accounts
pub const MODIFY_CURRENT_APPLICATION_ENDPOINT: &str = "/applications/@me";

#[derive(Serialize)]
pub struct ModifyCurrentApplicationRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<CdnUri>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<CdnUri>,
	pub flags: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rpc_origins: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_connections_verification_url: Option<String>,
	pub interactions_endpoint_url: String,
	/// https://docs.discord.food/resources/application#application-interactions-version
	pub interactions_version: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interactions_event_types: Option<Vec<String>>,
	/// https://docs.discord.food/resources/application#explicit-content-filter-level
	pub explicit_content_filter: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub install_params: Option<ApplicationInstallParams>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_install_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_participants: Option<i32>,
}

pub type ModifyCurrentApplicationResponse = Application;

/// Type: post
///
/// requires mfa
pub fn DELETE_APPLICATION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/delete", application_id)
}

/// Type: post
///
/// requires mfa
pub fn TRANSFER_APPLICATION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/transfer", application_id)
}

#[derive(Serialize)]
pub struct TransferApplicationRequest {
	pub team_id: Snowflake,
}

pub type TransferApplicationResponse = Application;

/// Type: post
///
/// requires mfa
pub fn RESET_APPLICATION_SECRET_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/reset", application_id)
}

#[derive(Deserialize)]
pub struct ResetApplicationSecretResponse {
	pub secret: String,
}

/// Type: post
///
/// requires mfa
#[deprecated]
pub fn CREATE_APPLICATION_BOT_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/bot", application_id)
}

#[derive(Deserialize)]
pub struct CreateApplicationBotResponse {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
}

/// Type: patch
pub fn MODIFY_APPLICATION_BOT_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/bot", application_id)
}

#[derive(Serialize)]
pub struct ModifyApplicationBotRequest {
	pub username: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<CdnUri>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<CdnUri>,
}

pub type ModifyApplicationBotResponse = User;

/// Type: post
///
/// requires mfa
pub fn RESET_APPLICATION_BOT_TOKEN_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/bot/reset", application_id)
}

#[derive(Deserialize)]
pub struct ResetApplicationBotTokenResponse {
	pub token: String,
}

/// Type: post
pub fn REQUEST_APPLICATION_GATEWAY_INTENTS_ENDPOINT(application_id: Snowflake) -> String {
	format!(
		"/applications/{}/request-additional-intents",
		application_id
	)
}

#[derive(Serialize)]
pub struct RequestApplicationGatewayIntentsRequest {
	pub application_description: String,
	/// https://docs.discord.food/resources/application#application-flags
	pub intents_flags_requested: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_use_case_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_use_case_supplemental_material_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_store_off_platform: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_retention: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_encrypted: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_opt_out_stored: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_presence_contact_deletion: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_use_case_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_use_case_supplemental_material_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_store_off_platform: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_retention: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_encrypted: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_guild_members_contact_deletion: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_use_case_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_use_case_supplemental_material_description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_store_off_platform: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_retention: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_encrypted: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_opt_out_stored: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_ai_training: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_public: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_location: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_privacy_policy_example: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intents_gateway_message_content_contact_deletion: Option<bool>,
}

/// Type: get
pub fn GET_APPLICATION_DISCOVERABILITY_STATE_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/discoverability-state", application_id)
}

#[derive(Deserialize)]
pub struct GetApplicationDiscoverabilityStateResponse {
	/// https://docs.discord.food/resources/application#application-discoverability-state
	pub discoverability_state: u8,
	/// https://docs.discord.food/resources/application#application-discovery-eligibility-flags
	pub discovery_eligibility_flags: u64,
	pub bad_commands: Vec<ApplicationCommand>,
}

/// Type: get
pub fn GET_EMBEDDED_ACTIVITIES_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/activities/shelf?guild_id={}", guild_id)
}

#[derive(Deserialize)]
pub struct GetEmbeddedActivitiesResponse {
	pub activities: Vec<EmbeddedActivityConfig>,
	pub applications: Vec<Application>,
	pub assets: HashMap<Snowflake, Vec<ApplicationAsset>>,
}

/// Type: post
pub fn SET_APPLICATION_EMBEDDABILITY_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/set-embedded", application_id)
}

#[derive(Serialize)]
pub struct SetApplicationEmbeddabilityRequest {
	pub embedded: bool,
}

/// Type: get
pub fn GET_APPLICATION_EMBEDDED_ACTIVITY_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/set-embedded-config", application_id)
}

pub type GetApplicationEmbeddedActivityConfigResponse = EmbeddedActivityConfig;

/// Type: patch
pub fn MODIFY_APPLICATION_EMBEDDED_ACTIVITY_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/set-embedded-config", application_id)
}

#[derive(Serialize)]
pub struct ModifyApplicationEmbeddedActivityConfigRequest {
	pub activity_preview_video_asset_id: Option<Snowflake>,
	pub supported_platforms: Option<Vec<String>>,
	/// https://docs.discord.food/resources/application#embedded-activity-orientation-lock-state-type
	pub default_orientation_lock_state: u8,
	/// https://docs.discord.food/resources/application#embedded-activity-orientation-lock-state-type
	pub tablet_default_orientation_lock_state: u8,
	pub requires_age_gate: bool,
	#[deprecated]
	pub free_period_starts_at: Option<Timestamp>,
	#[deprecated]
	pub free_period_ends_at: Option<Timestamp>,
	pub client_platform_config: HashMap<String, EmbeddedActivityPlatformConfig>,
	pub shelf_rank: u16,
}

pub type ModifyApplicationEmbeddedActivityConfigResponse = EmbeddedActivityConfig;

/// Type: get
pub fn GET_APPLICATION_PROXY_CONFIG_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/proxy-config", application_id)
}

pub type GetApplicationProxyConfigResponse = ApplicationProxyConfig;

/// Type: post
pub fn MODIFY_APPLICATION_PROXY_CONFIG_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/proxy-config", application_id)
}

#[derive(Serialize)]
pub struct ModifyApplicationProxyConfigRequest {
	pub url_map: ApplicationProxyMap,
}

pub type ModifyApplicationProxyConfigResponse = ApplicationProxyConfig;

/// Type: get
///
/// doesnt need Authentication header
pub fn GET_APPLICATION_ASSETS_ENDPOINT(
	application_id: Snowflake,
	no_cache: bool,
) -> String {
	format!(
		"/oauth2/applications/{}/assets?nocache={}",
		application_id, no_cache
	)
}

pub type GetApplicationAssetsResponse = Vec<ApplicationAsset>;

/// Type: post
pub fn CREATE_APPLICATION_ASSET_ENDPOINT(application_id: Snowflake) -> String {
	format!("/oauth2/applications/{}/assets", application_id)
}

#[derive(Serialize)]
pub struct CreateApplicationAssetRequest {
	pub name: String,
	/// https://docs.discord.food/resources/application#application-asset-type
	pub r#type: u8,
	pub image: CdnUri,
}

pub type CreateApplicationAssetResponse = ApplicationAsset;

/// Type: delete
pub fn DELETE_APPLICATION_ASSET_ENDPOINT(
	application_id: Snowflake,
	asset_id: Snowflake,
) -> String {
	format!(
		"/oauth2/applications/{}/assets/{}",
		application_id, asset_id
	)
}

/// Type: post
pub fn PROXY_APPLICATION_ASSETS_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/external-assets", application_id)
}

#[derive(Serialize)]
pub struct ProxyApplicationAssetsRequest {
	pub urls: Vec<String>,
}

pub type ProxyApplicationAssetsResponse = ExternalAsset;

/// Type: post
///
/// supports OAuth2 for auth
/// multipart/form-data NOT json
/// let form = reqwest::multipart::Form::new().file("file name", "path to file")?;
pub fn CREATE_APPLICATION_ATTATCHMENT_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/external-assets", application_id)
}

#[derive(Deserialize)]
pub struct CreateApplicationAttatchmentResponse {
	pub attachment: MessageAttachment,
}

/// Type: get
///
/// doesnt require Authentication header
pub const GET_DETECTABLE_APPLICATIONS_ENDPOINT: &str = "/applications/detectable";

pub type GetDetectableApplicationsResponse = Vec<DetectableApplication>;

/// Type: get
pub fn GET_PARTIAL_APPLICATIONS_ENDPOINT(application_ids: Vec<Snowflake>) -> String {
	format!("/applications/public?application_ids={:?}", application_ids)
}

pub type GetPartialApplicationsResponse = Vec<Application>;

/// Type: get
///
/// supports OAuth2 for auth
pub fn GET_PARTIAL_APPLICATION_ENDPOINT(
	application_id: Snowflake,
	with_guild: bool,
) -> String {
	format!(
		"/applications/{}/public?with_guild={}",
		application_id, with_guild
	)
}

pub type GetPartialApplicationResponse = Application;

/// Type: get
///
/// doesnt require Authentication header
pub fn GET_RICH_PRESENCE_APPLICATION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/rpc", application_id)
}

pub type GetRichPresenceApplicationResponse = Application;

/// Type: get
pub fn GET_APPLICATION_DISCLOSURES_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/disclosures", application_id)
}

#[derive(Deserialize)]
pub struct GetApplicationDisclosuresResponse {
	/// https://docs.discord.food/resources/application#application-disclosure-type
	pub disclosures: Vec<u8>,
	/// https://docs.discord.food/resources/application#application-disclosure-type
	pub acked_disclosures: Vec<u8>,
	pub all_acked: bool,
}

/// Type: post
pub fn ACKNOWLEDGE_APPLICATION_DISCLOSURES_ENDPOINT(application_id: Snowflake) -> String {
	format!("/applications/{}/disclosures", application_id)
}

#[derive(Serialize)]
pub struct AcknowledgeApplicationDisclsuresRequest {
	/// https://docs.discord.food/resources/application#application-disclosure-type
	pub disclosures: Vec<u8>,
}

#[derive(Deserialize)]
pub struct AcknowledgeApplicationDisclsuresResponse {
	/// https://docs.discord.food/resources/application#application-disclosure-type
	pub disclosures: Vec<u8>,
}

/// Type: get
pub fn GET_GUILD_APPLICATIONS_ENDPOINT(
	guild_id: Snowflake,
	query: GetGuildApplicationsRequest,
) -> String {
	format!(
		"/guilds/{}/applications{}",
		guild_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
pub struct GetGuildApplicationsRequest {
	/// https://docs.discord.food/resources/application#application-type
	pub r#type: u8,
	pub include_team: bool,
	pub channel_id: Snowflake,
}

pub type GetGuildApplicationsResponse = Vec<Application>;

/// Type: post
pub const REPORT_UNVERIFIED_APPLICATION_ENDPOINT: &str = "/unverified-applications";

#[derive(Serialize)]
pub struct ReportUnverifiedApplicationRequest {
	pub report_version: u8,
	pub name: String,
	pub icon: String,
	pub os: String,
	pub executable: String,
	pub publisher: String,
	pub distributor_application: ApplicationDistributor,
}

#[derive(Deserialize)]
pub struct ReportUnverifiedApplicationResponse {
	pub name: String,
	pub hash: String,
	/// https://docs.discord.food/resources/application#application-missing-data-type
	pub missing_data: Vec<String>,
}

/// Type: post
pub const UPLOAD_UNVERIFIED_APPLICATION_ICON_ENDPOINT: &str = "/unverified-applications/icons";

#[derive(Serialize)]
pub struct UploadUnverifiedApplicationIconRequest {
	pub application_name: String,
	pub application_hash: String,
	pub icon: CdnUri,
}

/// Type: get
pub const GET_USER_APPLICATION_ROLE_CONNECTIONS_ENDPOINT: &str =
	"/users/@me/applications/role-connections";

pub type GetUserApplicationRoleConnections = Vec<ApplicationRoleConnection>;

/// Type: get
///
/// only available via OAuth2 with role_connections.write scope
/// for the application specified in the path
pub fn GET_USER_APPLICATION_ROLE_CONNECTION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/users/@me/applications/{}/role-connection", application_id)
}

pub type GetUserApplicationRoleConnection = ApplicationRoleConnection;

/// Type: patch
///
/// only available via OAuth2 with role_connections.write scope
/// for the application specified in the path
pub fn MODIFY_USER_APPLICATION_ROLE_CONNECTION_ENDPOINT(application_id: Snowflake) -> String {
	format!("/users/@me/applications/{}/role-connection", application_id)
}

#[derive(Serialize)]
pub struct ModifyUserApplicationRoleConnectionRequest {
	pub platform_name: String,
	pub platform_username: String,
	/// mapping of https://docs.discord.food/resources/application#application-role-connection-metadata-object to stringified values
	pub metadata: HashMap<String, String>,
}

pub type ModifyUserApplicationRoleConnectionResponse = ApplicationRoleConnection;
