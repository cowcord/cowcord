use serde::{Deserialize, Serialize};

use crate::api::invites::{Invite, InviteFlags, InviteTargetType, InviteWithMetadata};
use crate::common::id::{ApplicationId, ChannelId, GuildId, UserId};
use crate::utils::url::ToStringQuery;

/// Method: `GET`
///
/// Does not require authentication
///
/// Returns an [invite](https://docs.discord.food/resources/invite#invite-object) object for the given code.
///
/// Bots and users who have been blocked by the inviter cannot retrieve friend invites.
pub fn GET_INVITE(
	query: &GetInviteQueryParams,
	invite_code: &str,
) -> String {
	format!("/invites/{}{}", invite_code, query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct GetInviteQueryParams {
	/// Whether the invite should contain approximate member counts (and partial recipients for group DM invites)
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_counts: Option<bool>,
	/// Whether the invite should contain permission-related fields
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_permissions: Option<bool>,
	/// The guild scheduled event to include with the invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_scheduled_event_id: Option<GuildScheduledEventId>,
}

pub type GetInviteResponse = Invite;

/// Method: `POST`
///
/// Supports OAuth2 for authentication with the `guilds.join` scope
///
/// Accepts an invite to a [guild](https://docs.discord.food/resources/invite#invite-guild-object),
/// [group DM](https://docs.discord.food/resources/channel#channel-object),
/// or [DM](https://docs.discord.food/resources/channel#channel-object).
///
/// Clients should not use this endpoint to join many guilds in a short period of time.
/// Suspicious guild join activity may be flagged by Discord and require [additional verification steps](https://docs.discord.food/resources/user#required-action-type) or lead to immediate account termination.
///
/// Accepting an invite to a guild with the `HUB` [guild feature](https://docs.discord.food/resources/guild#guild-features) will have no effect.
///
/// For OAuth2 requests, only guild invites are supported and the bot attached to the application must be a member of the guild the invite is for.
///
/// Users who have been blocked by the inviter cannot accept friend invites.
///
/// Returns an [invite](https://docs.discord.food/resources/invite#invite-object) object on success.
/// May fire a [Guild Create](https://docs.discord.food/topics/gateway-events#guild-create),
/// [Guild Member Add](https://docs.discord.food/topics/gateway-events#guild-member-add),
/// [Guild Join Request Create](https://docs.discord.food/topics/gateway-events#guild-join-request-create),
/// [Channel Create](https://docs.discord.food/topics/gateway-events#channel-create),
/// and/or [Relationship Add](https://docs.discord.food/topics/gateway-events#relationship-add) Gateway event.
pub fn ACCEPT_INVITE(invite_code: &str) -> String {
	format!("/invites/{}", invite_code)
}

pub type AcceptInviteResponse = Invite;

/// Method: `DELETE`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `MANAGE_CHANNELS` permission on the channel this invite belongs to,
/// or `MANAGE_GUILD` to remove any invite across the guild, if the invite is to a guild.
///
/// Deletes an invite.
///
/// Returns an [invite](https://docs.discord.food/resources/invite#invite-object) object on success.
/// May fire an [Invite Delete](https://docs.discord.food/topics/gateway-events#invite-delete) Gateway event.
pub fn DELETE_INVITE(invite_code: &str) -> String {
	format!("/invites/{}", invite_code)
}

pub type DeleteInviteResponse = Invite;

/// Method: `GET`
///
/// Requires the `MANAGE_GUILD` permission.
///
/// Returns a list of [invite](https://docs.discord.food/resources/invite#invite-object) objects (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)) for the guild.
pub fn GET_GUILD_INVITES(guild_id: &GuildId) -> String {
	format!("/guilds/{}/invites", guild_id)
}

pub type GetGuildInvitesResponse = Vec<InviteWithMetadata>;

/// Method: `GET`
///
/// Requires the `MANAGE_CHANNELS` permission if the channel is in a guild.
///
/// Returns a list of [invite](https://docs.discord.food/resources/invite#invite-object) objects (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)) for the channel. Only usable for guild channels and group DMs.
pub fn GET_CHANNEL_INVITES(channel_id: &ChannelId) -> String {
	format!("/channels/{}/invites", channel_id)
}

pub type GetChannelInvitesResponse = Vec<InviteWithMetadata>;

/// Method: `POST`
///
/// Supports the `X-Audit-Log-Reason header`
///
/// Requires the `CREATE_INSTANT_INVITE` permission if the channel is in a guild.
///
/// Creates a new [invite](https://docs.discord.food/resources/invite#invite-object) object for the channel.
///
/// Only usable for guild channels and group DMs.
///
/// Users cannot create invites for managed group DMs.
///
/// In the case of a guild with the instant invite operation disabled by Discord, this endpoint will return an unexpected 204 empty response.
///
/// Returns an [invite](https://docs.discord.food/resources/invite#invite-object) object (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)).
/// Fires an [Invite Create](https://docs.discord.food/topics/gateway-events#invite-create) Gateway event if the channel is in a guild.
pub fn CREATE_CHANNEL_INVITE(channel_id: &ChannelId) -> String {
	format!("/channels/{}/invites", channel_id)
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannelInvitesRequest {
	/// The invite's flags
	/// (only `IS_GUEST_INVITE` and `IS_APPLICATION_BYPASS` can be set)
	///
	/// Creating an invite with the `IS_APPLICATION_BYPASS` flag requires the `KICK_MEMBERS` permission.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<InviteFlags>,
	/// Number of seconds before expiry, or 0 for never
	/// (0-604800, default 86400)
	///
	/// For group DMs, `max_age` is the only supported parameter, and it does not support 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_age: Option<u32>,
	/// Max number of uses or 0 for unlimited
	/// (0-100, default 0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_uses: Option<u8>,
	/// Whether this invite only grants temporary membership
	/// (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temporary: Option<bool>,
	/// Whether to try to reuse a similar invite
	/// (useful for creating many unique one time use invites, default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unique: Option<bool>,
	/// The invite code to validate and attempt to reuse.
	/// If nonexistant, a new invite will be created as usual
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validate: Option<Option<String>>,
	/// The type of target for this voice channel invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_type: Option<InviteTargetType>,
	/// The ID of the user whose stream to display for this invite.
	///
	/// required if `target_type` is `STREAM`, and the user must be streaming in the channel
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_user_id: Option<UserId>,
	/// The ID of the embedded application to open for this invite
	///
	/// required if target_type is `EMBEDDED_APPLICATION`, and the application must have the `EMBEDDED` flag
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_application_id: Option<ApplicationId>,
}

pub type CreateChannelInvitesResponse = InviteWithMetadata;

/// Method: `GET`
///
/// Returns a list of friend [invite](https://docs.discord.food/resources/invite#invite-object) objects (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)) for the current user.
pub const GET_USER_INVITES: &str = "/users/@me/invites";

pub type GetUserInvitesResponse = Vec<InviteWithMetadata>;

/// Method: `POST`
///
/// Creates a new friend invite.
///
/// Returns a friend [invite](https://docs.discord.food/resources/invite#invite-object) object (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)) on success.
pub const CREATE_USER_INVITE: &str = "/users/@me/invites";

#[derive(Serialize, Deserialize)]
pub struct CreateUserInviteRequest {
	/// The pre-generated friend invite code to create an invite from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
}

pub type CreateUserInviteResponse = InviteWithMetadata;

/// Method: `DELETE`
///
/// Revokes all of the current user's friend invites.
///
/// Returns a list of revoked friend [invite](https://docs.discord.food/resources/invite#invite-object) objects (with [invite metadata](https://docs.discord.food/resources/invite#invite-metadata-object)) on success.
pub const REVOKE_USER_INVITES: &str = "/users/@me/invites";

pub type RevokeUserInvitesResponse = Vec<InviteWithMetadata>;
