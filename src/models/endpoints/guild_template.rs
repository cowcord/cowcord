#![allow(non_snake_case)]

use serde::Serialize;

use crate::models::api_types::guild::Guild;
use crate::models::api_types::guild_template::GuildTemplate;
use crate::models::api_types::types::{CdnUri, Snowflake};

/// Type: get
///
/// doesnt require Authentication header
pub fn GET_GUILD_TEMPLATE_ENDPOINT(template_code: Snowflake) -> String {
	format!("/guilds/templates/{}", template_code)
}

pub type GetGuildTemplateResponse = GuildTemplate;

/// Type: post
pub fn USE_GUILD_TEMPLATE_ENDPOINT(template_code: Snowflake) -> String {
	format!("/guilds/templates/{}", template_code)
}

#[derive(Serialize)]
#[serde(default)]
pub struct UseGuildTemplateRequest {
	pub name: String,
	pub icon: CdnUri,
}

pub type UseGuildTemplateResponse = Guild;

/// Type: get
///
/// requires MANAGE_GUILD permission
pub fn GET_GUILD_TEMPLATES_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/templates", guild_id)
}

pub type GetGuildTemplatesResponse = Vec<GuildTemplate>;

/// Type: post
///
/// requires MANAGE_GUILD permission
pub fn CREATE_GUILD_TEMPLATES_ENDPOINT(guild_id: Snowflake) -> String {
	format!("/guilds/{}/templates", guild_id)
}

#[derive(Serialize)]
#[serde(default)]
pub struct CreateGuildTemplatesRequest {
	pub name:        String,
	pub description: Option<String>,
}

pub type CreateGuildTemplatesResponse = GuildTemplate;

/// Type: put
///
/// requires MANAGE_GUILD permission
pub fn SYNC_GUILD_TEMPLATES_ENDPOINT(
	guild_id: Snowflake,
	template_code: Snowflake,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

pub type SyncGuildTemplatesResponse = GuildTemplate;

/// Type: patch
///
/// requires MANAGE_GUILD permission
pub fn MODIFY_GUILD_TEMPLATES_ENDPOINT(
	guild_id: Snowflake,
	template_code: Snowflake,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

#[derive(Serialize)]
#[serde(default)]
pub struct ModifyGuildTemplatesRequest {
	pub name:        String,
	pub description: Option<String>,
}

pub type ModifyGuildTemplatesResponse = GuildTemplate;

/// Type: delete
///
/// requires MANAGE_GUILD permission
pub fn DELETE_GUILD_TEMPLATES_ENDPOINT(
	guild_id: Snowflake,
	template_code: Snowflake,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

pub type DeleteGuildTemplatesResponse = GuildTemplate;
