use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{api_types::{application::PartialApplication, channel::{Channel, PartialChannel}, components::{Component, SelectMenuResolved}, emoji::Emoji, integrations::IntegrationApplication, presences::ActivityActionType, soundboard::SoundboardSound, stickers::{Sticker, StickerItem}, users::PartialUser}, types::Timestamp};

#[derive(Serialize, Deserialize)]
pub struct Message {
	/// The ID of the message
	pub id: Snowflake,
	/// The ID of the channel the message was sent in
	pub channel_id: Snowflake,
	/// The ID of the lobby the message was sent in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lobby_id: Option<Snowflake>,
	/// The author of the message
	pub author: PartialUser,
	/// Contents of the message
	pub content: String,
	/// When this message was sent
	pub timestamp: Timestamp,
	/// When this message was last edited
	pub edited_timestamp: Option<Timestamp>,
	/// Whether this message will be read out by TTS
	pub tts: bool,
	/// Whether this message mentions everyone
	pub mention_everyone: bool,
	/// Users specifically mentioned in the message
	pub mentions: Vec<PartialUser>,
	/// Roles specifically mentioned in this message
	pub mention_roles: Vec<Snowflake>,
	/// Channels specifically mentioned in this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mention_channels: Option<Vec<PartialChannel>>,
	/// The attached files
	pub attachments: Vec<Attachment>,
	/// Content embedded in the message
	pub embeds: Vec<Embed>,
	/// Reactions on the message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reactions: Option<Vec<Reaction>>,
	/// The message's nonce, used for message deduplication
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nonce: Option<i64>,
	/// Whether this message is pinned
	pub pinned: bool,
	/// The ID of the webhook that send the message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub webhook_id: Option<Snowflake>,
	/// The type of message
	pub r#type: MessageType,
	/// The rich presence activity the author is inviting users to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub activity: Option<MessageActivity>,
	/// The application of the message's rich presence activity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application: Option<IntegrationApplication>,
	/// The ID of the application; only sent for interaction responses and messages created through OAuth2
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<Snowflake>,
	/// The message's flags
	pub flags: MessageFlags,
	/// The source of a crosspost, snapshot, channel follow add, pin, or reply message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_reference: Option<MessageReference>,
	/// The message associated with the message_reference
	#[serde(skip_serializing_if = "Option::is_none")]
	pub referenced_message: Option<Option<Box<Message>>>,
	/// The partial message snapshot associated with the message_reference
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_snapshots: Option<Vec<MessageSnapshot>>,
	/// The private channel call that prompted this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub call: Option<MessageCall>,
	/// The interaction the message is responding to
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interaction: Option<MessageInteraction>,
	/// The interaction the message originated from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interaction_metadata: Option<MessageInteractionMetadata>,
	/// The resolved data for the select menu component's interaction
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resolved: Option<SelectMenuResolved>,
	/// The thread that was started from this message, with the member key representing thread member data
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thread: Option<Channel>,
	/// The role subscription purchase or renewal that prompted this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_subscription_data: Option<MessageRoleSubscription>,
	/// The guild purchase that prompted this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub purchase_notification: Option<MessagePurchaseNotification>,
	/// Information on the gift that prompted this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gift_info: Option<MessageGiftInfo>,
	/// The message's components (e.g. buttons, select menus)
	pub components: Vec<Component>,
	/// The message's sticker items
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticker_items: Option<Vec<StickerItem>>,
	/// Extra rich information for the message's sticker items; only available in some contexts
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stickers: Option<Vec<Sticker>>,
	/// A poll!
	#[serde(skip_serializing_if = "Option::is_none")]
	pub poll: Option<Poll>,
	/// The ID of the changelog that prompted this message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changelog_id: Option<Snowflake>,
	/// The message's soundboard sounds
	#[serde(skip_serializing_if = "Option::is_none")]
	pub soundboard_sounds: Option<Vec<SoundboardSound>>,
	/// Potions applied to the message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub potions: Option<Vec<Potion>>,
}

/// This structure is a subset of the [message](https://docs.discord.food/resources/message#message-object) struct
#[derive(Serialize, Deserialize)]
pub struct PartialMessage {
	/// The ID of the message
	pub id: Snowflake,
	/// The ID of the lobby the message was sent in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lobby_id: Option<Snowflake>,
	/// The ID of the channel the message was sent in
	pub channel_id: Snowflake,
	/// The type of message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<MessageType>,
	/// Contents of the message
	pub content: String,
	/// The author of the message
	pub author: PartialUser,
	/// The message's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<MessageFlags>,
	/// The ID of the application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<Snowflake>,
	/// The channel the message was sent in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel: Option<Channel>,
	/// The ID of the other recipient
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recipient_id: Option<Snowflake>,
}

/// Type `19` and `20` are only available in API v8 and above. In v7 and below, they are represented as type `0`. Additionally, type `21` is only available in API v9 and above.
/// 
/// none of that should matter howevere because api v9 is used
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
	/// A default message (see below)
	DEFAULT = 0,
	/// A message sent when a user is added to a group DM or thread
	RECIPIENT_ADD = 1,
	/// A message sent when a user is removed from a group DM or thread
	RECIPIENT_REMOVE = 2,
	/// A message sent when a user creates a call in a private channel
	CALL = 3,
	/// A message sent when a group DM or thread's name is changed
	CHANNEL_NAME_CHANGE = 4,
	/// A message sent when a group DM's icon is changed
	CHANNEL_ICON_CHANGE = 5,
	/// A message sent when a message is pinned in a channel
	CHANNEL_PINNED_MESSAGE = 6,
	/// A message sent when a user joins a guild
	USER_JOIN = 7,
	/// A message sent when a user subscribes to (boosts) a guild
	PREMIUM_GUILD_SUBSCRIPTION = 8,
	/// A message sent when a user subscribes to (boosts) a guild to tier 1
	PREMIUM_GUILD_SUBSCRIPTION_TIER_1 = 9,
	/// A message sent when a user subscribes to (boosts) a guild to tier 2
	PREMIUM_GUILD_SUBSCRIPTION_TIER_2 = 10,
	/// A message sent when a user subscribes to (boosts) a guild to tier 3
	PREMIUM_GUILD_SUBSCRIPTION_TIER_3 = 11,
	/// A message sent when a news channel is followed
	CHANNEL_FOLLOW_ADD = 12,
	/// A message sent when a guild is disqualified from discovery
	GUILD_DISCOVERY_DISQUALIFIED = 14,
	/// A message sent when a guild requalifies for discovery
	GUILD_DISCOVERY_REQUALIFIED = 15,
	/// A message sent when a guild has failed discovery requirements for a week
	GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
	/// A message sent when a guild has failed discovery requirements for 3 weeks
	GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17,
	/// A message sent when a thread is created
	THREAD_CREATED = 18,
	/// A message sent when a user replies to a message
	REPLY = 19,
	/// A message sent when a user uses a slash command
	CHAT_INPUT_COMMAND = 20,
	/// A message sent when a thread starter message is added to a thread
	THREAD_STARTER_MESSAGE = 21,
	/// A message sent to remind users to invite friends to a guild
	GUILD_INVITE_REMINDER = 22,
	/// A message sent when a user uses a context menu command
	CONTEXT_MENU_COMMAND = 23,
	/// A message sent when auto moderation takes an action
	AUTO_MODERATION_ACTION = 24,
	/// A message sent when a user purchases or renews a role subscription
	ROLE_SUBSCRIPTION_PURCHASE = 25,
	/// A message sent when a user is upsold to a premium interaction
	INTERACTION_PREMIUM_UPSELL = 26,
	/// A message sent when a stage channel starts
	STAGE_START = 27,
	/// A message sent when a stage channel ends
	STAGE_END = 28,
	/// A message sent when a user starts speaking in a stage channel
	STAGE_SPEAKER = 29,
	/// A message sent when a user raises their hand in a stage channel
	STAGE_RAISE_HAND = 30,
	/// A message sent when a stage channel's topic is changed
	STAGE_TOPIC = 31,
	/// A message sent when a user purchases an application premium subscription
	GUILD_APPLICATION_PREMIUM_SUBSCRIPTION = 32,
	/// A message sent when a user gifts a premium (Nitro) referral
	PREMIUM_REFERRAL = 35,
	/// A message sent when a user enabled lockdown for the guild
	GUILD_INCIDENT_ALERT_MODE_ENABLED = 36,
	/// A message sent when a user disables lockdown for the guild
	GUILD_INCIDENT_ALERT_MODE_DISABLED = 37,
	/// A message sent when a user reports a raid for the guild
	GUILD_INCIDENT_REPORT_RAID = 38,
	/// A message sent when a user reports a false alarm for the guild
	GUILD_INCIDENT_REPORT_FALSE_ALARM = 39,
	/// A message sent when no one sends a message in the current channel for 1 hour
	GUILD_DEADCHAT_REVIVE_PROMPT = 40,
	/// A message sent when a user buys another user a gift
	CUSTOM_GIFT = 41,
	GUILD_GAMING_STATS_PROMPT = 42,
	/// A message sent when a user purchases a guild product
	PURCHASE_NOTIFICATION = 44,
	/// A message sent when a poll is finalized
	POLL_RESULT = 46,
	/// A message sent by the Discord Updates account when a new changelog is posted
	CHANGELOG = 47,
	/// A message sent when a Nitro promotion is triggered
	NITRO_NOTIFICATION = 48,
	/// A message sent when a voice channel is linked to a lobby
	CHANNEL_LINKED_TO_LOBBY = 49,
	/// A local-only ephemeral message sent when a user is prompted to gift Nitro to a friend on their friendship anniversary
	GIFTING_PROMPT = 50,
	/// A local-only message sent when a user receives an in-game message NUX
	IN_GAME_MESSAGE_NUX = 51,
	/// A message sent when a user accepts a guild join request
	GUILD_JOIN_REQUEST_ACCEPT_NOTIFICATION = 52,
	/// A message sent when a user rejects a guild join request
	GUILD_JOIN_REQUEST_REJECT_NOTIFICATION = 53,
	/// A message sent when a user withdraws a guild join request
	GUILD_JOIN_REQUEST_WITHDRAWN_NOTIFICATION = 54,
	/// A message sent when a user upgrades to HD streaming
	HD_STREAMING_UPGRADED = 55,
}

/// The type of rendered message is determined via converting the message's timestamp to a unix timestamp with millisecond precision, modulo 13.
#[derive(Serialize, Deserialize)]
pub enum UserJoinMessageType {
	JOINED_THE_PARTY = 0,
	IS_HERE = 1,
	WELCOME_WE_HOPE_YOU_BROUGHT_PIZZA = 2,
	A_WILD_APPEARED = 3,
	JUST_LANDED = 4,
	JUST_SLID_INTO_THE_SERVER = 5,
	JUST_SHOWED_UP = 6,
	WELCOME_SAY_HI = 7,
	HOPPED_INTO_THE_SERVER = 8,
	EVERYONE_WELCOME = 9,
	GLAD_YOURE_HERE = 10,
	GOOD_TO_SEE_YOU = 11,
	YAY_YOU_MADE_IT = 12,
}

bitflags! {
	pub struct MessageFlags: u64 {
		/// Message has been published to subscribed channels (via Channel Following)
		const CROSSPOSTED = 1 << 0;
		/// Message originated from a message in another channel (via Channel Following)
		const IS_CROSSPOST = 1 << 1;
		/// Embeds will not be included when serializing this message
		const SUPPRESS_EMBEDS = 1 << 2;
		/// Source message for this crosspost has been deleted (via Channel Following)
		const SOURCE_MESSAGE_DELETED = 1 << 3;
		/// Message came from the urgent message system
		const URGENT = 1 << 4;
		/// Message has an associated thread, with the same ID as the message
		const HAS_THREAD = 1 << 5;
		/// Message is only visible to the user who invoked the interaction
		const EPHEMERAL = 1 << 6;
		/// Message is an interaction response and the bot is "thinking"
		const LOADING = 1 << 7;
		/// Some roles were not mentioned and added to the thread
		const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
		/// Message is hidden from the guild's feed
		const GUILD_FEED_HIDDEN = 1 << 9;
		/// Message contains a link that impersonates Discord
		const SHOULD_SHOW_LINK_NOT_DISCORD_WARNING = 1 << 10;
		/// Message will not trigger push and desktop notifications
		const SUPPRESS_NOTIFICATIONS = 1 << 12;
		/// Message's audio attachment is rendered as a voice message
		const IS_VOICE_MESSAGE = 1 << 13;
		/// Message has a forwarded message snapshot attached
		const HAS_SNAPSHOT = 1 << 14;
		/// Message contains components from version 2 of the UI kit
		const IS_COMPONENTS_V2 = 1 << 15;
		/// Message was triggered by the social layer integration
		const SENT_BY_SOCIAL_LAYER_INTEGRATION = 1 << 16;
	}
}

#[derive(Serialize, Deserialize)]
pub struct MessageActivity {
	/// The type of activity request
	pub r#type: ActivityActionType,
	/// The session ID associated with this activity
	pub session_id: String,
	/// The activity's party ID
	#[serde(skip_serializing_if = "Option::is_none")]
	pub party_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageCall {
	/// The channel recipients that participated in the call
	pub participants: Vec<Snowflake>,
	/// When the call ended, if it has
	pub ended_timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageInteractionMetadata {
	/// The ID of the interaction
	pub id: Snowflake,
	/// The type of interaction
	pub r#type: InteractionType,
	/// The name of the application command executed (including subcommands and subcommand groups), present only on APPLICATION_COMMAND interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The type of application command executed, present only on APPLICATION_COMMAND interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command_type: Option<i64>,
	/// The reason this interaction is ephemeral
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ephemerality_reason: Option<i64>,
	/// The user that initiated the interaction
	pub user: PartialUser,
	/// IDs for each installation context related to an interaction
	pub authorizing_integration_owners: HashMap<i64, Snowflake>,
	/// The ID of the original response message, present only on follow-up messages
	#[serde(skip_serializing_if = "Option::is_none")]
	pub original_response_message_id: Option<Snowflake>,
	/// ID of the message that contained interactive component, present only on messages created from component interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interacted_message_id: Option<Snowflake>,
	/// Metadata for the interaction that was used to open the modal, present only on MODAL_SUBMIT interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub triggering_interaction_metadata: Option<Box<MessageInteractionMetadata>>,
	/// The user that was targeted by the interaction, present only on USER_COMMAND interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_user: Option<PartialUser>,
	/// The ID of the message that was targeted by the interaction, present only on MESSAGE_COMMAND interactions
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_message_id: Option<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EphemeralityReason {
	/// Unknown reason
	NONE = 0,
	/// A required feature is temporarily limited
	FEATURE_LIMITED = 1,
	/// A required feature is temporarily limited for this guild
	GUILD_FEATURE_LIMITED = 2,
	/// A required feature is temporarily limited for this user
	USER_FEATURE_LIMITED = 3,
	/// The user is sending messages past their rate_limit_per_user
	SLOWMODE = 4,
	/// The user is being rate limited
	RATE_LIMIT = 5,
	/// The user does not have permission to message the target user
	CANNOT_MESSAGE_USER = 6,
	/// The user does not meet the guild verification_level requirement
	USER_VERIFICATION_LEVEL = 7,
	/// The user does not have permission to unarchive the thread
	CANNOT_UNARCHIVE_THREAD = 8,
	/// The user does not have permission to join the thread
	CANNOT_JOIN_THREAD = 9,
	/// The user does not have permission to send messages in the channel
	MISSING_PERMISSIONS = 10,
	/// The user does not have permission to send attachments in the channel
	CANNOT_SEND_ATTACHMENTS = 11,
	/// The user does not have permission to send embeds in the channel
	CANNOT_SEND_EMBEDS = 12,
	/// The user does not have permission to send stickers in the channel
	CANNOT_SEND_STICKERS = 13,
	/// The message was blocked by AutoMod
	AUTOMOD_BLOCKED = 14,
	/// The message contains a link blocked by Discord
	HARMFUL_LINK = 15,
	/// The user does not have permission to use this command in this channel
	CANNOT_USE_COMMAND = 16,
	/// The message is only visible to the user for this beta test
	BETA_GUILD_SIZE = 17,
	/// The user does not have permission to use external applications in this channel
	CANNOT_USE_EXTERNAL_APPS = 18,
}

#[derive(Serialize, Deserialize)]
pub struct MessageRoleSubscription {
	/// The ID of the sku and listing that the user is subscribed to
	pub role_subscription_listing_id: Snowflake,
	/// The name of the tier that the user is subscribed to
	pub tier_name: String,
	/// The cumulative number of months that the user has been subscribed for
	pub total_months_subscribed: i64,
	/// Whether this notification is for a renewal rather than a new purchase
	pub is_renewal: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MessagePurchaseNotification {
	/// The type of purchase
	pub r#type: i64,
	/// The guild product purchase that prompted this message
	pub guild_product_purchase: Option<GuildProductPurchase>,
}

/// Determines the type of purchase notification.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessagePurchaseNotificationType {
	/// A guild product purchase
	GUILD_PRODUCT = 0,
}

#[derive(Serialize, Deserialize)]
pub struct GuildProductPurchase {
	/// The ID of the product listing that was purchased
	pub listing_id: Snowflake,
	/// The name of the product that was purchased
	pub product_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageGiftInfo {
	/// The emoji associated with the gift
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Emoji>,
	/// The sound associated with the gift
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<MessageSoundboardSound>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageSoundboardSound {
	/// The ID of the soundboard sound
	pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageReference {
	/// The type of message reference (default DEFAULT )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<MessageReferenceType>,
	/// The ID of the originating message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_id: Option<Snowflake>,
	/// The ID of the originating channel
	pub channel_id: Snowflake,
	/// The ID of the originating channel's guild
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// Whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fail_if_not_exists: Option<bool>,
	/// What to include in the forwarded message
	#[serde(skip_serializing_if = "Option::is_none")]
	pub forward_only: Option<MessageForwardOnly>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageForwardOnly {
	/// The indices of the embeds from the original message to include
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embed_indices: Option<Vec<i64>>,
	/// The IDs of the attachments from the original message to include
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attachment_ids: Option<Vec<Snowflake>>,
}

/// Determines how associated data is populated.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageReferenceType {
	/// A standard reference used by replies and system messages
	DEFAULT = 0,
	/// A reference used to point to a message at a point in time
	FORWARD = 1,
}

#[derive(Serialize, Deserialize)]
pub struct MessageSnapshot {
	/// A snapshot of the message when the forward was created
	pub message: SnapshotMessage,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotMessage {
	/// Contents of the message
	pub content: String,
	/// When this message was sent
	pub timestamp: Timestamp,
	/// when this message was last edited
	pub edited_timestamp: Option<Timestamp>,
	/// Users specifically mentioned in the message
	pub mentions: Vec<PartialUser>,
	/// Roles specifically mentioned in this message
	pub mention_roles: Vec<Snowflake>,
	/// The attached files
	pub attachments: Vec<Attachment>,
	/// Content embedded in the message
	pub embeds: Vec<Embed>,
	/// The type of message
	pub r#type: MessageType,
	/// The message's flags
	pub flags: MessageFlags,
	/// The message's components (e.g. buttons, select menus)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub components: Option<Vec<Component>>,
	/// The resolved data for the select menu component's interaction
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resolved: Option<SelectMenuResolved>,
	/// The message's sticker items
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticker_items: Option<Vec<StickerItem>>,
	/// The message's soundboard sounds
	#[serde(skip_serializing_if = "Option::is_none")]
	pub soundboard_sounds: Option<Vec<SoundboardSound>>,
}

#[derive(Serialize, Deserialize)]
pub struct Reaction {
	/// Total amount of times this emoji has been used to react
	pub count: i64,
	/// Details about the number of times this emoji has been used to react
	pub count_details: ReactionCountDetails,
	/// Whether the current user reacted using this emoji
	pub me: bool,
	/// Whether the current user burst-reacted using this emoji
	pub me_burst: bool,
	/// Reaction emoji information
	pub emoji: Emoji,
	/// The hex-encoded colors to render the burst reaction with
	pub burst_colors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionCountDetails {
	/// Amount of times this emoji has been used to react normally
	pub normal: i64,
	/// Amount of times this emoji has been used to burst-react
	pub burst: i64,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReactionType {
	/// A normal reaction
	NORMAL = 0,
	/// A burst (super) reaction
	BURST = 1,
}

#[derive(Serialize, Deserialize)]
pub struct Embed {
	/// The title of the embed (max 256 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// The type of embed (always rich for sent embeds)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<EmbedType>,
	/// The description of the embed (max 4096 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The URL of the embed (max 2048 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// Timestamp of embed content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<Timestamp>,
	/// The color of the embed encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<i64>,
	/// Embed footer information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub footer: Option<EmbedFooter>,
	/// Embed image information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<EmbedMedia>,
	/// Embed thumbnail information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thumbnail: Option<EmbedMedia>,
	/// Embed video information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video: Option<EmbedMedia>,
	/// Embed provider information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider: Option<EmbedProvider>,
	/// Embed author information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub author: Option<EmbedAuthor>,
	/// The fields of the embed (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fields: Option<Vec<EmbedField>>,
	/// The ID of the message this embed was generated from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reference_id: Option<Snowflake>,
	/// The version of the explicit content scan filter this embed was scanned with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_scan_version: Option<i64>,
	/// The embed's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<EmbedFlags>,
}

/// Most embed types are "loosely defined" and, for the most part, are not used by our clients for rendering. Embed attributes power what is rendered.
#[derive(Serialize, Deserialize)]
pub enum EmbedType {
    /// Application news embed
    #[deprecated]
    application_news,
    /// Article embed
    article,
    /// AutoMod alert
    auto_moderation_message,
    /// AutoMod incident notification
    auto_moderation_notification,
    /// Gift embed
    gift,
    /// Animated GIF image rendered as a video embed
    gifv,
    /// Image embed
    image,
    /// Link embed
    link,
    /// Poll result embed
    poll_result,
    /// Media channel post preview embed
    post_preview,
    /// Generic embed rendered from embed attributes
    rich,
    /// Video embed
    video,
}

bitflags! {
	pub struct EmbedFlags: u64 {
		/// Embed was flagged as sensitive content
		const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
		/// Embed is a legacy content inventory reply
		const CONTENT_INVENTORY_ENTRY = 1 << 5;
	}
}

#[derive(Serialize, Deserialize)]
pub struct EmbedMedia {
	/// Source URL of media (only supports http(s) and attachments) (max 2048 characters)
	pub url: String,
	/// A proxied URL of the media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proxy_url: Option<String>,
	/// Height of media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub height: Option<i64>,
	/// Width of media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<i64>,
	/// The media's attachment flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<AttachmentFlags>,
	/// Alt text for the media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The attachment's media type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_type: Option<String>,
	/// The content scan metadata for the media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_scan_metadata: Option<ContentScanMetadata>,
	/// The attachment placeholder protocol version (currently 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder_version: Option<i64>,
	/// A low-resolution thumbhash of the media, to display before it is loaded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedProvider {
	/// The name of the provider (max 256 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// URL of the provider (max 2048 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedAuthor {
	/// The name of the author (max 256 characters)
	pub name: String,
	/// URL of the author (only supports http(s)) (max 2048 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// Source URL of the author's icon (only supports http(s) and attachments) (max 2048 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
	/// A proxied URL of the author's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedFooter {
	/// The footer text (max 2048 characters)
	pub text: String,
	/// Source URL of the footer icon (only supports http(s) and attachments) (max 2048 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
	/// A proxied URL of the footer icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedField {
	/// The name of the field (max 256 characters)
	pub name: String,
	/// The value of the field (max 1024 characters)
	pub value: String,
	/// Whether this field should display inline (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub inline: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentScanMetadata {
	/// The content scan flags of the media
	pub flags: i64,
	/// The version of the explicit content scan filter this media was scanned with
	pub version: i64,
}

bitflags! {
	pub struct ContentScanFlags: u64 {
		/// The media was flagged as explicit content
		const EXPLICIT = 1 << 0;
		/// The media was flagged as gore
		const GORE = 1 << 1;
	}
}

///
/// When sending/editing messages, only id is required. filename is also required when [uploading to Google Cloud](https://docs.discord.food/reference#uploading-to-google-cloud).
#[derive(Serialize, Deserialize)]
pub struct Attachment {
	/// The attachment ID
	pub id: Snowflake,
	/// The name of file attached (max 1024 characters)
	pub filename: String,
	/// The name of the file without the extension or title of the clip (max 1024 characters, automatically provided when the filename is normalized or randomly generated due to invalid characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// The name of the file pre-uploaded to Discord's GCP bucket
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uploaded_filename: Option<String>,
	/// Alt text for the file (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The attachment's media type
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_type: Option<String>,
	/// The size of file in bytes
	pub size: i64,
	/// Source URL of the file
	pub url: String,
	/// A proxied url of the file
	pub proxy_url: String,
	/// Height of image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub height: Option<Option<i64>>,
	/// Width of image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<Option<i64>>,
	/// The version of the explicit content scan filter this attachment was scanned with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_scan_version: Option<i64>,
	/// The attachment placeholder protocol version (currently 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder_version: Option<i64>,
	/// A low-resolution thumbhash of the attachment, to display before it is loaded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Whether this attachment is ephemeral
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ephemeral: Option<bool>,
	/// Duration of the audio file (if voice message )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub duration_secs: Option<f64>,
	/// Base64 encoded bytearray representing a sampled waveform (if voice message )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub waveform: Option<String>,
	/// The attachment's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<AttachmentFlags>,
	/// Whether the file being uploaded is a clipped recording of a stream
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_clip: Option<bool>,
	/// Whether the file being uploaded is a thumbnail
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_thumbnail: Option<bool>,
	/// Whether this attachment is a remixed version of another attachment
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_remix: Option<bool>,
	/// Whether this attachment is a spoiler
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_spoiler: Option<bool>,
	/// When the clip was created
	#[serde(skip_serializing_if = "Option::is_none")]
	pub clip_created_at: Option<Timestamp>,
	/// The IDs of the participants in the clip (max 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub clip_participant_ids: Option<Vec<Snowflake>>,
	/// The participants in the clip (max 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub clip_participants: Option<Vec<PartialUser>>,
	/// The ID of the application the clip was taken in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<Snowflake>,
	/// The application the clip was taken in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application: Option<PartialApplication>,
}

bitflags! {
	pub struct AttachmentFlags: u64 {
		/// Attachment is a clipped recording of a stream
		const IS_CLIP = 1 << 0;
		/// Attachment is a thumbnail
		const IS_THUMBNAIL = 1 << 1;
		/// Attachment has been remixed
		const IS_REMIX = 1 << 2;
		/// Attachment is a spoiler
		const IS_SPOILER = 1 << 3;
		/// Attachment was flagged as sensitive content
		const CONTAINS_EXPLICIT_MEDIA = 1 << 4;
		/// Attachment is an animated image
		const IS_ANIMATED = 1 << 5;
		/// Attachment was flagged as gore
		const CONTAINS_GORE_CONTENT = 1 << 6;
	}
}

#[derive(Serialize, Deserialize)]
pub enum AllowedMentionTypes {
	/// Controls role mentions
	roles,
	/// Controls user mentions
	users,
	/// Controls @everyone and @here mentions
	everyone,
}

#[derive(Serialize, Deserialize)]
pub struct AllowedMentions {
	/// The allowed mention types to parse from the content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parse: Option<Vec<String>>,
	/// The role IDs to mention (max 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<Snowflake>>,
	/// The user IDs to mention (max 100)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub users: Option<Vec<Snowflake>>,
	/// For replies, whether to mention the author of the message being replied to (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub replied_user: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Poll {
	/// The question of the poll
	pub question: PollMedia,
	/// The answers available in the poll
	pub answers: Vec<PollAnswer>,
	/// When the poll ends
	pub expiry: Option<Timestamp>,
	/// Whether a user can select multiple answers
	pub allow_multiselect: bool,
	/// The layout type of the poll
	pub layout_type: PollLayoutType,
	/// The results of the poll
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<PollResults>,
}

/// This is the request object used when creating a poll across the different endpoints. It is similar but not exactly identical to the main [poll](https://docs.discord.food/resources/message#poll-structure) object. The main difference is that the request has duration which eventually becomes expiry.
#[derive(Serialize, Deserialize)]
pub struct PollCreate {
	/// The question of the poll
	pub question: PollMedia,
	/// Each of the answers available in the poll (max 10)
	pub answers: Vec<PollAnswer>,
	/// Number of hours the poll should be open for (max 32 days, default 1)
	pub duration: i64,
	/// Whether a user can select multiple answers (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allow_multiselect: Option<bool>,
	/// The layout type of the poll (default DEFAULT )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub layout_type: Option<PollLayoutType>,
}

/// Different layouts for polls will come in the future. For now though, this value will always be DEFAULT.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PollLayoutType {
	/// The default layout type
	DEFAULT = 1,
}

/// The poll media object is a common object that backs both the question and answers. For now, question only supports text, while answers can have an optional emoji.
#[derive(Serialize, Deserialize)]
pub struct PollMedia {
	/// The text of the field (max 300 characters for question, 55 characters for answer)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/// The emoji of the field
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Emoji>,
}

/// Currently, there is a maximum of 10 answers per poll.
/// The answer_id is a number that labels each answer. As an implementation detail, it currently starts at 1 for the first answer and goes up sequentially. We recommend against depending on this sequence.
#[derive(Serialize, Deserialize)]
pub struct PollAnswer {
	/// The ID of the answer
	pub answer_id: i64,
	/// The data of the answer
	pub poll_media: PollMedia,
}

/// In a nutshell, this contains the number of votes for each answer. The results field may be not present in certain responses where, as an implementation detail, Discord does not fetch the poll results in the backend. This should be treated as "unknown results", as opposed to "no results". You can keep using the results if you have previously received them through other means. Due to the intricacies of counting at scale, while a poll is in progress the results may not be perfectly accurate. They usually are accurate, and shouldn't deviate significantly—it's just difficult to make guarantees. To compensate for this, after a poll is finished there is a background job which performs a final, accurate tally of votes. This tally concludes once is_finalized is true. Polls that have ended will also always contain results. If answer_counts does not contain an entry for a particular answer, then there are no votes for that answer.
#[derive(Serialize, Deserialize)]
pub struct PollResults {
	/// Whether the votes have been precisely counted
	pub is_finalized: bool,
	/// The counts for each answer
	pub answer_counts: Vec<PollAnswerCount>,
}

#[derive(Serialize, Deserialize)]
pub struct PollAnswerCount {
	/// The ID of the answer
	pub id: i64,
	/// The number of votes for this answer
	pub count: i64,
	/// Whether the current user voted for this answer
	pub me_voted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PollResultEmbed {
	/// The text of the poll question
	pub poll_question_text: String,
	/// The total number of votes on the poll
	pub total_votes: i64,
	/// The ID of the winning answer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victor_answer_id: Option<i64>,
	/// The text of the winning answer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victor_answer_text: Option<String>,
	/// The ID of the emoji of the winning answer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victor_answer_emoji_id: Option<Snowflake>,
	/// The name of the emoji of the winning answer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victor_answer_emoji_name: Option<String>,
	/// Whether the emoji of the winning answer is animated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victor_answer_emoji_animated: Option<bool>,
	/// The number of votes on the winning answer
	pub victor_answer_votes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Potion {
	/// The ID of the user who applied the potion
	pub used_by: Snowflake,
	/// The type of the potion
	pub r#type: PotionType,
	/// The emoji associated with the potion
	pub emoji: Vec<Emoji>,
	/// When the potion was applied
	pub created_at: Timestamp,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PotionType {
	/// A Confetti potion
	CONFETTI = 0,
}

#[derive(Serialize, Deserialize)]
pub struct ConfettiPotion {
	/// The emoji associated with the potion
	pub message_emoji: Emoji,
}

#[derive(Serialize, Deserialize)]
pub struct ConversationSummary {
	/// The ID of the summary
	pub id: Snowflake,
	/// A short description of the topic of the conversation
	pub topic: String,
	/// A brief summary of the conversation
	pub summ_short: String,
	/// The IDs of the messages included in the summary
	pub message_ids: Vec<Snowflake>,
	/// The IDs of the users included in the summary
	pub people: Vec<Snowflake>,
	/// Whether the summary contains potentially unsafe content
	pub r#unsafe: bool,
	/// The ID of the first message in the conversation
	pub start_id: Snowflake,
	/// The ID of the last message in the conversation
	pub end_id: Snowflake,
	/// The number of messages included in the summary
	pub count: i64,
	/// The source of the summary
	pub source: SummarySource,
	/// The type of summary
	pub r#type: SummaryType,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SummarySource {
	/// The summary was generated by source 0
	SOURCE_0 = 0,
	/// The summary was generated by source 1
	SOURCE_1 = 1,
	/// The summary was generated by source 2
	SOURCE_2 = 2,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SummaryType {
	/// The summary type is unset
	UNSET = 0,
	/// The summary was generated by source 1
	SOURCE_1 = 1,
	/// The summary was generated by source 2
	SOURCE_2 = 2,
	/// Unknown
	UNKNOWN = 3,
}

#[derive(Serialize, Deserialize)]
pub struct MessagePin {
	/// When the message was pinned
	pub pinned_at: Timestamp,
	/// The pinned message, without reactions key
	pub message: Message,
}

