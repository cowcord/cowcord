use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::auto_moderation::AutomodIncidentsData;
use crate::api::discovery::GuildProfile;
use crate::api::emoji::Emoji;
use crate::api::integrations::IntegrationApplication;
use crate::api::stickers::Sticker;
use crate::api::users::{AvatarDecorationData, PartialUser};
use crate::common::id::{
	ApplicationId,
	ChannelId,
	GenericSnowflake,
	GuildId,
	GuildJoinRequestId,
	IntegrationId,
	OnboardingPromptId,
	OnboardingPromptOptionId,
	RoleId,
	SkuId,
	SubscriptionId,
	UserId,
};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct Guild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The guild's banner hash
	pub banner: Option<String>,
	/// The guild's home header hash , also used in server guide
	pub home_header: Option<String>,
	/// The guild's splash hash
	pub splash: Option<String>,
	/// The guild's discovery splash hash
	pub discovery_splash: Option<String>,
	/// The user ID of the guild's owner
	pub owner_id: UserId,
	/// The application ID of the guild's owner, if bot-created
	pub application_id: Option<ApplicationId>,
	/// The description for the guild (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// The main voice region ID of the guild
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<Option<String>>,
	/// The ID of the guild's AFK channel; this is where members in voice idle for longer than afk_timeout are moved
	pub afk_channel_id: Option<ChannelId>,
	/// The AFK timeout of the guild (one of 60, 300, 900, 1800, 3600, in seconds)
	pub afk_timeout: u16,
	/// Whether the guild widget is enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub widget_enabled: Option<bool>,
	/// The channel ID that the widget will generate an invite to, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub widget_channel_id: Option<Option<ChannelId>>,
	/// The verification level required for the guild
	pub verification_level: VerificationLevel,
	/// Default message notification level for the guild
	pub default_message_notifications: MessageNotificationLevel,
	/// Whose messages are scanned and deleted for explicit content in the guild
	pub explicit_content_filter: ExplicitContentFilterLevel,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// Roles in the guild
	pub roles: Vec<Role>,
	/// Custom guild emojis
	pub emojis: Vec<Emoji>,
	/// Custom guild stickers
	pub stickers: Vec<Sticker>,
	/// Required MFA level for administrative actions within the guild
	pub mfa_level: MfaLevel,
	/// The ID of the channel where system event messages, such as member joins and premium subscriptions (boosts), are posted
	pub system_channel_id: Option<ChannelId>,
	/// The flags that limit system event messages
	pub system_channel_flags: SystemChannelFlags,
	/// The ID of the channel where community guilds display rules and/or guidelines
	pub rules_channel_id: Option<ChannelId>,
	/// The ID of the channel where admins and moderators of community guilds receive notices from Discord
	pub public_updates_channel_id: Option<ChannelId>,
	/// The ID of the channel where admins and moderators of community guilds receive safety alerts from Discord
	pub safety_alerts_channel_id: Option<ChannelId>,
	/// The maximum number of presences for the guild (null is usually returned, apart from the largest of guilds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_presences: Option<Option<u32>>,
	/// The maximum number of members for the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_members: Option<u64>,
	/// The guild's vanity invite code
	pub vanity_url_code: Option<String>,
	/// The guild's premium tier (boost level)
	pub premium_tier: PremiumTier,
	/// The number of premium subscriptions (boosts) the guild currently has
	pub premium_subscription_count: u32,
	/// The preferred locale of the guild; used in discovery and notices from Discord (default "en-US")
	pub preferred_locale: String,
	/// The maximum number of users in a voice channel while someone has video enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_video_channel_users: Option<u16>,
	/// The maximum number of users in a stage channel while someone has video enabled
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_stage_video_channel_users: Option<u16>,
	/// Whether the guild is considered NSFW ( EXPLICIT or AGE_RESTRICTED )
	#[deprecated]
	pub nsfw: bool,
	/// The NSFW level of the guild
	pub nsfw_level: NsfwLevel,
	/// The type of student hub the guild is, if it is a student hub
	pub hub_type: Option<HubType>,
	/// Whether the guild has the premium (boost) progress bar enabled
	pub premium_progress_bar_enabled: bool,
	/// The ID of the guild's latest onboarding prompt option
	pub latest_onboarding_question_id: Option<OnboardingPromptOptionId>,
	/// Information on the guild's AutoMod incidents
	pub incidents_data: Option<AutomodIncidentsData>,
	/// Settings for emoji packs
	#[deprecated]
	pub inventory_settings: Option<GuildInventorySettings>,
	/// Approximate count of total members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_member_count: Option<u32>,
	/// Approximate count of non-offline members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_presence_count: Option<u32>,
	/// The guild's powerup information
	pub premium_features: Option<GuildPremiumFeatures>,
	/// The guild's identity
	pub profile: Option<GuildIdentity>,
}

#[derive(Serialize, Deserialize)]
pub struct PartialGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The description for the guild (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// The guild's splash hash
	pub splash: Option<String>,
	/// The guild's discovery splash hash
	pub discovery_splash: Option<String>,
	/// The guild's home header hash , also used in server guide
	pub home_header: Option<String>,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// Custom guild emojis
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emojis: Option<Vec<Emoji>>,
	/// Custom guild stickers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stickers: Option<Vec<Sticker>>,
	/// Approximate number of total members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_member_count: Option<u32>,
	/// Approximate number of non-offline members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_presence_count: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildIdentity {
	/// The tag of the guild (2-4 characters)
	pub tag: ArrayString<4>,
	/// The guild badge hash
	pub badge: String,
}

#[derive(Serialize, Deserialize)]
pub struct GuildInventorySettings {
	/// Allows everyone to collect and use the guild's emoji globally
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_emoji_pack_collectible: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildPremiumFeatures {
	/// Enabled powerup-specific guild features
	pub features: Vec<String>,
	/// The number of additional emoji slots available to the guild
	pub additional_emoji_slots: u16,
	/// The number of additional sticker slots available to the guild
	pub additional_sticker_slots: u16,
	/// The number of additional soundboard slots available to the guild
	pub additional_sound_slots: u16,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageNotificationLevel {
	/// Receive notifications for all messages
	ALL_MESSAGES = 0,
	/// Receive notifications only for messages that @mention you
	ONLY_MENTIONS = 1,
	/// Don't receive notifications
	NO_MESSAGES = 2,
	/// Inherit value from guild settings when in a channel override context
	INHERIT = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
	/// Media content will not be scanned
	DISABLED = 0,
	/// Media content sent by members without roles will be scanned
	MEMBERS_WITHOUT_ROLES = 1,
	/// Media content sent by all members will be scanned
	ALL_MEMBERS = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MfaLevel {
	/// Guild has no MFA requirement for moderation actions
	NONE = 0,
	/// Guild has a MFA requirement for moderation actions
	ELEVATED = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VerificationLevel {
	/// Unrestricted
	NONE = 0,
	/// Must have a verified email on file
	LOW = 1,
	/// Must be registered on Discord for longer than 5 minutes
	MEDIUM = 2,
	/// Must be a member of the server for longer than 10 minutes
	HIGH = 3,
	/// Must have a verified phone number on file
	VERY_HIGH = 4,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NsfwLevel {
	/// Guild is not yet rated by Discord
	DEFAULT = 0,
	/// Guild has mature content only suitable for users over 18
	EXPLICIT = 1,
	/// Guild is safe for work
	SAFE = 2,
	/// Guild has mildly mature content that may not be suitable for users under 18
	AGE_RESTRICTED = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PremiumTier {
	/// Guild has not unlocked any Server Boost perks
	NONE = 0,
	/// Guild has unlocked Server Boost level 1 perks
	TIER_1 = 1,
	/// Guild has unlocked Server Boost level 2 perks
	TIER_2 = 2,
	/// Guild has unlocked Server Boost level 3 perks
	TIER_3 = 3,
}

bitflags! {
	pub struct SystemChannelFlags: u64 {
		/// Suppress member join notifications
		const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
		/// Suppress premium subscription (boost) notifications
		const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
		/// Suppress guild setup tips
		const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
		/// Hide member join sticker reply buttons
		const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
		/// Suppress role subscription purchase and renewal notifications
		const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
		/// Hide role subscription sticker reply buttons
		const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
		/// Suppress dead chat channel prompts
		const SUPPRESS_CHANNEL_PROMPT_DEADCHAT = 1 << 7;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PrivacyLevel {
	/// Scheduled event or stage instance is visible publicly
	PUBLIC = 1,
	/// Scheduled event or stage instance is only visible to guild members
	GUILD_ONLY = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HubType {
	/// Student hub is not categorized as a high school or post-secondary institution
	DEFAULT = 0,
	/// Student hub is for a high school
	HIGH_SCHOOL = 1,
	/// Student hub is for a post-secondary institution (college or university)
	COLLEGE = 2,
}

/// The available guild features, their functionality, and their requirements is subject to arbitrary change. The following table is a best-effort attempt to document the current state of guild features.
#[derive(Serialize, Deserialize)]
pub enum GuildFeatures {
	/// Access to alpha embedded activities ( soft_launch release phase )
	ACTIVITIES_ALPHA,
	/// Access to employee-released embedded activities ( employee_release release phase )
	ACTIVITIES_EMPLOYEE,
	/// Access to internal developer embedded activities ( activities_team release phase )
	ACTIVITIES_INTERNAL_DEV,
	/// Member list activity feed disabled
	ACTIVITY_FEED_DISABLED_BY_USER,
	/// Member list activity feed enabled
	ACTIVITY_FEED_ENABLED_BY_USER,
	/// Ability to set an animated guild banner image
	ANIMATED_BANNER,
	/// Ability to set an animated guild icon
	ANIMATED_ICON,
	/// Increased maximum voice channel bitrate (128 kbps)
	AUDIO_BITRATE_128_KBPS,
	/// Increased maximum voice channel bitrate (256 kbps)
	AUDIO_BITRATE_256_KBPS,
	/// Increased maximum voice channel bitrate (384 kbps)
	AUDIO_BITRATE_384_KBPS,
	/// AutoMod feature enabled
	AUTO_MODERATION,
	/// Ability to set a guild banner image
	BANNER,
	/// Big what now 🤨
	BFG,
	/// Access to early feature testing for bot and library developers
	BOT_DEVELOPER_EARLY_ACCESS,
	/// Guild has channel icon emojis populated
	CHANNEL_ICON_EMOJIS_GENERATED,
	/// Access to store channels
	COMMERCE,
	/// Access to welcome screen , member verification , stage channels , discovery, community updates reception , and more
	COMMUNITY,
	/// Early access to experimental community features
	COMMUNITY_CANARY,
	COMMUNITY_EXP_LARGE_GATED,
	COMMUNITY_EXP_LARGE_UNGATED,
	COMMUNITY_EXP_MEDIUM,
	/// Guild owner has accepted the updated monetization agreements
	CREATOR_ACCEPTED_NEW_TERMS,
	/// Monetization enabled
	CREATOR_MONETIZABLE,
	/// Monetization permanently disabled by Discord
	CREATOR_MONETIZABLE_DISABLED,
	/// Monetization features are pending until the new guild owner completes the onboarding process
	CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING,
	/// Monetization enabled
	CREATOR_MONETIZABLE_PROVISIONAL,
	/// Guild has restrictions on monetization features
	CREATOR_MONETIZABLE_RESTRICTED,
	CREATOR_MONETIZABLE_WHITEGLOVE,
	CREATOR_MONETIZATION_APPLICATION_ALLOWLIST,
	/// Monetization store page enabled
	CREATOR_STORE_PAGE,
	/// Guild is an application support server
	DEVELOPER_SUPPORT_SERVER,
	/// Guild is public and discoverable in the directory
	DISCOVERABLE,
	/// Discovery permanently disabled by Discord
	DISCOVERABLE_DISABLED,
	/// Guild has previously been discoverable
	ENABLED_DISCOVERABLE_BEFORE,
	/// Guild has enabled the members tab in the channel list without being a community guild
	ENABLED_MODERATION_EXPERIENCE_FOR_NON_COMMUNITY,
	/// Ability to set gradient role colors
	ENHANCED_ROLE_COLORS,
	EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT,
	FORWARDING_DISABLED,
	/// Guild has used guest invites
	GUESTS_ENABLED,
	/// Onboarding feature enabled
	GUILD_ONBOARDING,
	/// Guild has previously enabled the onboarding feature
	GUILD_ONBOARDING_EVER_ENABLED,
	/// Guild has prompts configured in onboarding
	GUILD_ONBOARDING_HAS_PROMPTS,
	/// Access to guild products
	GUILD_PRODUCTS,
	/// Allows uploading archive formats as guild products
	GUILD_PRODUCTS_ALLOW_ARCHIVED_FILE,
	/// Server guide feature enabled
	GUILD_SERVER_GUIDE,
	/// Ability to set a guild tag
	GUILD_TAGS,
	/// Guild has an immutable vanity given by the server web page feature
	GUILD_WEB_PAGE_VANITY_URL,
	/// Guild previously had access to embedded activities and can bypass the premium tier requirement
	HAD_EARLY_ACTIVITIES_ACCESS,
	/// Guild is listed in a directory channel
	HAS_DIRECTORY_ENTRY,
	HIDE_FROM_EXPERIMENT_UI,
	/// Guild is a student hub
	HUB,
	/// Ability to have over 1,000 active threads
	INCREASED_THREAD_LIMIT,
	/// Restricts guild joining to Discord employees
	INTERNAL_EMPLOYEE_ONLY,
	/// Ability to set an invite splash background
	INVITE_SPLASH,
	/// Guild has game leaderboard enabled
	LEADERBOARD_ENABLED,
	/// Guild has paused invites , preventing new members from joining
	INVITES_DISABLED,
	/// Guild is linked to a student hub
	LINKED_TO_HUB,
	/// Increased maximum file upload size (50 MB)
	MAX_FILE_SIZE_50_MB,
	/// Increased maximum file upload size (100 MB)
	MAX_FILE_SIZE_100_MB,
	/// Access to member safety tab
	MEMBER_SAFETY_PAGE_ROLLOUT,
	/// Member verification enabled, requiring new members to pass the verification gate before interacting with the guild
	MEMBER_VERIFICATION_GATE_ENABLED,
	/// Membership verification manual approval enabled
	MEMBER_VERIFICATION_MANUAL_APPROVAL,
	/// Early access to member verification manual approval general availability
	MEMBER_VERIFICATION_ROLLOUT_TEST,
	/// Increased guild emoji slots (200 each for normal and animated)
	MORE_EMOJI,
	/// Increased guild soundboard slots (96)
	MORE_SOUNDBOARD,
	/// Increased guild sticker slots (60)
	MORE_STICKERS,
	/// Access to news channels
	NEWS,
	/// Non-community guild is opt-in to raid alerts
	NON_COMMUNITY_RAID_ALERTS,
	/// Guild is partnered with Discord
	PARTNERED,
	/// Guild premium tier is overrided to 3 regardless of premium subscription count
	PREMIUM_TIER_3_OVERRIDE,
	/// Guild is accessable (read-only) without passing member verification
	PREVIEW_ENABLED,
	/// Guild has guild products available for purchase
	PRODUCTS_AVAILABLE_FOR_PURCHASE,
	/// Raid alerts opted out
	RAID_ALERTS_DISABLED,
	/// Shards connections to the guild to different nodes that relay information between each other
	RELAY_ENABLED,
	/// Ability to set an image or emoji as a role icon
	ROLE_ICONS,
	/// Guild has role subscriptions available for purchase
	ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE,
	/// Role subscriptions enabled
	ROLE_SUBSCRIPTIONS_ENABLED,
	/// Shards student hub UI
	SHARD,
	/// Access to the shared canvas feature
	SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST,
	/// Guild has a custom soundboard sound
	SOUNDBOARD,
	/// Increased maximum stage stream viewers (50)
	STAGE_CHANNEL_VIEWERS_50,
	/// Increased maximum stage stream viewers (150)
	STAGE_CHANNEL_VIEWERS_150,
	/// Increased maximum stage stream viewers (300)
	STAGE_CHANNEL_VIEWERS_300,
	/// Access to conversation summaries general access
	SUMMARIES_ENABLED_GA,
	/// Conversation summaries opted out
	SUMMARIES_DISABLED_BY_USER,
	/// Conversation summaries enabled
	SUMMARIES_ENABLED_BY_USER,
	SUMMARIES_LONG_LOOKBACK,
	/// Conversation summaries enabled by default, must be opted out instead of opted in
	SUMMARIES_OPT_OUT_EXPERIENCE,
	/// Restricts guild joining to users with the COLLABORATOR flag or higher
	STAFF_LEVEL_COLLABORATOR_REQUIRED,
	/// Restricts guild joining to users with the RESTRICTED_COLLABORATOR flag or higher
	STAFF_LEVEL_RESTRICTED_COLLABORATOR_REQUIRED,
	THREAD_DEFAULT_AUTO_ARCHIVE_DURATION,
	/// Guild is using the new powerups-based boosting system
	TIERLESS_BOOSTING,
	/// Early access to guild powerup management
	TIERLESS_BOOSTING_CLIENT_TEST,
	/// Early access to guild powerup management
	TIERLESS_BOOSTING_TEST,
	/// Ability to set a vanity URL
	VANITY_URL,
	/// Guild is verified
	VERIFIED,
	/// Increased camera feed quality (720p)
	VIDEO_BITRATE_ENHANCED,
	/// Increased maximum streaming quality (720p, 60fps)
	VIDEO_QUALITY_720_60FPS,
	/// Increased maximum streaming quality (1080p, 60fps)
	VIDEO_QUALITY_1080_60FPS,
	/// Increased maximum voice channel bitrate (384 kbps)
	VIP_REGIONS,
	/// Access to voice in threads (calls within threads)
	VOICE_IN_THREADS,
	/// Welcome screen enabled
	WELCOME_SCREEN_ENABLED,
}

/// These guild features are mutable, and can be edited with the [Modify Guild](https://docs.discord.food/resources/guild#modify-guild) endpoint.
#[derive(Serialize, Deserialize)]
pub enum MutableGuildFeatures {
	/// Whether the member list activity feed is explicitly disabled
	ACTIVITY_FEED_DISABLED_BY_USER,
	/// Whether the member list activity feed is explicitly enabled
	ACTIVITY_FEED_ENABLED_BY_USER,
	/// Wether community features are disabled
	COMMUNITY,
	/// Whether the guild is public and discoverable in the directory
	DISCOVERABLE,
	/// Whether the member tab is shown in the channel list for non-community guilds
	ENABLED_MODERATION_EXPERIENCE_FOR_NON_COMMUNITY,
	/// Whether joining the guild is disabled
	INVITES_DISABLED,
	/// Whether the member verification gate is enabled
	MEMBER_VERIFICATION_GATE_ENABLED,
	/// Whether raid alerts are opted in for non-community guilds
	NON_COMMUNITY_RAID_ALERTS,
	/// Whether raid alerts are opted out for community guilds
	RAID_ALERTS_DISABLED,
	/// Whether conversation summaries are enabled
	SUMMARIES_ENABLED_BY_USER,
}

#[derive(Serialize, Deserialize)]
pub struct UserGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The guild's banner hash
	pub banner: Option<String>,
	/// Whether the user is the owner of the guild
	pub owner: bool,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// Total permissions for the user in the guild (excludes overwrites)
	pub permissions: String,
	/// Approximate count of total members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_member_count: Option<u32>,
	/// Approximate count of non-offline members in the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_presence_count: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildWidget {
	/// The ID of the guild the widget is for
	pub id: GuildId,
	/// The name of the guild the widget is for
	pub name: String,
	/// The invite URL for the guild's widget channel, if any
	pub instant_invite: Option<String>,
	/// Approximate count of non-offline members in the guild
	pub presence_count: u32,
	/// The public voice and stage channels in the guild
	pub channels: Vec<GuildWidgetChannel>,
	/// The non-offline guild members (max 100)
	pub members: ArrayVec<GuildWidgetMember, 100>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildWidgetChannel {
	/// The ID of the channel
	pub id: ChannelId,
	/// The name of the channel (1-100 characters)
	pub name: ArrayString<100>,
	/// Sorting position of the channel
	pub position: i16,
}

/// Due to privacy concerns, `id`, `discriminator`, and `avatar` are anonymized. `id` is replaced with an incrementing integer, `discriminator` is always 0000, and `avatar` is always null (replaced with an encrypted `avatar_url` field).
#[derive(Serialize, Deserialize)]
pub struct GuildWidgetMember {
	/// The incrementing ID of the member
	pub id: u64,
	/// The display name or censored username of the member
	pub username: String,
	/// The avatar URL of the member
	pub avatar_url: String,
	/// The status of the member
	pub status: String,
	/// The primary activity the member is participating in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub activity: Option<GuildWidgetMemberActivity>,
	/// The ID of the voice or stage channel the member is in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<ChannelId>,
	/// Whether the member is deafened by the guild, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deaf: Option<bool>,
	/// Whether the member is muted by the guild, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute: Option<bool>,
	/// Whether the member is locally deafened
	#[serde(skip_serializing_if = "Option::is_none")]
	pub self_deaf: Option<bool>,
	/// Whether the member is locally muted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub self_mute: Option<bool>,
	/// Whether the member's permission to speak is denied
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suppress: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildWidgetMemberActivity {
	/// The name of the activity
	pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GuildWidgetSettings {
	/// Whether the widget is enabled
	pub enabled: bool,
	/// The channel ID that the widget will generate an invite to, if any
	pub channel_id: Option<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub struct Role {
	/// The ID of the role
	pub id: RoleId,
	/// The name of the role (max 100 characters)
	pub name: ArrayString<100>>,
	/// The description for the role (max 90 characters)
	pub description: Option<ArrayString<90>>,
	/// The color of the role represented as an integer representation of a hexadecimal color code
	#[deprecated]
	pub color: u32,
	/// The colors of the role encoded as an integer representation of hexadecimal color codes
	pub colors: RoleColors,
	/// Whether this role is pinned in the user listing
	pub hoist: bool,
	/// The role's icon hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<Option<String>>,
	/// The role's unicode emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unicode_emoji: Option<Option<String>>,
	/// Position of this role
	pub position: i64,
	/// The permission bitwise value for the role
	pub permissions: String,
	/// Whether this role is managed by an integration
	pub managed: bool,
	/// Whether this role is mentionable
	pub mentionable: bool,
	/// The role's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<RoleFlags>,
	/// The tags this role has
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<RoleTags>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleColors {
	/// The primary color of the role (matches color)
	pub primary_color: u32,
	/// The secondary color of the role, creating a two-point gradient
	pub secondary_color: Option<u32>,
	/// The tertiary color of the role, creating a three-point gradient
	pub tertiary_color: Option<u32>,
}

/// Tags with type null represent booleans. They will be present and set to null if they are true, and will be not present if they are false.
#[derive(Serialize, Deserialize)]
pub struct RoleTags {
	/// The ID of the bot this role belongs to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot_id: Option<UserId>,
	/// The ID of the integration this role belongs to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_id: Option<IntegrationId>,
	/// Whether this is the guild's premium subscriber (booster) role
	#[serde(skip_serializing_if = "Option::is_none")]
	pub premium_subscriber: Option<Value>,
	/// The ID of this role's subscription SKU and listing
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_listing_id: Option<SkuId>,
	/// Whether this role is available for purchase
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_for_purchase: Option<Value>,
	/// Whether this role has a connection requirement
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_connections: Option<Value>,
}

bitflags! {
	pub struct RoleFlags: u64 {
		/// Role is part of an onboarding prompt option
		const IN_PROMPT = 1 << 0;
	}
}

#[derive(Serialize, Deserialize)]
pub struct RoleConnectionRequirement {
	/// The type of connection required
	pub connection_type: String,
	/// The metadata field to check for the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub connection_metadata_field: Option<Option<String>>,
	/// The comparison operator to use
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operator: Option<Option<RoleConnectionOperatorType>>,
	/// The value to compare the metadata field to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<Option<String>>,
	/// The ID of the application to check for the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<ApplicationId>,
	/// The application to check for the connection
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application: Option<IntegrationApplication>,
	/// The friendly name of the application's metadata field
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The description of the application's metadata field
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The result of the connection check
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RoleConnectionOperatorType {
	/// The metadata value ( integer ) is less than or equal to the guild's configured value ( integer )
	INTEGER_LESS_THAN_OR_EQUAL = 1,
	/// The metadata value ( integer ) is greater than or equal to the guild's configured value ( integer )
	INTEGER_GREATER_THAN_OR_EQUAL = 2,
	/// The metadata value ( integer ) is equal to the guild's configured value ( integer )
	INTEGER_EQUAL = 3,
	/// The metadata value ( integer ) is not equal to the guild's configured value ( integer )
	INTEGER_NOT_EQUAL = 4,
	/// The metadata value ( ISO8601 string ) is less than or equal to the guild's configured value ( integer ; days before current date )
	DATETIME_LESS_THAN_OR_EQUAL = 5,
	/// The metadata value ( ISO8601 string ) is greater than or equal to the guild's configured value ( integer ; days before current date )
	DATETIME_GREATER_THAN_OR_EQUAL = 6,
	/// The metadata value ( integer ) is equal to the guild's configured value ( integer ; 1 )
	BOOLEAN_EQUAL = 7,
	/// The metadata value ( integer ) is not equal to the guild's configured value ( integer ; 1 )
	BOOLEAN_NOT_EQUAL = 8,
}

#[derive(Serialize, Deserialize)]
pub struct GuildMember {
	/// The user this guild member represents
	pub user: PartialUser,
	/// The guild-specific nickname of the member (1-32 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nick: Option<Option<String>>,
	/// The member's guild avatar hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<Option<String>>,
	/// The member's guild avatar decoration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar_decoration_data: Option<Option<AvatarDecorationData>>,
	/// The member's guild banner hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner: Option<Option<String>>,
	/// The role IDs assigned to this member
	pub roles: Vec<RoleId>,
	/// When the user joined the guild
	pub joined_at: Timestamp,
	/// When the member subscribed to (started boosting ) the guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub premium_since: Option<Option<Timestamp>>,
	/// Whether the member is deafened in voice channels
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deaf: Option<bool>,
	/// Whether the member is muted in voice channels
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mute: Option<bool>,
	/// Whether the member has not yet passed the guild's member verification requirements
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pending: Option<bool>,
	/// When the member's timeout will expire and they will be able to communicate in the guild again
	#[serde(skip_serializing_if = "Option::is_none")]
	pub communication_disabled_until: Option<Option<Timestamp>>,
	/// When the member's unusual DM activity flag will expire
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unusual_dm_activity_until: Option<Option<Timestamp>>,
	/// The member's flags
	pub flags: GuildMemberFlags,
	/// Total permissions of the member in the channel, including overwrites
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions: Option<String>,
}

bitflags! {
	pub struct GuildMemberFlags: u64 {
		/// Guild member has left and rejoined the guild
		const DID_REJOIN = 1 << 0;
		/// Guild member has completed onboarding
		const COMPLETED_ONBOARDING = 1 << 1;
		/// Guild member bypasses guild verification requirements and member verification
		const BYPASSES_VERIFICATION = 1 << 2;
		/// Guild member has started onboarding
		const STARTED_ONBOARDING = 1 << 3;
		/// Guild member is a guest and not a true member
		const IS_GUEST = 1 << 4;
		/// Guild member has started the server guide
		const STARTED_SERVER_GUIDE = 1 << 5;
		/// Guild member has completed all of the server guide
		const COMPLETED_SERVER_GUIDE = 1 << 6;
		/// Guild member has been indefinitely quarantined by an AutoMod Rule for their username, display name, or nickname
		const AUTOMOD_QUARANTINED_NAME = 1 << 7;
		/// Guild member has acknowledged the DM privacy settings upsell modal
		const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1 << 9;
		/// Guild member has been indefinitely quarantined by an AutoMod Rule for their primary guild tag
		const AUTOMOD_QUARANTINED_GUILD_TAG = 1 << 10;
	}
}

#[derive(Serialize, Deserialize)]
pub struct SupplementalGuildMember {
	/// The ID of the user this guild member represents
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<UserId>,
	/// The associated guild member
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member: Option<GuildMember>,
	/// How the user joined the guild
	pub join_source_type: JoinSourceType,
	/// The invite code or vanity used to join the guild, if applicable
	pub source_invite_code: Option<String>,
	/// The ID of the user who invited the user to the guild, if applicable
	pub inviter_id: Option<UserId>,
	/// The type of integration that added the user to the guild, if applicable
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integration_type: Option<Option<i64>>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum JoinSourceType {
	/// The user joined the guild through an unknown source
	UNSPECIFIED = 0,
	/// The user was added to the guild by a bot using the guilds.join OAuth2 scope
	BOT = 1,
	/// The user was added to the guild by an integration (e.g. Twitch)
	INTEGRATION = 2,
	/// The user joined the guild through guild discovery
	DISCOVERY = 3,
	/// The user joined the guild through a student hub
	HUB = 4,
	/// The user joined the guild through an invite
	INVITE = 5,
	/// The user joined the guild through a vanity URL
	VANITY_URL = 6,
	/// The user was accepted into the guild after applying for membership
	MANUAL_MEMBER_VERIFICATION = 7,
}

#[derive(Serialize, Deserialize)]
pub struct Ban {
	/// The banned user
	pub user: PartialUser,
	/// The reason for the ban
	pub reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeScreen {
	/// The welcome message shown in the welcome screen (max 140 characters)
	pub description: Option<ArrayString<140>>,
	/// The channels shown in the welcome screen (max 5)
	pub welcome_channels: ArrayVec<WelcomeScreenChannel, 5>,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeScreenChannel {
	/// The ID of the channel
	pub channel_id: ChannelId,
	/// The description shown for the channel (1-50 characters)
	pub description: ArrayString<50>,
	/// The emoji ID , if the emoji is custom
	pub emoji_id: Option<Emoji>,
	/// The emoji name if custom, the unicode character if standard, or null if no emoji is set
	pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberVerification {
	/// When the member verification was last modified
	pub version: Option<Timestamp>,
	/// Questions for applicants to answer (max 5)
	pub form_fields: ArrayVec<MemberVerificationFormField, 5>,
	/// A description of what the guild is about; this can be different than the guild's description (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// The guild this member verification is for
	pub guild: Option<MemberVerificationGuild>,
	/// The profile of the guild this member verification is for
	pub profile: GuildProfile,
}

#[derive(Serialize, Deserialize)]
pub struct MemberVerificationFormField {
	/// The type of question
	pub field_type: String,
	/// The label for the form field (max 300 characters)
	pub label: ArrayString<300>,
	/// Multiple choice answers (1-8, max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub choices: Option<Vec<String>>,
	/// The rules that the user must agree to (1-16, max 300 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Option<Vec<String>>>,
	/// Response for this field
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response: Option<Option<MemberVerificationFormFieldResponse>>,
	/// Whether this field is required for a successful application
	pub required: bool,
	/// The subtext of the form field
	pub description: Option<String>,
	/// Unknown (max 300 characters, max 10)
	pub automations: Option<ArrayVec<ArrayString<300>, 10>>,
	/// Placeholder text for the field's response area
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<Option<String>>,
}

#[derive(Serialize, Deserialize)]
enum MemberVerificationFormFieldResponse {
	bool(bool),
	i64(i64),
	String(String),
}

#[derive(Serialize, Deserialize)]
pub enum MemberVerificationFormFieldType {
	/// User must agree to the guild rules
	TERMS,
	/// User must respond with a short answer (max 150 characters)
	TEXT_INPUT,
	/// User must respond with a paragraph (max 1000 characters)
	PARAGRAPH,
	/// User must select one of the provided choices
	MULTIPLE_CHOICE,
}

#[derive(Serialize, Deserialize)]
pub struct MemberVerificationGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: ArrayString<100>,
	/// The guild's icon hash
	pub icon: Option<String>,
	/// The description for the guild (max 300 characters)
	pub description: Option<ArrayString<300>>,
	/// The guild's splash hash
	pub splash: Option<String>,
	/// The guild's discovery splash hash
	pub discovery_splash: Option<String>,
	/// The guild's home header hash , also used in server guide
	pub home_header: Option<String>,
	/// The verification level required for the guild
	pub verification_level: VerificationLevel,
	/// Enabled guild features
	pub features: Vec<GuildFeatures>,
	/// Custom guild emojis
	pub emojis: Vec<Emoji>,
	/// Approximate number of total members in the guild
	pub approximate_member_count: u32,
	/// Approximate number of non-offline members in the guild
	pub approximate_presence_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GuildJoinRequest {
	/// The ID of the join request
	pub id: GuildJoinRequestId,
	/// The ID of the join request
	pub join_request_id: GuildJoinRequestId,
	/// When the join request was created
	pub created_at: Timestamp,
	/// The status of the join request
	pub application_status: GuildJoinRequestStatus,
	/// The ID of the guild this join request is for
	pub guild_id: GuildId,
	/// Responses to the guild's member verification questions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub form_responses: Option<Option<Vec<MemberVerificationFormField>>>,
	/// When the request was acknowledged by the user
	pub last_seen: Option<Timestamp>,
	/// A snowflake representing when the join request was actioned
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actioned_at: Option<GenericSnowflake>,
	/// The moderator who actioned the join request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actioned_by_user: Option<PartialUser>,
	/// Why the join request was rejected
	pub rejection_reason: Option<String>,
	/// The ID of the user who created this join request
	pub user_id: UserId,
	/// The user who created this join request
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// The ID of a channel where an interview regarding this join request may be conducted
	pub interview_channel_id: Option<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub enum GuildJoinRequestStatus {
	/// The request is started but not submitted
	STARTED,
	/// The request has been submitted
	SUBMITTED,
	/// The request has been rejected
	REJECTED,
	/// The request has been approved
	APPROVED,
}

#[derive(Serialize, Deserialize)]
pub struct Onboarding {
	/// The ID of the guild this onboarding is part of
	pub guild_id: GuildId,
	/// The prompts shown during onboarding and in community customization
	pub prompts: Vec<OnboardingPrompt>,
	/// The channel IDs that members get opted into automatically
	pub default_channel_ids: Vec<ChannelId>,
	/// Whether onboarding is enabled in the guild
	pub enabled: bool,
	/// Whether the guild is below the requirements for onboarding
	pub below_requirements: bool,
	/// The current criteria mode for onboarding
	pub mode: OnboardingMode,
}

/// Defines the criteria used to satisfy Onboarding constraints that are required for enabling.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OnboardingMode {
	/// Count only default channels towards constraints
	ONBOARDING_DEFAULT = 0,
	/// Count default channels and questions towards constraints
	ONBOARDING_ADVANCED = 1,
}

#[derive(Serialize, Deserialize)]
pub struct OnboardingPrompt {
	/// The ID of the prompt
	pub id: OnboardingPromptId,
	/// The type of prompt
	pub r#type: OnboardingPromptType,
	/// Options available within the prompt
	pub options: Vec<OnboardingPromptOption>,
	/// The title of the prompt
	pub title: String,
	/// Whether users are limited to selecting one option for the prompt
	pub single_select: bool,
	/// Whether the prompt is required before a user completes the onboarding flow
	pub required: bool,
	/// Wether the prompt is present in the onboarding flow, or only appears in community customization
	pub in_onboarding: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OnboardingPromptOption {
	/// The ID of the prompt option
	pub id: OnboardingPromptOptionId,
	/// The channel IDs a member is added to when the option is selected
	pub channel_ids: Vec<ChannelId>,
	/// The role IDs assigned to a member when the option is selected
	pub role_ids: Vec<RoleId>,
	/// Emoji representing the option
	pub emoji: Emoji,
	/// The title of the option
	pub title: String,
	/// The description for the option
	pub description: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OnboardingPromptType {
	/// Prompt offers multiple options to select from
	MULTIPLE_CHOICE = 0,
	/// Prompt offers a dropdown menu to select from
	DROPDOWN = 1,
}

#[derive(Serialize, Deserialize)]
pub struct PremiumGuildSubscription {
	/// The ID of the premium guild subscription
	pub id: SubscriptionId,
	/// The ID of the guild this subscription is for
	pub guild_id: GuildId,
	/// The ID of the user who created this premium guild subscription
	pub user_id: UserId,
	/// If this premium guild subscription has ended
	pub ended: bool,
	/// When this premium guild subscription will expire
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ends_at: Option<Timestamp>,
	/// When the user's overall subscription pause will end, reactivating the premium guild subscription
	pub pause_ends_at: Option<Timestamp>,
	/// The user this premium guild subscription is for
	pub user: PartialUser,
}
