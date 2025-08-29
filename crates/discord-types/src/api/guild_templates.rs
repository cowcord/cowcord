use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use crate::{
	api::{guild::PartialGuild, users::PartialUser},
	common::{id::{GuildId, UserId}, timestamp::Timestamp},
};

/// default guild template when creating a guild:
/// ```json
/// {
///   "code": "2TffvPucqHkN",
///   "name": "Blank Server",
///   "description": null,
///   "usage_count": 34729,
///   "creator_id": "268473310986240001",
///   "creator": {
///     "id": "268473310986240001",
///     "username": "discordapp",
///     "avatar": "f749bb0cbeeb26ef21eca719337d20f1",
///     "discriminator": "0",
///     "public_flags": 4325376,
///     "banner": null,
///     "accent_color": null,
///     "global_name": null,
///     "avatar_decoration_data": null,
///     "primary_guild": null
///   },
///   "created_at": "2020-04-17T20:59:35+00:00",
///   "updated_at": "2020-04-17T20:59:35+00:00",
///   "source_guild_id": "700811170902179862",
///   "serialized_source_guild": {
///     "name": "Blank Server",
///     "description": null,
///     "region": "us-west",
///     "verification_level": 0,
///     "default_message_notifications": 0,
///     "explicit_content_filter": 0,
///     "preferred_locale": "en-US",
///     "afk_channel_id": null,
///     "afk_timeout": 300,
///     "system_channel_id": 2,
///     "system_channel_flags": 0,
///     "roles": [
///       {
///         "id": 0,
///         "name": "@everyone",
///         "permissions": "2248329584430657",
///         "color": 0,
///         "hoist": false,
///         "mentionable": false,
///         "icon": null,
///         "unicode_emoji": null
///       }
///     ],
///     "channels": [
///       {
///         "id": 1,
///         "type": 4,
///         "name": "Text Channels",
///         "position": 0,
///         "topic": null,
///         "bitrate": 64000,
///         "user_limit": 0,
///         "nsfw": false,
///         "rate_limit_per_user": 0,
///         "parent_id": null,
///         "default_auto_archive_duration": null,
///         "permission_overwrites": [],
///         "available_tags": null,
///         "template": "",
///         "default_reaction_emoji": null,
///         "default_thread_rate_limit_per_user": null,
///         "default_sort_order": null,
///         "default_forum_layout": null,
///         "icon_emoji": null,
///         "theme_color": null
///       },
///       {
///         "id": 2,
///         "type": 0,
///         "name": "general",
///         "position": 0,
///         "topic": null,
///         "bitrate": 64000,
///         "user_limit": 0,
///         "nsfw": false,
///         "rate_limit_per_user": 0,
///         "parent_id": 1,
///         "default_auto_archive_duration": null,
///         "permission_overwrites": [],
///         "available_tags": null,
///         "template": "",
///         "default_reaction_emoji": null,
///         "default_thread_rate_limit_per_user": null,
///         "default_sort_order": null,
///         "default_forum_layout": null,
///         "icon_emoji": null,
///         "theme_color": null
///       },
///       {
///         "id": 3,
///         "type": 4,
///         "name": "Voice Channels",
///         "position": 0,
///         "topic": null,
///         "bitrate": 64000,
///         "user_limit": 0,
///         "nsfw": false,
///         "rate_limit_per_user": 0,
///         "parent_id": null,
///         "default_auto_archive_duration": null,
///         "permission_overwrites": [],
///         "available_tags": null,
///         "template": "",
///         "default_reaction_emoji": null,
///         "default_thread_rate_limit_per_user": null,
///         "default_sort_order": null,
///         "default_forum_layout": null,
///         "icon_emoji": null,
///         "theme_color": null
///       },
///       {
///         "id": 4,
///         "type": 2,
///         "name": "General",
///         "position": 0,
///         "topic": null,
///         "bitrate": 64000,
///         "user_limit": 0,
///         "nsfw": false,
///         "rate_limit_per_user": 0,
///         "parent_id": 3,
///         "default_auto_archive_duration": null,
///         "permission_overwrites": [],
///         "available_tags": null,
///         "template": "",
///         "default_reaction_emoji": null,
///         "default_thread_rate_limit_per_user": null,
///         "default_sort_order": null,
///         "default_forum_layout": null,
///         "icon_emoji": null,
///         "theme_color": null
///       }
///     ]
///   },
///   "is_dirty": null
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct GuildTemplate {
	/// The code of the template (unique ID)
	pub code: String,
	/// The name of the template
	/// (1-100 characters)
	pub name: ArrayString<100>,
	/// The description for the template
	/// (max 120 characters)
	pub description: Option<ArrayString<120>>,
	/// Number of times this template has been used
	pub usage_count: u32,
	/// The ID of the user who created the template
	pub creator_id: UserId,
	/// The user who created the template
	pub creator: PartialUser,
	/// When this template was created
	pub created_at: Timestamp,
	/// When this template was last synced to the source guild
	pub updated_at: Timestamp,
	/// The ID of the guild this template is based on
	pub source_guild_id: GuildId,
	/// The guild snapshot this template contains
	pub serialized_source_guild: PartialGuild,
	/// Whether the template has unsynced changes
	pub is_dirty: Option<bool>,
}
