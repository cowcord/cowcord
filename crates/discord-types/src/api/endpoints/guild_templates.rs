use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::api::guild::Guild;
use crate::api::guild_templates::GuildTemplate;
use crate::common::id::GuildId;
use crate::common::image::ImageHash;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns a [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) object for the given code.
pub fn GET_GUILD_TEMPLATE(template_code: &str) -> String {
	format!("/guilds/templates/{}", template_code)
}

pub type GetGuildTemplateResponse = GuildTemplate;

/// Method: `POST`
///
/// Create a new guild based on a template.
///
/// Returns a [guild](https://docs.discord.food/resources/guild#guild-object) object on success.
/// Fires a [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create) Gateway event.
pub fn USE_GUILD_TEMPLATE(template_code: &str) -> String {
	format!("/guilds/templates/{}", template_code)
}

#[derive(Serialize, Deserialize)]
pub struct UseGuildTemplateRequest {
	/// The name of the guild (2-100 characters)
	pub name: String,
	/// 128x128 image for the guild's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<ImageHash>,
}

pub type UseGuildTemplateResponse = Guild;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns an array of [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) objects.
pub fn GET_GUILD_TEMPLATES(guild_id: &GuildId) -> String {
	format!("/guilds/{}/templates", guild_id)
}

pub type GetGuildTemplates = Vec<GuildTemplate>;

/// Method: `POST`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Creates a template for the guild.
///
/// Returns the created [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) object on success.
pub fn CREATE_GUILD_TEMPLATE(guild_id: &GuildId) -> String {
	format!("/guilds/{}/templates", guild_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildTemplateRequest {
	/// The name of the template
	/// (1-100 characters)
	pub name: ArrayString<100>,
	/// The description for the template
	/// (max 120 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<120>>>,
}

pub type CreateGuildTemplateResponse = GuildTemplate;

/// Method: `PUT`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Syncs the template to the guild's current state.
///
/// Returns the updated [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) object on success.
pub fn SYNC_GUILD_TEMPLATE(
	guild_id: &GuildId,
	template_code: &str,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

pub type SyncGuildTemplateResponse = GuildTemplate;

/// Method: `PATCH`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Modifies the template's metadata.
///
/// Returns the updated [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) object on success.
pub fn MODIFY_GUILD_TEMPLATE(
	guild_id: &GuildId,
	template_code: &str,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGuildTemplateRequest {
	/// The name of the template
	/// (1-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<ArrayString<100>>,
	/// The description for the template
	/// (max 120 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<ArrayString<120>>>,
}

pub type ModifyGuildTemplateResponse = GuildTemplate;

/// Method: `DELETE`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Deletes the template.
///
/// Returns the deleted [guild template](https://docs.discord.food/resources/guild-template#guild-template-object) object on success.
pub fn DELETE_GUILD_TEMPLATE(
	guild_id: &GuildId,
	template_code: &str,
) -> String {
	format!("/guilds/{}/templates/{}", guild_id, template_code)
}

pub type DeleteGuildTemplateResponse = GuildTemplate;
