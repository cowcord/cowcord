#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::api::data::channel::Channel;
use crate::api::data::guild::{
	Guild,
	GuildMember,
	GuildMemberUnusualDmActivity,
	MemberFilter,
	MemberPagination,
	Role,
	SupplementalGuildMember,
	UserGuild,
	WelcomeScreen,
};
use crate::api::types::{CdnUri, Snowflake, timestamp::Timestamp};
use crate::utils::request::to_string_query;

/// Type: get
/// support OAuth2 for auth
pub fn GET_USER_GUILDS_ENDPOINT(query: GetUserGuildsRequest) -> String {
	format!("/users/@me/guilds{}", to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetUserGuildsRequest {
	pub before:      Snowflake,
	pub after:       Snowflake,
	pub limit:       u8,
	/// requires guilds.members.read scope for OAuth2 requests
	pub with_counts: bool,
}

pub type GetUserGuildsResponse = Vec<UserGuild>;

/// Type: get
pub const GET_JOIN_REQUEST_GUILDS_ENDPOINT: &str = "/users/@me/join-request-guilds";

pub type GetJoinRequestGuildsResponse = Vec<Guild>;

/// Type: delete
/// support OAuth2 for auth
pub fn LEAVE_GUILD_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/users/@me/guilds/{}", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct LeaveGuildRequest {
	pub lurking: bool,
}

/// Type: get
pub const CREATE_GUILD_ENDPOINT: &str = "/guilds";

#[derive(Serialize)]
#[serde(default)]
/// If not specified, the params use defaults from the 2TffvPucqHkN guild template.
pub struct CreateGuildRequest {
	pub name:                          String,
	pub description:                   Option<String>,
	pub region:                        Option<String>,
	pub icon:                          Option<CdnUri>,
	/// https://docs.discord.sex/resources/guild#verification-level
	pub verification_level:            Option<u8>,
	/// https://docs.discord.sex/resources/guild#message-notification-level
	pub default_message_notifications: Option<u8>,
	/// https://docs.discord.sex/resources/guild#explicit-content-filter-level
	pub explicit_content_filter:       Option<u8>,
	pub preferred_locale:              Option<String>,
	pub roles:                         Option<Vec<Role>>,
	pub channels:                      Option<Vec<Channel>>,
	pub afk_channel_id:                Option<Snowflake>,
	pub afk_timeout:                   Option<u8>,
	pub system_channel_id:             Option<Snowflake>,
	pub system_channel_flags:          Option<u8>,
	pub guild_template_code:           Option<String>,
	pub staff_only:                    bool,
}

pub type CreateGuildResponse = Guild;

/// Type: get
pub fn GET_GUILD_ENDPOINT(
	guild_id: Snowflake,
	query: GetGuildRequest,
) -> String {
	format!("/guilds/{}{}", guild_id, to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetGuildRequest {
	pub with_counts: bool,
}

pub type GetGuildResponse = Guild;

/// Type: get
pub fn GET_GUILD_BASIC_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/basic", guild_id)
}

pub type GetGuildBasicResponse = Guild;

/// Type: get
pub fn GET_GUILD_PREVIW_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/preview", guild_id)
}

pub type GetGuildPreviewResponse = Guild;

/// Type: patch
/// requires mfa
/// supports the X-Audit-Log-Reason header
/// requires MANAGE_GUILD permission
pub fn MODIFY_GUILD_ENDPOINT(guild_id: Snowflake) -> String { format!("/guilds/{}", guild_id) }

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyGuildRequest {
	pub name:                          String,
	pub icon:                          Option<CdnUri>,
	pub banner:                        Option<CdnUri>,
	pub home_header:                   Option<CdnUri>,
	pub splash:                        Option<CdnUri>,
	pub discovery_splash:              Option<CdnUri>,
	pub owner_id:                      Snowflake,
	/// obtained by requesting a verification code with the GET_GUILD_OWNERSHIP_TRANSFER_CODE_ENDPOINT endpoint
	pub code:                          String,
	pub description:                   Option<String>,
	#[deprecated]
	pub region:                        Option<String>,
	pub afk_channel_id:                Option<Snowflake>,
	pub afk_timeout:                   u16,
	/// https://docs.discord.sex/resources/guild#verification-level
	pub verification_level:            u8,
	/// https://docs.discord.sex/resources/guild#message-notification-level
	pub default_message_notifications: u8,
	/// https://docs.discord.sex/resources/guild#explicit-content-filter-level
	pub explicit_content_filter:       u8,
	/// https://docs.discord.sex/resources/guild#mutable-guild-features
	pub features:                      Vec<String>,
	pub system_channel_id:             Option<Snowflake>,
	/// https://docs.discord.sex/resources/guild#system-channel-flags
	pub system_channel_flags:          u64,
	pub rules_channel_id:              Option<Snowflake>,
	pub public_updates_channel_id:     Option<Snowflake>,
	pub safety_alerts_channel_id:      Option<Snowflake>,
	pub preferred_locale:              String,
	pub premium_progress_bar_enabled:  bool,
}

pub type ModifyGuildResponse = Guild;

/// Type: post
/// requires mfa
/// supports the X-Audit-Log-Reason header
pub fn MODIFY_GUILD_MFA_LEVEL_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/mfa", guild_id)
}

#[derive(Serialize)]
pub struct ModifyGuildMfaLevelRequest {
	/// https://docs.discord.sex/resources/guild#mfa-level
	pub level: u8,
}

#[derive(Deserialize)]
pub struct ModifyGuildMfaLevelResponse {
	/// https://docs.discord.sex/resources/guild#mfa-level
	pub level: u8,
}

/// Type: put
/// if owner doesnt have an accosiaed email or has mfa enabled then this code isnt needed
pub fn GET_GUILD_OWNERSHIP_TRANFER_CODE_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/pincode", guild_id)
}

/// Type: delete
/// requires mfa
pub fn DELET_GUILD_ENDPOINT(guild_id: Snowflake) -> String { format!("/guilds/{}", guild_id) }

/// Type: get
/// not usable by user accounts
pub fn GET_GUILD_MEMBERS_ENDPOINT(
	guild_id: Snowflake,
	query: GetGuildMembersRequest,
) -> String {
	format!("/guilds/{}/members{}", guild_id, to_string_query(&query))
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetGuildMembersRequest {
	pub limit: u16,
	pub after: Snowflake,
}

pub type GetGuildMembersResponse = Vec<GuildMember>;

/// Type: get
/// basically identical to the Request Guild Members Gateway Opcode
/// not usable by user accounts
pub fn QUERY_GUILD_MEMBERS_ENDPOINT(
	guild_id: Snowflake,
	query: QueryGuildMembersRequest,
) -> String {
	format!(
		"/guilds/{}/members/search{}",
		guild_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct QueryGuildMembersRequest {
	pub query: String,
	pub limit: u16,
}

pub type QueryGuildMembersResponse = Vec<GuildMember>;

/// Type: post
/// requires MANAGE_GUILD permission
pub fn SEARCH_GUILD_MEMBERS_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/members-search", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct SearchGuildMembersRequest {
	pub limit:     u16,
	pub sort:      u8,
	pub or_query:  MemberFilter,
	pub and_query: MemberFilter,
	pub before:    MemberPagination,
	pub after:     MemberPagination,
}

#[derive(Deserialize)]
pub struct SearchGuildMembersResponse {
	pub guild_id:           Snowflake,
	pub members:            Vec<SupplementalGuildMember>,
	pub page_result_count:  u32,
	pub total_result_count: u32,
}

/// Type: post
/// requires MANAGE_GUILD permission
pub fn GET_GUILD_MEMBERS_SUPPLEMENTAL_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/members/supplemental", guild_id)
}

#[derive(Serialize)]
pub struct GetGuildMembersSupplementalRequest {
	pub users: Vec<Snowflake>,
}

pub type GetGuildMembersSupplementalResponse = Vec<SupplementalGuildMember>;

/// Type: get
pub fn GET_GUILD_MEMBERS_WITH_UNUSUAL_DM_ACTIVITY_ENDPOINT(
	guild_id: Snowflake,
	query: GetGuildMembersWithUnusualDmActivityRequest,
) -> String {
	format!(
		"/guilds/{}/members/unusual-dm-activity{}",
		guild_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct GetGuildMembersWithUnusualDmActivityRequest {
	pub limit: u16,
	pub after: Snowflake,
}

pub type GetGuildMembersWithUnusualDmActivityResponse = Vec<GuildMemberUnusualDmActivity>;

/// Type: get
pub fn GET_GUILD_MEMBER_ENDPOINT(
	guild_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}

pub type GetGuildMemberResponse = GuildMember;

/// Type: put
pub fn JOIN_GUILD_ENDPOINT(
	guild_id: Snowflake,
	query: JoinGuildRequest,
) -> String {
	format!(
		"/guilds/{}/members/@me{}",
		guild_id,
		to_string_query(&query)
	)
}

#[derive(Serialize)]
#[serde(default)]
pub struct JoinGuildRequest {}

#[derive(Deserialize)]
pub struct JoinGuildResponse {
	#[serde(flatten)]
	pub guild:                  Guild,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub show_verification_form: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub welcome_screen:         Option<WelcomeScreen>,
}

/// Type: put
/// supports OAuth2 of auth
pub fn ADD_GUILD_MEMBER_ENDPOINT(
	guild_id: Snowflake,
	user_id: Snowflake,
) -> String {
	format!("/guilds/{}/members/{}", guild_id, user_id)
}
