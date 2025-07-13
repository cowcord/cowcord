use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::application::PartialApplication;
use crate::api::channel::PartialChannel;
use crate::api::discovery::GuildProfile;
use crate::api::guild::{GuildFeatures, NsfwLevel, PremiumGuildSubscription, VerificationLevel};
use crate::api::guild_scheduled_event::GuildScheduledEvent;
use crate::api::users::PartialUser;
use crate::common::id::GuildId;
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct Invite {
	/// The invite code (unique ID)
	pub code: String,
	/// The type of invite
	pub r#type: InviteType,
	/// The channel this invite is for; null for friend invites that did not have a DM channel created
	pub channel: Option<PartialChannel>,
	/// The ID of the guild this invite is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// The guild this invite is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild: Option<InviteGuild>,
	/// The profile of the guild this invite is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub profile: Option<GuildProfile>,
	/// The user who created the invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub inviter: Option<PartialUser>,
	/// The invite's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<InviteFlags>,
	/// The type of target for this guild invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_type: Option<InviteTargetType>,
	/// The user whose stream to display for this voice channel stream invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_user: Option<PartialUser>,
	/// The embedded application to open for this voice channel embedded application invite
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_application: Option<PartialApplication>,
	/// Approximate count of total members in the guild or group DM
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_member_count: Option<u32>,
	/// Approximate count of non-offline members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_presence_count: Option<u32>,
	/// The expiry date of the invite, if it expires
	pub expires_at: Option<Timestamp>,
	/// Guild scheduled event data, only included if guild_scheduled_event_id contains a valid guild scheduled event ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_scheduled_event: Option<GuildScheduledEvent>,
	/// Whether the user is a new member of the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_member: Option<bool>,
	/// Whether the user should be shown the guild's member verification form
	#[serde(skip_serializing_if = "Option::is_none")]
	pub show_verification_form: Option<bool>,
	/// Whether the @everyone role has the CHANGE_NICKNAME permission in the guild this invite is for
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_nickname_changeable: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InviteType {
	/// Joins the user to a guild
	GUILD = 0,
	/// Joins the user to a group DM
	GROUP_DM = 1,
	/// Adds the user as a friend to the inviter
	FRIEND = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InviteTargetType {
	/// The invite is for a stream in a voice channel
	STREAM = 1,
	/// The invite is for an embedded application (activity) in a voice channel
	EMBEDDED_APPLICATION = 2,
	/// The invite redirects to the role subscriptions page within a guild
	ROLE_SUBSCRIPTIONS = 3,
	/// The invite originates from the creator page of a guild
	CREATOR_PAGE = 4,
}

bitflags! {
	pub struct InviteFlags: u64 {
		/// Invite grants one-time access to a voice channel in the guild
		const IS_GUEST_INVITE = 1 << 0;
		/// Invite has been viewed by any user (has been retrieved using Get Invite )
		const IS_VIEWED = 1 << 1;
		/// Unknown
		const IS_ENHANCED = 1 << 2;
		/// Invite bypasses guild join requests and adds the user directly to the guild with pending set to false
		const IS_APPLICATION_BYPASS = 1 << 3;
	}
}

#[derive(Serialize, Deserialize)]
pub struct InviteMetadata {
	/// Number of times this invite has been used
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uses: Option<u32>,
	/// Max number of times this invite can be used
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_uses: Option<u32>,
	/// Duration (in seconds) after which the invite expires (default 0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_age: Option<u32>,
	/// Whether this invite only grants temporary membership (default false for unsupported invite types)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temporary: Option<bool>,
	/// When this invite was created
	pub created_at: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct InviteGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: String,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The description for the guild (max 300 characters)
	pub description: Option<String>,
	/// The guild's banner hash
	pub banner: Option<String>,
	/// The guild's splash hash
	pub splash: Option<String>,
	/// The verification level required for the guild
	pub verification_level: VerificationLevel,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// The guild's vanity invite code
	pub vanity_url_code: Option<String>,
	/// The number of premium subscriptions (boosts) the guild currently has
	#[serde(skip_serializing_if = "Option::is_none")]
	pub premium_subscription_count: Option<u32>,
	/// The guild's premium tier (boost level)
	pub premium_tier: PremiumGuildSubscription,
	/// Whether the guild is considered NSFW ( EXPLICIT or AGE_RESTRICTED )
	#[deprecated]
	pub nsfw: bool,
	/// The guild's NSFW level
	pub nsfw_level: NsfwLevel,
}
