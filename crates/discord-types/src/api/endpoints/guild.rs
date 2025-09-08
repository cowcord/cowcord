use std::collections::HashMap;
use std::num::{NonZeroU8, NonZeroU16};

use arrayvec::{ArrayString, ArrayVec};
use hex::Hex;
use serde::{Deserialize, Serialize};

use crate::api::audit_log::PartialRole;
use crate::api::channel::{Channel, PartialChannel};
use crate::api::entitlements::Entitlement;
use crate::api::guild::{
	Ban,
	ExplicitContentFilterLevel,
	GameActivity,
	Guild,
	GuildJoinRequest,
	GuildJoinRequestStatus,
	GuildMember,
	GuildMemberFlags,
	GuildMemberUnusualDmActivity,
	GuildWidget,
	GuildWidgetImageStyleOption,
	GuildWidgetSettings,
	MemberFilter,
	MemberPaginationFilter,
	MemberSortType,
	MemberVerification,
	MemberVerificationFormField,
	MessageNotificationLevel,
	MfaLevel,
	MutableGuildFeatures,
	NewMemberAction,
	NewMemberActionProgress,
	NewMemberWelcome,
	NewMemberWelcomeMessage,
	Onboarding,
	OnboardingMode,
	OnboardingPrompt,
	PartialGuild,
	PremiumGuildSubscription,
	ResourceChannel,
	Role,
	RoleColors,
	RoleConnectionConfiguration,
	StudentHubGuild,
	SupplementalGuildMember,
	SystemChannelFlags,
	UserGuild,
	VerificationLevel,
	WelcomeScreen,
	WelcomeScreenChannel,
};
use crate::api::integrations::IntegrationApplication;
use crate::api::users::ProfileMetadata;
use crate::common::id::{
	ApplicationId,
	ChannelId,
	EmojiId,
	GuildId,
	GuildJoinRequestId,
	ProfileEffectId,
	RoleId,
	SkuId,
	UserId,
};
use crate::common::image::ImageHash;
use crate::common::locale::Locale;
use crate::common::timestamp::Timestamp;
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `guilds` scope
///
/// This endpoint returns 200 guilds by default, which is the maximum number of guilds a non-bot user can join.
/// Therefore, pagination is not needed in order to get a list of the users' guilds, and all parameters are optional.
///
/// Returns a list of [user guild](https://docs.discord.food/resources/guild#user-guild-object) objects representing the guilds the current user is a member of.
pub fn GET_USER_GUILDS(query: &GetUserGuildsQueryParams) -> String {
	format!("/users/@me/guilds{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetUserGuildsQueryParams {
	/// Get guilds before this guild ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<GuildId>,
	/// Get guilds after this guild ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<GuildId>,
	/// Max number of guilds to return
	/// (1-200, default 200)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// Whether to include approximate member and presence counts
	/// (default false)
	///
	/// For OAuth2 requests, this parameter requires the additional `guilds.members.read` scope.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_counts: Option<bool>,
}

pub type GetUserGuildsResponse = Vec<UserGuild>;

/// Method: `GET`
///
/// Returns a list of partial [guild](https://docs.discord.food/resources/guild#guild-object) objects representing [non-previewable guilds](https://docs.discord.food/resources/guild#guild-previewing) the current user has pending join requests for.
pub const GET_JOIN_REQUEST_GUILDS: &str = "/users/@me/join-request-guilds";

pub type GetJoinRequestGuildsResponse = Vec<Guild>;

/// Method: `DELETE`
///
/// Leaves the given guild ID.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Delete](https://docs.discord.food/topics/gateway-events#guild-delete) and a [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway event.
pub fn LEAVE_GUILD(guild_id: &GuildId) -> String {
	format!("/users/@me/guilds/{}", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct LeaveGuildRequest {
	/// Whether the user is lurking in the guild (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lurking: Option<bool>,
}

/// Method: `POST`
///
/// Creates a new guild.
///
/// If not specified, the below parameters use defaults from the 2TffvPucqHkN [guild template](https://docs.discord.food/resources/guild-template#guild-template-object).
///
/// Returns a [guild](https://docs.discord.food/resources/guild#guild-object) object on success.
/// Fires a [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create) Gateway event.
pub const CREATE_GUILD: &str = "/guilds";

#[derive(Serialize, Deserialize)]
pub struct CreateGuildRequest {
	/// The name of the guild
	/// (2-100 characters, excluding trailing and leading whitespace)
	pub name: String,
	/// The description for the guild
	/// (max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<300>>>,
	/// The main voice region ID of the guild
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<Option<String>>,
	/// The guild's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The verification level required for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verification_level: Option<Option<VerificationLevel>>,
	/// Default message notification level for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_message_notifications: Option<Option<MessageNotificationLevel>>,
	/// Whose messages are scanned and deleted for explicit content in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub explicit_content_filter: Option<Option<ExplicitContentFilterLevel>>,
	/// The preferred locale of the guild
	/// (default "en-US")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_locale: Option<Option<Locale>>,
	/// Roles in the new guild
	///
	/// The first member of the array is used to change properties of the guild's default (@everyone) role.
	/// If you are trying to bootstrap a guild with additional roles, keep this in mind.
	/// Additionally, the required `id` field within each role object is an integer placeholder, and will be replaced by the API upon consumption.
	/// Its purpose is to allow you to overwrite a role's permissions in a channel when also passing in channels with the channels array.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Option<Vec<PartialRole>>>,
	/// Channels in the new guild
	///
	/// The `id` field within each channel object may be set to an integer placeholder, and will be replaced by the API upon consumption.
	/// Its purpose is to allow you to create `GUILD_CATEGORY` channels by setting the `parent_id` field on any children to the category's `id` field.
	/// Category channels must be listed before any children.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channels: Option<Option<Vec<PartialChannel>>>,
	/// The ID of the guild's AFK channel; this is where members in voice idle for longer than afk_timeout are moved
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_channel_id: Option<Option<ChannelId>>,
	/// The AFK timeout of the guild (one of 60, 300, 900, 1800, 3600, in seconds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_timeout: Option<Option<u16>>,
	/// The ID of the channel where system event messages, such as member joins and premium subscriptions (boosts), are posted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_id: Option<Option<ChannelId>>,
	/// The flags that limit system event messages
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_flags: Option<Option<SystemChannelFlags>>,
	/// The template code that inspired this guild, used for analytics
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_template_code: Option<Option<String>>,
	/// Whether the new guild will only be accessible for Discord employees
	///
	/// Adds the `INTERNAL_EMPLOYEE_ONLY` guild feature, making the server only available for Discord employees. Only settable by Discord employees.
	#[cfg(feature = "employee")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub staff_only: Option<bool>,
}

pub type CreateGuildResponsee = Guild;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a [guild](https://docs.discord.food/resources/guild#guild-object) object for the given guild ID.
pub fn GET_GUILD(
	query: &GetGuildQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildQueryParams {
	/// Whether to include approximate member and presence counts
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_counts: Option<bool>,
}

pub type GetGuildResponse = Guild;

/// Method: `GET`
///
/// If the user is not in the guild, the guild must be discoverable.
///
/// Returns a partial [guild](https://docs.discord.food/resources/guild#guild-object) object for the given guild ID.
pub fn GET_GUILD_BASIC(guild_id: &GuildId) -> String {
	format!("/guilds/{}/basic", guild_id)
}

pub type GetGuildBasicResponse = PartialGuild;

/// Method: `GET`
///
/// If the user is not in the guild, the guild must be discoverable.
///
/// Returns a partial [guild](https://docs.discord.food/resources/guild#guild-object) object for the given guild ID with all partial fields.
pub fn GET_GUILD_PREVIEW(guild_id: &GuildId) -> String {
	format!("/guilds/{}/preview", guild_id)
}

pub type GetGuildPreviewResponse = PartialGuild;

/// Method: `PATCH`
///
/// Valid MFA code is required for some actions
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Modifies a guild's settings.
///
/// Returns the updated [guild](https://docs.discord.food/resources/guild#guild-object) object on success.
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD(guild_id: &GuildId) -> String {
	format!("/guilds/{}", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildRequest {
	/// The name of the guild
	/// (2-100 characters, excluding trailing and leading whitespace)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The guild's icon
	///
	/// animated icons are only shown when the guild has the `ANIMATED_ICON` feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The guild's banner
	///
	/// banners are only shown when the guild has the `BANNER` feature,
	/// animated banners are only shown when the guild has the `ANIMATED_BANNER` feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<ImageHash>>,
	/// The guild's home header, used in new member welcome
	///
	/// home headers are only shown when the guild has the `BANNER` feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub home_header: Option<Option<ImageHash>>,
	/// The guild's invite splash
	///
	/// splashes are only shown when the guild has the `INVITE_SPLASH` feature
	#[serde(skip_serializing_if = "Option::is_none")]
	pub splash: Option<Option<ImageHash>>,
	/// The guild's discovery splash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discovery_splash: Option<Option<ImageHash>>,
	/// The user ID of the guild's owner (must be the current owner)
	///
	/// if the current owner has an email address associated with their account and does not have MFA enabled, code must be provided
	#[serde(skip_serializing_if = "Option::is_none")]
	pub owner_id: Option<UserId>,
	/// The guild ownership transfer code
	///
	/// This value can be obtained by requesting a verification code with the [`GET_GUILD_OWNERSHIP_TRANSFER_CODE`] endpoint
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/// The description for the guild
	/// (max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<300>>>,
	/// The main voice region ID of the guild
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<Option<String>>,
	/// The ID of the guild's AFK channel
	///
	/// this is where members in voice idle for longer than `afk_timeout` are moved
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_channel_id: Option<Option<ChannelId>>,
	/// The AFK timeout of the guild (one of 60, 300, 900, 1800, 3600, in seconds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub afk_timeout: Option<u16>,
	/// The verification level required for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verification_level: Option<VerificationLevel>,
	/// Default message notification level for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_message_notifications: Option<MessageNotificationLevel>,
	/// Whose messages are scanned and deleted for explicit content in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
	/// Mutable guild features
	#[serde(skip_serializing_if = "Option::is_none")]
	pub features: Option<Vec<MutableGuildFeatures>>,
	/// The ID of the channel where system event messages, such as member joins and premium subscriptions (boosts), are posted
	///
	/// Setting this to `1` will implicitly create a new `#moderator-only` channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_id: Option<Option<ChannelId>>,
	/// The flags that limit system event messages
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_channel_flags: Option<SystemChannelFlags>,
	/// The ID of the channel where community guilds display rules and/or guidelines
	///
	/// Setting this to `1` will implicitly create a new `#rules` channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rules_channel_id: Option<Option<ChannelId>>,
	/// The ID of the channel where admins and moderators of community guilds receive notices from Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub public_updates_channel_id: Option<Option<ChannelId>>,
	/// The ID of the channel where admins and moderators of community guilds receive safety alerts from Discord
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safety_alerts_channel_id: Option<Option<ChannelId>>,
	/// The preferred locale of the guild; used in discovery and notices from Discord
	/// (default "en-US")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_locale: Option<Locale>,
	/// Whether the guild has the premium (boost) progress bar enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub premium_progress_bar_enabled: Option<bool>,
}

pub type ModifyGuildResponse = Guild;

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies the guild's [MFA requirement](https://docs.discord.food/resources/guild#mfa-level) for administrative actions within the guild.
///
/// User must be the owner.
///
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD_MFA_LEVEL(guild_id: &GuildId) -> String {
	format!("/guilds/{}/mfa", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildMfaLevelRequest {
	/// Required MFA level for administrative actions within the guild
	pub level: MfaLevel,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildMfaLevelResponse {
	/// Required MFA level for administrative actions within the guild
	pub level: MfaLevel,
}

/// Method: `PUT`
///
/// Sends a verification code to the guild owner's email address to initiate the guild ownership transfer process.
///
/// User must be the owner.
///
/// This endpoint should only be used when the current owner has an email address associated with their account and does not have MFA enabled.
/// If the owner does not have an email address associated with their account (e.g. a bot) or has MFA enabled, a code is not required to transfer ownership.
///
/// Returns a `204` empty response on success.
pub fn GET_GUILD_OWNERSHIP_TRANSFER_CODE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/pincode", guild_id)
}

/// Method: `DELETE`
///
/// Valid MFA code is required for some actions
///
/// Deletes a guild permanently.
/// User must be the owner.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Delete](https://docs.discord.food/topics/gateway-events#guild-delete) Gateway event.
pub fn DELETE_GUILD(guild_id: &GuildId) -> String {
	format!("/guilds/{}", guild_id)
}

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// This endpoint is not usable by user accounts and is restricted according to whether the `GUILD_MEMBERS` [Privileged Intent](https://docs.discord.food/topics/gateway#privileged-intents) is enabled for the application.
///
/// Returns a list of [guild member](https://docs.discord.food/resources/guild#guild-member-object) objects that are members of the guild.
#[cfg(feature = "non-user")]
pub fn GET_GUILD_MEMBERS(
	query: &GetGuildMembersQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/members{}", guild_id, query.to_string_query())
}

#[cfg(feature = "non-user")]
#[derive(Serialize, Deserialize)]
pub struct GetGuildMembersQueryParams {
	/// Max number of members to return
	/// (1-1000, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU16>,
	/// Get members after this member ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<UserId>,
}

#[cfg(feature = "non-user")]
pub type GetGuildMembersResponse = Vec<GuildMember>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// This endpoint is not usable by user accounts.
///
/// Functionally identical to the [Request Guild Members](https://docs.discord.food/topics/gateway-events#request-guild-members) Gateway Opcode.
///
/// Returns a list of [guild member](https://docs.discord.food/resources/guild#guild-member-object) objects whose username or nickname contains a provided string.
#[cfg(feature = "non-user")]
pub fn QUERY_GUILD_MEMBERS(
	query: &QueryGuildMembersQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/members/search{}",
		guild_id,
		query.to_string_query()
	)
}

#[cfg(feature = "non-user")]
#[derive(Serialize, Deserialize)]
pub struct QueryGuildMembersQueryParams {
	/// Query to match username(s) and nickname(s) against
	pub query: String,
	/// Max number of members to return
	/// (1-1000, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU16>,
}

#[cfg(feature = "non-user")]
pub type QueryGuildMembersRequest = Vec<GuildMember>;

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// This endpoint utilizes Elasticsearch to power results.
/// This means that while it is very powerful, it's also tricky to use and reliant on the index, meaning results may not be immediately available for a recently-joined member.
///
/// If the guild you are searching is not yet indexed, the endpoint will return a 202 accepted response. The response body will not contain any search results, and will look similar to an error response:
/// ```json
/// {
///   "message": "Index not yet available. Try again later",
///   "code": 110000,
///   "documents_indexed": 0,
///   "retry_after": 15
/// }
/// ```
///
/// You should retry the request after the timeframe specified in the `retry_after` field.
/// If the `retry_after` field is `0`, you should retry the request after a short delay.
/// See [the unavailable resources section](https://docs.discord.food/topics/rate-limits#unavailable-resources) for more information.
///
/// Returns [supplemental guild member](https://docs.discord.food/resources/guild#supplemental-guild-member-object) objects
/// containing [guild member](https://docs.discord.food/resources/guild#guild-member-object) objects that match a specified query.
pub fn SEARCH_GUILD_MEMBERS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/members-search", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct SearchGuildMembersRequest {
	/// Max number of members to return
	/// (1-1000, default 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU16>,
	/// The sorting algorithm to use
	/// (default `JOINED_AT_DESC`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort: Option<MemberSortType>,
	/// The filter criteria to match against members using OR logic
	#[serde(skip_serializing_if = "Option::is_none")]
	pub or_query: Option<MemberFilter>,
	/// The filter criteria to match against members using AND logic
	#[serde(skip_serializing_if = "Option::is_none")]
	pub and_query: Option<MemberFilter>,
	/// Get members before this member
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<MemberPaginationFilter>,
	/// Get members after this member
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<MemberPaginationFilter>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchGuildMembersResponse {
	/// The ID of the guild searched
	pub guild_id: GuildId,
	/// The resulting members
	pub members: Vec<SupplementalGuildMember>,
	/// The number of results returned
	pub page_result_count: u32,
	/// The total number of results found
	pub total_result_count: u32,
}

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a list of [supplemental guild member](https://docs.discord.food/resources/guild#supplemental-guild-member-object) objects including join source information for the given user IDs.
pub fn GET_GUILD_MEMBERS_SUPPLEMENTAL(guild_id: &GuildId) -> String {
	format!("/guilds/{}/members/supplemental", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildMembersSupplementalRequest {
	/// The user IDs to fetch supplemental guild member information for (max 200)
	pub users: Vec<UserId>,
}

pub type GetGuildMembersSupplemental = Vec<SupplementalGuildMember>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a list of [guild member unusual DM activity](https://docs.discord.food/resources/guild#guild-member-unusual-dm-activity-structure) objects representing the members that have ever had unusual DM activity.
pub fn GET_GUILD_MEMBERS_WITH_UNUSUAL_DM_ACTIVITY(
	query: &GetGuildMembersWithUnusualDmActivityQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/members/unusual-dm-activity{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildMembersWithUnusualDmActivityQueryParams {
	/// Max number of members to return
	/// (max 1000, default 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// Get members after this member ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<UserId>,
}

pub type GetGuildMembersWithUnusualDmActivityResponse = Vec<GuildMemberUnusualDmActivity>;

/// Method: `GET`
///
/// Supports OAuth2 for authentication with the `guilds.members.read` scope
///
/// Returns the [guild member](https://docs.discord.food/resources/guild#guild-member-object) object for the current user in the specified guild.
pub fn GET_CURRENT_GUILD_MEMBER(guild_id: &GuildId) -> String {
	format!("/users/@me/guilds/{}/member", guild_id)
}

pub type GetCurrentGuildMemberResponse = GuildMember;

/// Method: `GET`
///
/// Returns a [guild member](https://docs.discord.food/resources/guild#guild-member-object) object for the specified user.
pub fn GET_GUILD_MEMBER(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}

pub type GetGuildMemberResponse = GuildMember;

/// Method: `PUT`
///
/// Adds the current user to the guild.
///
/// The guild must be discoverable.
///
/// For guilds with [member verification](https://docs.discord.food/resources/guild#member-verification-object) enabled,
/// this endpoint will default to adding new members as pending in the [guild member](https://docs.discord.food/resources/guild#guild-member-object) object.
/// Members that are pending will have to complete member verification before they become full members that can talk.
///
/// For guilds with [previewing disabled](https://docs.discord.food/resources/guild#guild-previewing),
/// the return type will instead be a partial [guild](https://docs.discord.food/resources/guild#guild-object) object with the extra fields below.
///
/// If the user is not a member of the guild, returns a [guild](https://docs.discord.food/resources/guild#guild-object) object with the extra fields below.
/// Otherwise, returns a `204` empty response.
/// May fire a [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create),
/// [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add),
/// and/or [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway event.
pub fn JOIN_GUILD(
	query: &JoinGuildQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/members/@me{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct JoinGuildQueryParams {
	/// Whether the user will lurk the guild
	/// (default false)
	///
	/// Lurking a guild allows the user to receive Gateway events for the guild without being a full member for the lifetime of the Gateway session.
	/// This is useful for previewing the guild before joining.
	///
	/// Lurking requires the `PREVIEW_ENABLED` guild feature.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lurker: Option<bool>,
	/// The session ID to lurk with, required for lurking
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
	/// The analytics location the request initiated from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	/// The unique identifier for the current guild discovery recommendations
	/// (client-generated UUID as a hexadecimal string)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "hex::as_str")]
	pub recommendation_load_id: Option<Hex>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinGuildResponse {
	#[serde(flatten)]
	pub guild: PartialGuild,
	#[serde(flatten)]
	pub extra_fields: Option<JoinGuildResponseExtraFields>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinGuildResponseExtraFields {
	/// Whether the user should be shown the guild's member verification form
	#[serde(skip_serializing_if = "Option::is_none")]
	pub show_verification_form: Option<bool>,
	/// The guild's welcome screen, shown to new members when joining the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub welcome_screen: Option<WelcomeScreen>,
}

/// Method: `PUT`
///
/// Supports OAuth2 for authentication with the `guilds.join` scope
///
/// Adds a user to the guild, provided you have a valid OAuth2 access token for the user with the `guilds.join scope`.
///
/// For guilds with [member verification](https://docs.discord.food/resources/guild#member-verification-object) enabled, this endpoint will default to adding new members as pending in the [guild member](https://docs.discord.food/resources/guild#guild-member-object) object.
/// Members that are pending will have to complete member verification before they become full members that can talk.
/// Note that this endpoint ignores whether [guild previewing](https://docs.discord.food/resources/guild#guild-previewing) is enabled and will always join the user as a member.
///
/// This endpoint is not usable by user accounts.
/// The Authorization header must be a bot token (belonging to the same application used for authorization), and the bot must be a member of the guild with the `CREATE_INSTANT_INVITE` permission.
///
/// Returns the joined [guild member](https://docs.discord.food/resources/guild#guild-member-object) object
/// or a `204` empty response (if the user is already a member of the guild) on success.
/// May fire a [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add)
/// and/or [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway event.
#[cfg(feature = "non-user")]
pub fn ADD_GUILD_MEMBER(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}

#[cfg(feature = "non-user")]
#[derive(Serialize, Deserialize)]
pub struct AddGuildMemberRequest {
	/// An OAuth2 access token granted with the `guilds.join` to the bot's application for the user you want to add to the guild
	///
	/// permission required to set this: `CREATE_INSTANT_INVITE`
	pub access_token: String,
	/// The guild-specific nickname of the member
	/// (1-32 characters)
	///
	/// permission required to set this: `MANAGE_NICKNAMES`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<Option<ArrayString<32>>>,
	/// The role IDs assigned to this member
	///
	/// permission required to set this: `MANAGE_ROLES`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<RoleId>>,
	/// Whether the member is muted in voice channels
	///
	/// permission required to set this: `MUTE_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute: Option<bool>,
	/// Whether the user is deafened in voice channels
	///
	/// permission required to set this: `DEAFEN_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deaf: Option<bool>,
	/// The member's flags (only `BYPASSES_VERIFICATION` can be set)
	///
	/// for guilds with member verification enabled, assigning the `BYPASSES_VERIFICATION` guild member flag will add the user as a full member (pending is false in the member object).
	///
	/// permission(s) required to set this: `MANAGE_GUILD` *or* (`MODERATE_MEMBERS` and `KICK_MEMBERS` and `BAN_MEMBERS`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<GuildMemberFlags>,
}

#[cfg(feature = "non-user")]
pub type AddGuildMemberResponse = GuildMember;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies attributes of a guild member.
///
/// User must be a member of the guild.
///
/// Returns the updated [guild member](https://docs.discord.food/resources/guild#guild-member-object) object on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn MODIFY_GUILD_MEMBER(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildMembersRequest {
	/// The guild-specific nickname of the member
	/// (1-32 characters)
	///
	/// permission required to set this: `MANAGE_NICKNAMES`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<Option<ArrayString<32>>>,
	/// The role IDs assigned to this member
	///
	/// permission required to set this: `MANAGE_ROLES`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<RoleId>>,
	/// Whether the member is muted in voice channels
	///
	/// Requires the `member` to be connected to voice.
	///
	/// permission required to set this: `MUTE_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute: Option<bool>,
	/// Whether the user is deafened in voice channels
	///
	/// Requires the `member` to be connected to voice.
	///
	/// permission required to set this: `DEAFEN_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deaf: Option<bool>,
	/// The ID of the voice channel the user is connected to
	///
	/// Requires the `member` to be connected to voice.
	/// When moving members to channels, the current user must have permissions to both connect to the channel and have the `MOVE_MEMBERS` permission.
	/// If the `channel_id` is set to null, this will force the target user to be disconnected from voice.
	///
	/// permission required to set this: `MOVE_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<ChannelId>,
	/// When the user's timeout will expire and they will be able to communicate in the guild again (up to 28 days in the future)
	///
	/// Guild administrators cannot be timed out.
	/// If a member is timed out and becomes an administrator before their timeout expires, the timeout will no longer have an effect.
	///
	/// permission required to set this: `MODERATE_MEMBERS`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub communication_disabled_until: Option<Option<Timestamp>>,
	/// The member's flags (only `BYPASSES_VERIFICATION` can be set)
	///
	/// permission(s) required to set this: `MANAGE_GUILD` *or* (`MODERATE_MEMBERS` and `KICK_MEMBERS` and `BAN_MEMBERS`)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<GuildMemberFlags>,
}

pub type ModifyGuildMembersResponse = GuildMember;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies the current user's member in the guild.
///
/// Returns the updated [guild member](https://docs.discord.food/resources/guild#guild-member-object) object on success (including extra `bio` and `banner` fields).
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn MODIFY_CURRENT_GUILD_MEMBER(guild_id: &GuildId) -> String {
	format!("/guilds/{}/members/@me", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyCurrentGuildMemberRequest {
	/// The guild-specific nickname of the member
	/// (1-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<Option<ArrayString<32>>>,
	/// The member's guild avatar.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<Option<ImageHash>>,
	/// The ID of the member's guild avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_id: Option<Option<AvatarDecorationId>>,
	/// The SKU ID of the member's guild avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_sku_id: Option<Option<SkuId>>,
	/// The member's guild pronouns
	/// (max 40 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pronouns: Option<Option<ArrayString<40>>>,
	/// The member's guild bio.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<Option<String>>,
	/// The member's guild banner.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<ImageHash>>,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyCurrentGuildMemberResponse {
	/// Guild member data.
	#[serde(flatten)]
	pub guild_member: GuildMember,
	/// The member's guild bio.
	pub bio: Option<String>,
	/// The member's guild banner.
	pub banner: Option<ImageHash>,
}

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies the current user's member in the guild.
///
/// See [Modify Current Guild Member](https://docs.discord.food/resources/guild#modify-current-guild-member) for more information.
///
/// Returns the updated [guild member](https://docs.discord.food/resources/guild#guild-member-object) object on success (including extra bio and banner fields).
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
#[deprecated(note = "Replaced by MODIFY_CURRENT_GUILD_MEMBER.")]
pub fn MODIFY_CURRENT_GUILD_MEMBER_NICK(guild_id: &GuildId) -> String {
	format!("/guilds/{}/members/@me/nick", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyCurrentGuildMemberNickRequest {
	/// The guild-specific nickname of the member
	/// (1-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<Option<ArrayString<32>>>,
	/// The member's guild avatar.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<Option<ImageHash>>,
	/// The ID of the member's guild avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_id: Option<Option<AvatarDecorationId>>,
	/// The SKU ID of the member's guild avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_sku_id: Option<Option<SkuId>>,
	/// The member's guild pronouns
	/// (max 40 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pronouns: Option<Option<ArrayString<40>>>,
	/// The member's guild bio.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<Option<String>>,
	/// The member's guild banner.
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<ImageHash>>,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyCurrentGuildMemberNickResponse {
	/// Guild member data.
	#[serde(flatten)]
	pub guild_member: GuildMember,
	/// The member's guild bio.
	pub bio: Option<String>,
	/// The member's guild banner.
	pub banner: Option<ImageHash>,
}

/// Method: `PATCH`
///
/// Modifies the current user's profile in the guild.
///
/// Returns a [profile metadata](https://docs.discord.food/resources/user#profile-metadata-object) object on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn MODIFY_GUILD_MEMBER_PROFILE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/profile/@me", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildMemberProfileRequest {
	/// The member's guild pronouns
	/// (max 40 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pronouns: Option<Option<ArrayString<40>>>,
	/// The member's guild bio
	/// (max 190 characters)
	///
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<Option<ArrayString<190>>>,
	/// The member's guild banner
	///
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<ImageHash>>,
	/// The member's guild accent color as a hex integer
	///
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "hex::as_num")]
	pub accent_color: Option<Option<Hex>>,
	/// The member's two guild theme colors encoded as an array of integers representing hexadecimal color codes
	///
	/// can only be changed for premium (Nitro) users
	#[serde(skip_serializing_if = "Option::is_none")]
	// todo: check if this field is serialized as Vec<u8> instead of u64
	#[serde(with = "hex::as_num")]
	pub theme_colors: Option<Option<(Hex, Hex)>>,
	/// The member's guild profile popout animation particle type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub popout_animation_particle_type: Option<Option<PopoutAnimationParticleId>>,
	/// The member's guild profile emoji ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji_id: Option<Option<EmojiId>>,
	/// The member's guild profile effect ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile_effect_id: Option<Option<ProfileEffectId>>,
}

pub type ModifyGuildMemberProfileResponse = ProfileMetadata;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Adds a role to a [guild member](https://docs.discord.food/resources/guild#guild-member-object).
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn ADD_GUILD_MEMBER_ROLE(
	guild_id: &GuildId,
	user_id: &UserId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id)
}

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Removes a role from a [guild member](https://docs.discord.food/resources/guild#guild-member-object).
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn REMOVE_GUILD_MEMBER_ROLE(
	guild_id: &GuildId,
	user_id: &UserId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id)
}

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `KICK_MEMBERS` permission.
///
/// Removes a [member](https://docs.discord.food/resources/guild#guild-member-object) from a guild.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway event.
pub fn REMOVE_GUILD_MEMBER(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}

/// Method: `POST`
///
/// Adds the `DM_SETTINGS_UPSELL_ACKNOWLEDGED` [member flag](https://docs.discord.food/resources/guild#guild-member-flags) to the current user.
///
/// User must be a member of the guild.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn ACKNOWLEDGE_DM_SETTINGS_UPSELL_MODAL(guild_id: &GuildId) -> String {
	format!(
		"/users/@me/guilds/{}/member/ack-dm-upsell-settings",
		guild_id
	)
}

/// Method: `GET`
///
/// Requires the `BAN_MEMBERS` permission.
///
/// Returns a list of [ban](https://docs.discord.food/resources/guild#ban-object) objects for the guild.
pub fn GET_GUILD_BANS(
	query: &GetGuildBansQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/bans{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildBansQueryParams {
	/// Get bans before this user ID
	///
	/// Bans will always be returned in ascending order by user ID.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<UserId>,
	/// Get bans after this user ID
	///
	/// Bans will always be returned in ascending order by user ID.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<UserId>,
	/// Max number of bans to return
	/// (1-1000, default all or 1000)
	///
	/// Ban pagination for user accounts is currently optional.
	/// It must be explicitly opted into by specifying the limit query string parameter.
	/// Without specifying it, the before and after query string parameters will not work, and all bans will be returned by default.
	/// For bots, pagination is always enabled and 1000 bans are returned by default.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU16>,
}

pub type GetGuildBansResponse = Vec<Ban>;

/// Method: `GET`
///
/// Requires the `BAN_MEMBERS` permission.
///
/// Returns a list of [ban](https://docs.discord.food/resources/guild#ban-object) objects whose username or display name contains a provided string.
pub fn SEARCH_GUILD_BANS(
	query: &SearchGuildBansQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/bans/search{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct SearchGuildBansQueryParams {
	/// Query to match username(s) and display name(s) against
	/// (1-32 characters)
	pub query: ArrayString<32>,
	/// Max number of members to return
	/// (1-10, default 10)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU8>,
}

pub type SearchGuildBansResponse = Vec<Ban>;

/// Method: `GET`
///
/// Requires the `BAN_MEMBERS` permission.
///
/// Returns a [ban](https://docs.discord.food/resources/guild#ban-object) object for the given user.
pub fn GET_GUILD_BAN(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/bans/{}", guild_id, user_id)
}

pub type GetGuildBanResponse = Ban;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `BAN_MEMBERS` permission.
///
/// Creates a guild ban and optionally deletes previous messages sent by the banned user.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Ban Add](https://docs.discord.food/topics/gateway-events#guild-ban-add)
/// and optionally a [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway event.
pub fn CREATE_GUILD_BAN(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/bans/{}", guild_id, user_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildBanRequest {
	/// Number of days to delete messages for
	/// (0-7, default 0)
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delete_message_days: Option<u8>,
	/// Number of seconds to delete messages for
	/// (0-604800, default 0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delete_message_seconds: Option<u32>,
}

/// Method: `POST`
///
/// Valid MFA code is required for some actions
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires both the `BAN_MEMBERS` and `MANAGE_GUILD` permissions.
///
/// Create multiple guild bans and optionally delete previous messages sent by the banned users.
///
/// Fires multiple [Guild Ban Add](https://docs.discord.food/topics/gateway-events#guild-ban-add)
/// and optionally [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway events.
pub fn BULK_GUILD_BAN(guild_id: &GuildId) -> String {
	format!("/guilds/{}/bulk-ban", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct BulkGuildBanRequest {
	/// The user IDs to ban
	/// (max 200)
	pub user_ids: ArrayVec<UserId, 200>,
	/// Number of seconds to delete messages for
	/// (0-604800, default 0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delete_message_seconds: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct BulkGuildBanResponse {
	/// The user IDs that were successfully banned
	pub banned_users: Vec<UserId>,
	/// The user IDs that were not banned
	///
	/// A ban will fail if the user is already banned,
	/// the user has a higher role than the current user,
	/// the user is the owner of the guild,
	/// or the user is the current user.
	///
	/// If a bulk ban has no successful bans, the response will be an error.
	pub failed_users: Vec<UserId>,
}

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `BAN_MEMBERS` permission.
///
/// Removes the ban for a user.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Ban Remove](https://docs.discord.food/topics/gateway-events#guild-ban-remove) Gateway event.
pub fn DELETE_GUILD_BAN(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/bans/{}", guild_id, user_id)
}

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a list of [role](https://docs.discord.food/resources/guild#role-object) objects for the guild.
pub fn GET_GUILD_ROLES(guild_id: &GuildId) -> String {
	format!("/guilds/{}/roles", guild_id)
}

pub type GetGuildRolesResponse = Vec<Role>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a [role](https://docs.discord.food/resources/guild#role-object) object for the given role.
pub fn GET_GUILD_ROLE(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}", guild_id, role_id)
}

pub type GetGuildRoleResponse = Role;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a mapping of role IDs to their respective member counts.
pub fn GET_GUILD_ROLE_MEMBER_COUNTS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/roles/member-counts", guild_id)
}

pub type GetGuildRoleMemberCounts = HashMap<RoleId, u32>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// This endpoint does not return results for the default `@everyone` role.
///
/// Returns a list of member IDs that have the specified [role](https://docs.discord.food/resources/guild#role-object), up to a maximum of 100.
pub fn GET_GUILD_ROLE_MEMBERS(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}/member-ids", guild_id, role_id)
}

pub type GetGuildRoleMembers = Vec<UserId>;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Adds multiple [guild members](https://docs.discord.food/resources/guild#guild-member-object) to a [role](https://docs.discord.food/resources/guild#role-object).
///
/// Returns a mapping of member IDs to [guild member](https://docs.discord.food/resources/guild#guild-member-object) objects.
/// Fires multiple [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway events.
pub fn ADD_GUILD_ROLE_MEMBERS(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}/members", guild_id, role_id)
}

#[derive(Serialize, Deserialize)]
pub struct AddGuildRoleMembersRequest {
	/// The member IDs to assign the role to
	/// (max 30)
	pub member_ids: ArrayVec<UserId, 30>,
}

pub type AddGuildRoleMembersResponse = HashMap<UserId, GuildMember>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a list of [role connection rule](https://docs.discord.food/resources/guild#role-connection-rule-structure) objects representing the role connections for the guild.
pub fn GET_GUILD_ROLE_CONNECTIONS_CONFIGURATIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/roles/connections-configurations", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildRoleConnectionsConfigurationsResponseInner {
	/// The ID of the linkable role
	pub role_id: RoleId,
	/// The requirements for the linkable role
	pub rules: RoleConnectionConfiguration,
	/// The applications referenced in the rules
	pub applications: HashMap<ApplicationId, IntegrationApplication>,
}

pub type GetGuildRoleConnectionsConfigurationsResponse =
	Vec<GetGuildRoleConnectionsConfigurationsResponseInner>;

/// Method: `GET`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Returns a [role connection configuration](https://docs.discord.food/resources/guild#role-connection-configuration-object) object representing the role connection for the given role.
pub fn GET_GUILD_ROLE_CONNECTION_CONFIGURATION(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!(
		"/guilds/{}/roles/{}/connections/configuration",
		guild_id, role_id
	)
}

pub type GetGuildRoleConnectionConfigurationResponse =
	GetGuildRoleConnectionsConfigurationsResponseInner;

/// Method: `PUT`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Replaces the [role connection configuration](https://docs.discord.food/resources/guild#role-connection-configuration-object) for the given role.
///
/// Returns the updated [role connection configuration](https://docs.discord.food/resources/guild#role-connection-configuration-object) object on success.
pub fn MODIFY_GUILD_ROLE_CONNECTION_CONFIGURATION(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!(
		"/guilds/{}/roles/{}/connections/configuration",
		guild_id, role_id
	)
}

pub type ModifyGuildMemberConnectionConfigurationResponse =
	GetGuildRoleConnectionsConfigurationsResponseInner;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a [role connection configuration](https://docs.discord.food/resources/guild#role-connection-configuration-object) object with extra fields representing the user's eligibility to link the given role.
pub fn GET_GUILD_ROLE_CONNECTION_ELIGIBILITY(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!(
		"/guilds/{}/roles/{}/connections/eligibility",
		guild_id, role_id
	)
}

pub type GetGuildRoleConnectionEligibilityResponse = RoleConnectionConfiguration;

/// Method: `PUT`
///
/// Assigns an eligibile role connection to the current user.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn ASSIGN_GUILD_ROLE_CONNECTION(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}/connections/assign", guild_id, role_id)
}

/// Method: `POST`
///
/// Unassigns a role connection from the current user.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn UNASSIGN_GUILD_ROLE_CONNECTION(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!(
		"/guilds/{}/roles/{}/connections/unassign",
		guild_id, role_id
	)
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Creates a new role for the guild.
///
/// Returns the new [role](https://docs.discord.food/resources/guild#role-object) object on success.
/// Fires a [Guild Role Create](https://docs.discord.food/topics/gateway-events#guild-role-create) Gateway event.
pub fn CREATE_GUILD_ROLE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/roles", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildRoleRequest {
	/// The name of the role
	/// (max 100 characters, default "new role")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<Option<ArrayString<100>>>,
	/// The description for the role
	/// (max 90 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<90>>>,
	/// Integer representation of a hexadecimal color code for the role
	///
	/// If both `color` and `colors` are provided, the `color` field will be ignored.
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "hex::as_num")]
	pub color: Option<Option<Hex>>,
	/// The colors of the role encoded as an integer representation of hexadecimal color codes
	///
	/// Requires the `ENHANCED_ROLE_COLORS` guild feature.
	///
	/// If both `color` and `colors` are provided, the `color` field will be ignored.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub colors: Option<Option<RoleColors>>,
	/// Whether this role is pinned in the user listing (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hoist: Option<bool>,
	/// The role's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The role's unicode emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unicode_emoji: Option<Option<String>>,
	/// The permission bitwise value for the role
	/// (default @everyone permissions)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<Option<String>>,
	/// Whether this role is mentionable
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentionable: Option<bool>,
}

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Modifies the positions of a set of [role](https://docs.discord.food/resources/guild#role-object) objects for the guild. Requires the MANAGE_ROLES permission. Returns a list of all of the guild's [role](https://docs.discord.food/resources/guild#role-object) objects on success. Fires multiple [Guild Role Update](https://docs.discord.food/topics/gateway-events#guild-role-update) Gateway events.
///
/// This endpoint takes a JSON array of parameters in the following format:
pub fn MODIFY_GUILD_ROLE_POSITIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/roles", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildRolePositionsRequest {
	/// The ID of the role
	pub id: RoleId,
	/// Sorting position of the role
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<Option<u16>>,
}

pub type ModifyGuildRolePositionsResponse = Vec<Role>;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Modifies a guild role.
///
/// Returns the updated [role](https://docs.discord.food/resources/guild#role-object) on success.
/// Fires a [Guild Role Update](https://docs.discord.food/topics/gateway-events#guild-role-update) Gateway event.
pub fn MODIFY_GUILD_ROLE(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}", guild_id, role_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildRolesRequest {
	/// The name of the role
	/// (max 100 characters, default "new role")
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<Option<ArrayString<100>>>,
	/// The description for the role
	/// (max 90 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<90>>>,
	/// Integer representation of a hexadecimal color code for the role
	///
	/// If both `color` and `colors` are provided, the `color` field will be ignored.
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(with = "hex::as_num")]
	pub color: Option<Option<Hex>>,
	/// The colors of the role encoded as an integer representation of hexadecimal color codes
	///
	/// Requires the `ENHANCED_ROLE_COLORS` guild feature.
	///
	/// If both `color` and `colors` are provided, the `color` field will be ignored.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub colors: Option<Option<RoleColors>>,
	/// Whether this role is pinned in the user listing (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hoist: Option<bool>,
	/// The role's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
	/// The role's unicode emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unicode_emoji: Option<Option<String>>,
	/// The permission bitwise value for the role
	/// (default @everyone permissions)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<Option<String>>,
	/// Whether this role is mentionable
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentionable: Option<bool>,
}

pub type ModifyGuildRolesResponse = Role;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_ROLES` permission.
///
/// Deletes a guild role.
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Role Delete](https://docs.discord.food/topics/gateway-events#guild-role-delete) Gateway event.
pub fn DELETE_GUILD_ROLE(
	guild_id: &GuildId,
	role_id: &RoleId,
) -> String {
	format!("/guilds/{}/roles/{}", guild_id, role_id)
}

/// Method: `GET`
///
/// Requires both the `MANAGE_GUILD` and `KICK_MEMBERS` permissions.
///
/// Returns the number of members that would be removed in a prune operation.
///
/// By default, prune will not remove users with roles. You can optionally include specific roles in your prune by providing the include_roles parameter. Any inactive user that has a subset of the provided role(s) will be counted in the prune and users with additional roles will not.
pub fn GET_GUILD_PRUNE(
	query: &GetGuildPruneQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/prune{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildPruneQueryParams {
	/// Number of inactive days to count prune for
	/// (1-30, default 7)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub days: Option<u8>,
	/// Additional roles to include
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_roles: Option<Vec<RoleId>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildPruneResponse {
	/// The number of members that would be removed in a prune operation with the provided parameters
	pub pruned: u32,
}

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires both the `MANAGE_GUILD` and `KICK_MEMBERS` permissions.
///
/// Begins a prune operation.
///
/// For large guilds, it's recommended to set the `compute_prune_count` option to `false`, allowing the request to return before all members are pruned.
///
/// By default, prune will not remove users with roles. You can optionally include specific roles in your prune by providing the `include_roles` parameter.
/// Any inactive user that has a subset of the provided role(s) will be included in the prune and users with additional roles will not.
///
/// Fires multiple [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway events.
pub fn PRUNE_GUILD(guild_id: &GuildId) -> String {
	format!("/guilds/{}/prune", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct PruneGuildRequest {
	/// Number of inactive days to prune for
	/// (1-30, default 7)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub days: Option<u8>,
	/// Whether to wait for the prune to complete before responding
	/// (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compute_prune_count: Option<bool>,
	/// Additional roles to include
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_roles: Option<Vec<RoleId>>,
}

#[derive(Serialize, Deserialize)]
pub struct PruneGuildResponse {
	/// The number of members that were removed in the prune operation with the provided parameters
	///
	/// null if not computed
	pub pruned: Option<u32>,
}

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a [guild widget settings](https://docs.discord.food/resources/guild#guild-widget-settings-structure) object for the guild.
pub fn GET_GUILD_WIDGET_SETTINGS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/widget", guild_id)
}

pub type GetGuildWidgetSettingsResponse = GuildWidgetSettings;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Modifies the widget settings for the guild.
///
/// Returns the updated [guild widget settings](https://docs.discord.food/resources/guild#guild-widget-settings-structure) object on success.
pub fn MODIFY_GUILD_WIDGET_SETTINGS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/widget", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildWidgetSettingsRequest {
	/// Whether the widget is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// The channel ID that the widget will generate an invite to, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<Option<ChannelId>>,
}

pub type ModifyGuildWidgetSettingsResponse = GuildWidgetSettings;

/// Method: `GET`
///
/// Does not require authentication
///
/// The guild must have the widget enabled.
///
/// If a widget channel is set and a usable invite for it does not already exist, fetching the widget will create one. Subsequent calls will attempt to reuse the generated invite.
///
/// Returns a [guild widget](https://docs.discord.food/resources/guild#guild-widget-object) object for the given guild ID.
/// May fire an [Invite Create](https://docs.discord.food/topics/gateway-events#invite-create) Gateway event.
pub fn GET_GUILD_WIDGET(guild_id: &GuildId) -> String {
	format!("/guilds/{}/widget.json", guild_id)
}

pub type GetGuildWidgetResponse = GuildWidget;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a widget image PNG for the given guild ID. The guild must have the widget enabled.
pub fn GET_GUILD_WIDGET_IMAGE(
	query: &GetGuildWidgetImageQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/widget.png{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildWidgetImageQueryParams {
	/// Style of widget image returned (default shield )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub style: Option<GuildWidgetImageStyleOption>,
}

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// The guild must have the `VANITY_URL` or `GUILD_WEB_PAGE_VANITY_URL` feature.
///
/// Returns the vanity invite for the guild.
pub fn GET_GUILD_VANITY_INVITE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/vanity-url", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildVanityInviteResponse {
	/// The vanity invite code for the guild
	pub code: Option<String>,
	/// The number of times this invite has been used
	pub uses: u32,
}

/// Method: `PATCH`
///
/// Valid MFA code is required for some actions
///
/// The guild must have the VANITY_URL or GUILD_WEB_PAGE_VANITY_URL feature. Guilds without the VANITY_URL feature can only clear their vanity invite.
/// Requires both the MANAGE_GUILD and CREATE_INSTANT_INVITE permissions.
///
/// Modifies the vanity invite for the guild.
///
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD_VANITY_INVITE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/vanity-url", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildVanityInviteRequest {
	/// The vanity invite code for the guild
	/// (2-25 characters, alphanumeric and - )
	pub code: Option<ArrayString<25>>,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildVanityInviteResponse {
	/// The vanity invite code for the guild
	pub code: Option<String>,
	/// The number of times this invite has been used
	/// (now 0)
	pub uses: u8,
}

/// Method: `GET`
///
/// If the user is not in the guild, the guild must be discoverable or have [guild previewing](https://docs.discord.food/resources/guild#guild-previewing) disabled.
///
/// Returns the [member verification](https://docs.discord.food/resources/guild#member-verification-object) object for the guild if one is set.
pub fn GET_GUILD_MEMBER_VERIFICATION(
	query: &GetGuildMemberVerificationQueryParams,
	guild_id: &GuildId,
) -> String {
	format!(
		"/guilds/{}/member-verification{}",
		guild_id,
		query.to_string_query()
	)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildMemberVerificationQueryParams {
	/// Whether to include the guild object in the response
	/// (default false)
	///
	/// Requires that the user is not a member of the guild, and that the guild is not full.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_guild: Option<bool>,
	/// The invite code the verification is fetched from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite_code: Option<String>,
}

pub type GetGuildMemberVerificationResponse = MemberVerification;

/// Method: `PATCH`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Modifies the member verification for the guild.
///
/// Returns the updated [member verification](https://docs.discord.food/resources/guild#member-verification-object) object.
/// May fire a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD_MEMBER_VERIFICATION(guild_id: &GuildId) -> String {
	format!("/guilds/{}/member-verification", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildMemberVerificationRequest {
	/// Whether the member verification gate is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Questions for applicants to answer (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub form_fields: Option<Vec<MemberVerificationFormField>>,
	/// A description of what the guild is about.
	/// this can be different than the guild's description
	/// (max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<300>>>,
}

pub type ModifyGuildMemberVerificationResponse = MemberVerification;

/// Method: `GET`
///
/// Requires the `KICK_MEMBERS` permission.
///
/// Returns a list of join requests for the guild for manual approval.
pub fn GET_GUILD_JOIN_REQUESTS(
	query: &GetGuildJoinRequestsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/requests{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildJoinRequestsQueryParams {
	/// The status of the join requests to filter by
	pub status: GuildJoinRequestStatus,
	/// Max number of join requests to return
	/// (1-100, default 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<NonZeroU8>,
	/// Get join requests before this request ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before: Option<GuildJoinRequestId>,
	/// Get join requests after this request ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after: Option<GuildJoinRequestId>,
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildJoinRequestsResponse {
	/// The join requests for the guild
	///
	/// Join requests are omitted when retrieving requests with the `STARTED` status.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_join_requests: Option<Vec<GuildJoinRequest>>,
	/// The total number of join requests that match the query
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total: Option<u32>,
	/// The maximum number of join requests returned
	pub limit: u32,
}

/// Method: `GET`
///
/// Requires the `KICK_MEMBERS` permission if the request is not for the current user.
///
/// Returns a [guild join request](https://docs.discord.food/resources/guild#guild-join-request-object) object for the given request ID.
pub fn GET_GUILD_JOIN_REQUEST(guild_join_request_id: &GuildJoinRequestId) -> String {
	format!("/join-requests/{}", guild_join_request_id)
}

pub type GetGuildJoinRequestResponse = GuildJoinRequest;

/// Method: `GET`
///
/// Returns the remaining time until the current user can submit another join request for the guild.
pub fn GET_GUILD_JOIN_REQUEST_COOLDOWN(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests/@me/cooldown", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildJoinRequestCooldownResponse {
	/// How long (in seconds) the user has to wait until the current user can submit a join request
	pub cooldown: u32,
}

/// Method: `PUT`
///
/// Submits a request to join a guild.
///
/// Returns a partial [guild join request](https://docs.discord.food/resources/guild#guild-join-request-object) object on success.
/// Fires a [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create)
/// or [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update) Gateway event.
pub fn CREATE_GUILD_JOIN_REQUEST(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests/@me", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildjoinRequestRequest {
	/// The answered member verification questions
	///
	/// The `form_fields` array must contain all fields from the guild's `MemberVerification` object, with a populated `response` field for each `required` field.
	pub form_fields: Vec<MemberVerificationFormField>,
	/// When the member verification was last modified, same as version in The [Member Verification object](https://docs.discord.food/resources/guild#member-verification-object)
	pub version: Option<Timestamp>,
}

/// Method: `POST`
///
/// Resets the current user's join request for the guild.
///
/// Returns a partial [guild join request](https://docs.discord.food/resources/guild#guild-join-request-object) object on success.
/// Fires a [Guild Join Request Delete](https://docs.discord.food/topics/gateway-events#guild-join-request-delete)
/// and [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway event.
pub fn RESET_GUILD_JOIN_REQUEST(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests/@me", guild_id)
}

pub type ResetGuildJoinRequestResponse = GuildJoinRequest;

/// Method: `POST`
///
/// Acknowledges an approved join request for the current user.
///
/// Users can only acknowledge their own join requests, and only if the request is approved and is not already acknowledged (has a `last_seen` of `null`).
/// Upon acknowledgement, the join request is no longer considered active and will not be returned in the [Ready event](https://docs.discord.food/topics/gateway-events#ready).
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update) Gateway event.
#[deprecated(
	note = "This endpoint is for completons sake, please use ACKNOWLEDGE_USER_GUILD_JOIN_REQUEST"
)]
pub fn ACKNOWLEDGE_GUILD_JOIN_REQUEST(
	guild_id: &GuildId,
	guild_join_request_id: &GuildJoinRequestId,
) -> String {
	format!(
		"/guilds/{}/requests/{}/ack",
		guild_id, guild_join_request_id
	)
}

/// Method: `POST`
///
/// Acknowledges an approved join request for the current user.
///
/// Users can only acknowledge their own join requests, and only if the request is approved and is not already acknowledged (has a `last_seen` of `null`).
/// Upon acknowledgement, the join request is no longer considered active and will not be returned in the [Ready event](https://docs.discord.food/topics/gateway-events#ready).
///
/// Returns a `204` empty response on success.
/// Fires a [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update) Gateway event.
pub fn ACKNOWLEDGE_USER_GUILD_JOIN_REQUEST(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests/@me/ack", guild_id)
}

/// Method: `DELETE`
///
/// If the guild has [previewing disabled](https://docs.discord.food/resources/guild#guild-previewing), deletes the current user's join request.
/// Else, functions the same as [Reset Guild Join Request](https://docs.discord.food/resources/guild#reset-guild-join-request).
///
/// Returns a `204` empty response if deletion is successful
/// or a partial [guild join request](https://docs.discord.food/resources/guild#guild-join-request-object) object if the join request is reset.
/// Fires a [Guild Join Request Delete](https://docs.discord.food/topics/gateway-events#guild-join-request-delete)
/// and optionally a [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway event.
pub fn DELETE_GUILD_JOIN_REQUEST(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests/@me", guild_id)
}

pub type DeleteGuildJoinRequestResponse = GuildJoinRequest;

/// Method: `POST`
///
/// Requires the `KICK_MEMBERS` permission.
///
/// Creates or returns an existing private interview channel for the join request.
///
/// The created group DM channel will share the same ID as the join request and will be accessible by the join request user and the user who created the interview using this endpoint.
/// If a group DM channel already exists for the join request, the requestor will be added to it.
///
/// Returns a [group DM channel](https://docs.discord.food/resources/channel#channel-object) object on success.
/// Fires a [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update) and [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create) Gateway event.
pub fn CREATE_GUILD_JOIN_REQUEST_INTERVIEW(guild_join_request_id: &GuildJoinRequestId) -> String {
	format!("/join-requests/{}/interview", guild_join_request_id)
}

pub type CreateGuildJoinRequestInterviewResponse = Channel;

/// Method: `PATCH`
///
/// Requires the `KICK_MEMBERS` permission.
///
/// Accepts or denies a join request for the guild.
///
/// Returns a [guild join request](https://docs.discord.food/resources/guild#guild-join-request-object) object on success.
/// Fires a [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update)
/// and optionally a [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add)
/// or [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway event.
pub fn ACTION_GUILD_JOIN_REQUEST(
	guild_id: &GuildId,
	guild_join_request_id: &GuildJoinRequestId,
) -> String {
	format!("/guilds/{}/requests/id/{}", guild_id, guild_join_request_id)
}

#[derive(Serialize, Deserialize)]
pub struct ActionGuildJoinRequestRequest {
	/// The action to take on the join requests
	/// (only APPROVED and REJECTED can be used)
	pub action: GuildJoinRequestStatus,
	/// The reason for rejecting the join request
	/// (max 160 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rejection_reason: Option<Option<ArrayString<160>>>,
}

/// Method: `PATCH`
///
/// Same as `ACTION_GUILD_JOIN_REQUEST`, except this endpoint is keyed by the user ID instead of the guild join request ID.
pub fn ACTION_GUILD_JOIN_REQUEST_BY_USER(
	guild_id: &GuildId,
	user_id: &UserId,
) -> String {
	format!("/guilds/{}/requests/{}", guild_id, user_id)
}

#[derive(Serialize, Deserialize)]
pub struct ActionGuildJoinRequestByUserRequest {
	/// The action to take on the join requests
	/// (only APPROVED and REJECTED can be used)
	pub action: GuildJoinRequestStatus,
	/// The reason for rejecting the join request
	/// (max 160 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rejection_reason: Option<Option<ArrayString<160>>>,
}

/// Method: `PATCH`
///
/// Requires the `KICK_MEMBERS` permission.
///
/// Accepts or denies all pending join requests for the guild.
///
/// Returns a `204` empty response on success.
/// May fire multiple [Guild Join Request Update](https://docs.discord.food/topics/gateway-events#guild-join-request-update),
/// [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add),
/// and [Guild Member Remove](https://docs.discord.food/topics/gateway-events#guild-member-remove) Gateway events.
pub fn BULK_ACTION_GUILD_JOIN_REQUESTS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/requests", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct BulkActionGuildJoinRequestsRequest {
	/// The action to take on the join requests
	/// (only APPROVED and REJECTED can be used)
	pub action: GuildJoinRequestStatus,
}

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission if the welcome screen is not yet enabled, otherwise no permission is required.
///
/// Returns the [welcome screen](https://docs.discord.food/resources/guild#welcome-screen-object) object for the guild.
pub fn GET_GUILD_WELCOME_SCREEN(guild_id: &GuildId) -> String {
	format!("/guilds/{}/welcome-screen", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildWelcomeScreenRequest {
	/// Whether the welcome screen is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<Option<bool>>,
	/// The welcome message shown in the welcome screen
	/// (max 140 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<140>>>,
	/// The channels shown in the welcome screen (max 5)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub welcome_channels: Option<Option<Vec<WelcomeScreenChannel>>>,
}

pub type GetGuildWelcomeScreenResponse = WelcomeScreen;

/// Method: `GET`
///
/// Requires the MANAGE_GUILD permission if the feature is disabled, otherwise requires that the user is a member of the guild.
///
/// Returns the [onboarding](https://docs.discord.food/resources/guild#onboarding-object) object for the guild.
pub fn GET_GUILD_ONBOARDING(guild_id: &GuildId) -> String {
	format!("/guilds/{}/onboarding", guild_id)
}

pub type GetGuildOnboardingResponse = Onboarding;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the MANAGE_GUILD permission.
///
/// Modifies the onboarding configuration of the guild.
///
/// Onboarding enforces constraints when enabled.
/// These constraints are that there must be at least 7 default channels and at least 5 of them must allow sending messages to the default (@everyone) role.
/// The mode field modifies what is considered when enforcing these constraints.
///
/// Returns the updated [onboarding](https://docs.discord.food/resources/guild#onboarding-object) object.
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD_ONBOARDING(guild_id: &GuildId) -> String {
	format!("/guilds/{}/onboarding", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildOnboardingRequest {
	/// The prompts shown during onboarding and in community customization
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompts: Option<Vec<OnboardingPrompt>>,
	/// The channel IDs that members get opted into automatically
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_channel_ids: Option<Vec<ChannelId>>,
	/// Whether onboarding is enabled in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// The current criteria mode for onboarding
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<OnboardingMode>,
}

pub type ModifyGuildOnboardingResponse = Onboarding;

/// Method: `GET`
///
/// Requires the MANAGE_GUILD permission if the feature is disabled, otherwise no permission is required.
///
/// If it exists, returns the [new member welcome](https://docs.discord.food/resources/guild#new-member-welcome-object) object for the guild.
/// Otherwise, returns a `204` empty response.
pub fn GET_GUILD_NEW_MEMBER_WELCOME(guild_id: &GuildId) -> String {
	format!("/guilds/{}/new-member-welcome", guild_id)
}

pub type GetGuildNewMemberWelcomeResponse = NewMemberWelcome;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the MANAGE_GUILD permission.
///
/// Modifies the guild's new member welcome configuration.
///
/// The new member welcome experience enforces constraints when enabled.
/// These constraints are that there must be at least 3 new member actions,
/// all referenced channels must be viewable by the default role,
/// and new member action channels with an [`action_type`](https://docs.discord.food/resources/guild#new-member-action-type) of `CHAT` must allow sending messages to the default role.
///
/// Returns the updated [new member welcome](https://docs.discord.food/resources/guild#new-member-welcome-object) object.
/// May fire a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update) Gateway event.
pub fn MODIFY_GUILD_NEW_MEMBER_WELCOME(guild_id: &GuildId) -> String {
	format!("/guilds/{}/new-member-welcome", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildNewMemberWelcomeRequest {
	/// Whether the new member welcome experience is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enabled: Option<bool>,
	/// Welcome message shown to new members of the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub welcome_message: Option<NewMemberWelcomeMessage>,
	/// Actions shown to new members of the guild
	/// (max 5)
	///
	/// Only the `channel_id`, `action_type`, and `title` fields are required.
	/// `icon` cannot be set.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_member_actions: Option<ArrayVec<NewMemberAction, 5>>,
	/// Read-only channels that provide resources for new members
	/// (max 7)
	///
	/// Only the `channel_id` and `title` fields are required.
	/// `icon` cannot be set.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resource_channels: Option<ArrayVec<ResourceChannel, 7>>,
}

pub type ModifyGuildNewMemberWelcomeResponse = NewMemberWelcome;

/// Method: `PATCH`
///
/// Requires the MANAGE_GUILD permission.
///
/// Modifies a new member action for the guild.
///
/// Returns the updated [new member action](https://docs.discord.food/resources/guild#new-member-action-structure) object on success.
pub fn MODIFY_GUILD_NEW_MEMBER_ACTION(
	guild_id: &GuildId,
	channel_id: &ChannelId,
) -> String {
	format!("/guilds/{}/new-member-actions/{}", guild_id, channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildNewMemberActionRequest {
	/// The new member action's icon (max 10000 KiB)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
}

pub type ModifyGuildNewMemberActionResponse = NewMemberAction;

/// Method: `PUT`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the MANAGE_GUILD permission.
///
/// Modifies a resource channel for the guild.
///
/// All parameters to this endpoint are optional and nullable.
/// Omitting or setting a null value will set it to default.
///
/// Returns the updated [resource channel](https://docs.discord.food/resources/guild#resource-channel-structure) object on success.
pub fn MODIFY_GUILD_RESOURCE_CHANNEL(
	guild_id: &GuildId,
	channel_id: &ChannelId,
) -> String {
	format!("/guilds/{}/resource-channels/{}", guild_id, channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildResourceChannelRequest {
	/// The resource channel's icon (max 10000 KiB)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<ImageHash>>,
}

pub type ModifyGuildResourceChannelResponse = ResourceChannel;

/// Method: `GET`
///
/// If it exists, returns a [new member actions progress](https://docs.discord.food/resources/guild#new-member-actions-progress-object) object for the user in the guild, representing the user's progress towards completing the new member actions.
/// Otherwise, returns a 204 empty response.
pub fn GET_GUILD_NEW_MEMBER_ACTIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/new-member-actions", guild_id)
}

pub type GetGuildNewMemberActionsResponse = NewMemberActionProgress;

/// Method: `POST`
///
/// Completes a new member action for the user in the guild.
///
/// Returns the updated [new member actions progress](https://docs.discord.food/resources/guild#new-member-actions-progress-object) object on success.
/// May fire a [Guild Member Update](https://docs.discord.food/topics/gateway-events#guild-member-update) Gateway event.
pub fn COMPLETE_GUILD_NEW_MEMBER_ACTION(
	guild_id: &GuildId,
	channel_id: &ChannelId,
) -> String {
	format!("/guilds/{}/new-member-action/{}", guild_id, channel_id)
}

pub type CompleteGuildNewMemberActionsResponse = NewMemberActionProgress;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns up to 20 of the top most played games for the guild.
pub fn GET_GUILD_TOP_GAMES(guild_id: &GuildId) -> String {
	format!("/guilds/{}/top-games", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildTopGamesResponse {
	/// The top games played in the guild
	pub top_games: Vec<GameActivity>,
}

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Returns a list of [premium guild subscription](https://docs.discord.food/resources/guild#premium-guild-subscription-object) objects for the guild.
pub fn GET_PREMIUM_GUILD_SUBSCRIPTIONS(guild_id: &GuildId) -> String {
	format!("/guilds/{}/premium/subscriptions", guild_id)
}

pub type GetPremiumGuildSubscriptionsResponse = Vec<PremiumGuildSubscription>;

/// Method: `GET`
///
/// User must be a member of the guild.
///
/// Powerups are special SKUs that can be purchased using premium subscriptions (boosts) to enhance the guild's features.
///
/// Returns a list of [entitlement](https://docs.discord.food/resources/entitlement#entitlement-object) objects representing the guild's applied powerups.
pub fn GET_GUILD_POWERUPS(
	query: &GetGuildPowerupsQueryParams,
	guild_id: &GuildId,
) -> String {
	format!("/guilds/{}/powerups{}", guild_id, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetGuildPowerupsQueryParams {
	/// no clue what this means
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_ends_at: Option<bool>,
}

pub type GetGuildPowerupsResponse = Vec<Entitlement>;

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Adds the given powerup SKU to the guild.
///
/// Returns a 204 empty response on success.
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update)
/// and [Guild Powerup Entitlements Create](https://docs.discord.food/topics/gateway-events#guild-powerup-entitlements-create) Gateway event.
pub fn ADD_GUILD_POWERUP(
	guild_id: &GuildId,
	sku_id: &SkuId,
) -> String {
	format!("/guilds/{}/skus/{}", guild_id, sku_id)
}

/// Method: `DELETE`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Removes the given powerup SKU from the guild.
///
/// Returns a 204 empty response on success.
/// Fires a [Guild Update](https://docs.discord.food/topics/gateway-events#guild-update)
/// and [Guild Powerup Entitlements Delete](https://docs.discord.food/topics/gateway-events#guild-powerup-entitlements-delete) Gateway event.
pub fn REMOVE_GUILD_POWERUP(
	guild_id: &GuildId,
	sku_id: &SkuId,
) -> String {
	format!("/guilds/{}/skus/{}", guild_id, sku_id)
}

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Checks if the user is eligible to join the Discord Admin Community through the guild.
pub fn GET_ADMIN_COMMUNITY_ELIGIBILITY(guild_id: &GuildId) -> String {
	format!("/guilds/{}/admin-server-eligibility", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct GetAdminCommunityEligibilityResponse {
	/// Whether the user is eligible to join the Discord Admin Community through the guild
	pub eligible_for_admin_server: bool,
}

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Joins the Discord Admin Community through the guild.
///
/// Returns the joined [guild](https://docs.discord.food/resources/guild#guild-object) object on success.
/// Fires [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create),
/// [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add),
/// and optionally [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway events.
pub fn JOIN_ADMIN_COMMUNITY(guild_id: &GuildId) -> String {
	format!("/guilds/{}/join-admin-server", guild_id)
}

pub type JoinAdminCommunityResponse = Guild;

/// Method: `POST`
///
/// Queries the student hubs for the given email.
pub const QUERY_STUDENT_HUBS: &str = "/guilds/automations/email-domain-lookup";

#[derive(Serialize, Deserialize)]
pub struct QueryStudentHubsRequest {
	/// The email to lookup
	/// (max 320 characters)
	pub email: ArrayString<320>,
	/// Whether to email the user a code to verify their student status
	/// (default false)
	///
	/// `use_verification_code` should always be set to `true` as the old behavior is deprecated and results in an email asking the user to update their client to join the student hub.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_verification_code: Option<bool>,
	/// Whether to return a list of guilds for the email instead of picking the first one
	/// (default false)
	///
	/// `allow_multiple_guilds` should always be set to `true` as the resultant guild ID is required for the Join Student Hub endpoint.
	/// If `allow_multiple_guilds` is set to `false`, the `guild_id` parameter is ignored and an email is sent for the first guild found.
	/// If `allow_multiple_guilds` is set to `true`, a second request must be made with a `guild_id` provided to send the email.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allow_multiple_guilds: Option<bool>,
	/// The guild ID to email a verification code for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryStudentHubsResponse {
	/// Whether a student hub was found and verification code was sent to the email provided
	pub has_matching_guild: bool,
	/// The guilds found for the email provided
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guilds_info: Option<Vec<StudentHubGuild>>,
}

/// Method: `POST`
///
/// Verifies the student status of the user and joins them to the student hub guild.
///
/// May fire [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create),
/// [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add),
/// and optionally [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create) Gateway events.
pub const JOIN_STUDENT_HUB: &str = "/guilds/automations/email-domain-lookup/verify-code";

#[derive(Serialize, Deserialize)]
pub struct JoinStudentHubRequest {
	/// The email to verify
	/// (max 320 characters)
	pub email: ArrayString<320>,
	/// The ID of the student hub being joined
	pub guild_id: GuildId,
	/// The verification code sent to the email
	/// (8 characters)
	pub code: ArrayString<8>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinStudentHubResponse {
	/// Whether the user successfully joined the student hub guild
	pub joined: bool,
	/// The student hub guild the user joined
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild: Option<Guild>,
}

/// Method: `POST`
///
/// Signs up the user for the student hub waitlist.
///
/// The user will get an email and system message when a student hub is created for their email domain.
pub const JOIN_STUDENT_HUB_WAITLIST: &str = "/hub-waitlist/signup";

#[derive(Serialize, Deserialize)]
pub struct JoinStudentHubWaitlistRequest {
	/// The email to sign up for the waitlist
	/// (max 320 characters)
	pub email: ArrayString<320>,
	/// The name of the school the user is attending
	/// (3-20 characters)
	pub school: ArrayString<20>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinStudentHubWaitlistResponse {
	/// The email that was signed up for the waitlist
	pub email: String,
	/// The domain of the email that was signed up for the waitlist
	pub email_domain: String,
	/// The name of the school the user is attending
	pub school: String,
	/// The ID of the user that signed up for the waitlist
	pub user_id: UserId,
}
