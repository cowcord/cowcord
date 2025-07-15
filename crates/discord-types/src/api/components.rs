use std::collections::HashMap;

use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::audit_log::PartialRole;
use crate::api::channel::{ChannelType, PartialChannel};
use crate::api::emoji::Emoji;
use crate::api::guild::GuildMember;
use crate::api::messages::{AttachmentFlags, ContentScanMetadata};
use crate::api::users::PartialUser;
use crate::common::id::{
	ApplicationId,
	AttachmentId,
	ChannelId,
	GenericSnowflake,
	GuildId,
	RoleId,
	SkuId,
	UserId,
};
use crate::common::timestamp::Timestamp;

/// All components have the following fields:
#[derive(Serialize, Deserialize)]
pub struct Component {
	/// The type of the component
	pub r#type: ComponentType,
	/// 32 bit integer used as an optional identifier for component
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<i32>,
	#[serde(flatten)]
	pub component: ComponentType,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentType {
	#[serde(rename = "1")]
	ActionRow(ActionRow),
	#[serde(rename = "2")]
	Button(Button),
	#[serde(rename = "3")]
	StringSelect(StringSelect),
	#[serde(rename = "4")]
	TextInput(TextInput),
	#[serde(rename = "5")]
	UserSelect(UserSelect),
	#[serde(rename = "6")]
	RoleSelect(RoleSelect),
	#[serde(rename = "7")]
	MentionableSelect(MentionableSelect),
	#[serde(rename = "8")]
	ChannelSelect(ChannelSelect),
	#[serde(rename = "9")]
	Section(Section),
	#[serde(rename = "10")]
	TextDisplay(TextDisplay),
	#[serde(rename = "11")]
	Thumbnail(Thumbnail),
	#[serde(rename = "12")]
	MediaGallery(MediaGallery),
	#[serde(rename = "13")]
	File(File),
	#[serde(rename = "14")]
	Separator(Separator),
	#[serde(rename = "16")]
	ContentInventoryEntry(ContentInventoryEntry),
	#[serde(rename = "17")]
	Container(Container),
}

#[derive(Serialize, Deserialize)]
pub struct ActionRow {
	/// Up to 5 button components or a single select component
	pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize)]
pub struct Button {
	/// A button style
	pub style: ButtonStyle,
	/// Text that appears on the button (max 80 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// The emoji that appears on the button
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Emoji>,
	/// Developer-defined identifier for the button (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Identifier for a purchasable SKU, only available when using premium-style buttons
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sku_id: Option<SkuId>,
	/// URL for link-style buttons (max 512 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// Whether the button is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
	PRIMARY = 1,
	SECONDARY = 2,
	SUCCESS = 3,
	DANGER = 4,
	LINK = 5,
	PREMIUM = 6,
}

#[derive(Serialize, Deserialize)]
pub struct StringSelect {
	/// Developer-defined identifier for the select menu (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Specified choices in a select menu (max 25)
	pub options: ArrayVec<SelectOption, 25>,
	/// Placeholder text if nothing is selected (max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Minimum number of items that must be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_values: Option<u8>,
	/// Maximum number of items that can be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_values: Option<u8>,
	/// Whether the select menu is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SelectOption {
	/// User-facing name of the option (max 100 characters)
	pub label: ArrayString<100>,
	/// Developer-defined value of the option (max 100 characters)
	pub value: ArrayString<100>,
	/// Additional description of the option (max 100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Emoji to show next to the name
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Emoji>,
	/// Whether to show this option as selected by default (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default: Option<bool>,
}

/// members and users may both be present in the resolved object when a user is selected (in either a [user select](https://docs.discord.food/resources/components#user-select) or [mentionable select](https://docs.discord.food/resources/components#mentionable-select)).
/// The resolved object is included in interaction payloads for user, role, mentionable, and channel select menu components. resolved contains a nested object with additional details about the selected options.
#[derive(Serialize, Deserialize)]
pub struct SelectMenuResolved {
	/// Selected channels
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channels: Option<HashMap<ChannelId, PartialChannel>>,
	/// Selected roles
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<HashMap<RoleId, PartialRole>>,
	/// Selected users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub users: Option<HashMap<UserId, PartialUser>>,
	/// Selected members
	#[serde(skip_serializing_if = "Option::is_none")]
	pub members: Option<HashMap<UserId, GuildMember>>,
}

#[derive(Serialize, Deserialize)]
pub struct TextInput {
	/// Developer-defined identifier for the input (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// The text input style
	pub style: TextInputStyle,
	/// Label for this component (max 45 characters)
	pub label: ArrayString<45>,
	/// Minimum input length for a text input (max 4000)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_length: Option<u16>,
	/// Maximum input length for a text input (1-4000)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_length: Option<u16>,
	/// Whether this component is required to be filled (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required: Option<bool>,
	/// Prefilled value for the component (max 4000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
	/// Custom placeholder text if the input is empty (max 100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TextInputStyle {
	/// Single-line input
	SMALL = 1,
	/// Multi-line input
	PARAGRAPH = 2,
}

#[derive(Serialize, Deserialize)]
pub struct UserSelect {
	/// Developer-defined identifier for the select menu (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Placeholder text if nothing is selected (max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Default values for auto-populated select menu components (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_values: Option<Vec<SelectDefaultValue>>,
	/// Minimum number of items that must be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_values: Option<u8>,
	/// Maximum number of items that can be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_values: Option<u8>,
	/// Whether the select menu is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SelectDefaultValue {
	/// ID of a user, role, or channel
	pub id: UserRoleChannelId,
	/// Type of value associated with the ID
	pub r#type: String,
}

#[derive(Serialize, Deserialize)]
enum UserRoleChannelId {
	UserId(UserId),
	RoleId(RoleId),
	ChannelId(ChannelId),
}

#[derive(Serialize, Deserialize)]
pub struct RoleSelect {
	/// Developer-defined identifier for the select menu (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Placeholder text if nothing is selected (max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Default values for auto-populated select menu components (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_values: Option<ArrayVec<SelectDefaultValue, 25>>,
	/// Minimum number of items that must be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_values: Option<u8>,
	/// Maximum number of items that can be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_values: Option<u8>,
	/// Whether the select menu is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct MentionableSelect {
	/// Developer-defined identifier for the select menu (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Placeholder text if nothing is selected (max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Default values for auto-populated select menu components (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_values: Option<ArrayVec<SelectDefaultValue, 25>>,
	/// Minimum number of items that must be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_values: Option<u8>,
	/// Maximum number of items that can be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_values: Option<u8>,
	/// Whether the select menu is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelSelect {
	/// Developer-defined identifier for the select menu (max 100 characters)
	pub custom_id: ArrayString<100>,
	/// Channel types to include in the channel select component
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_types: Option<Vec<ChannelType>>,
	/// Placeholder text if nothing is selected (max 150 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// Default values for auto-populated select menu components (max 25)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_values: Option<ArrayVec<SelectDefaultValue, 25>>,
	/// Minimum number of items that must be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_values: Option<u8>,
	/// Maximum number of items that can be chosen (max 25, default 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_values: Option<u8>,
	/// Whether the select menu is disabled (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Section {
	/// Text components to display (1-3)
	pub components: Vec<TextDisplay>,
	/// A thumbnail or a button component
	pub accessory: SectionAccessory,
}

#[derive(Serialize, Deserialize)]
enum SectionAccessory {
	Button(Button),
	Thumbnail(Thumbnail),
}

#[derive(Serialize, Deserialize)]
pub struct TextDisplay {
	/// Text that will be displayed similar to a message (1-4000 characters)
	pub content: ArrayString<4000>,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
	/// A URL or attachment
	pub media: UnfurledMediaItem,
	/// Alt text for the media (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Whether the thumbnail should be spoilered (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spoiler: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct MediaGallery {
	/// Items to display in the gallery (1-10)
	pub items: Vec<MediaGalleryItem>,
}

#[derive(Serialize, Deserialize)]
pub struct MediaGalleryItem {
	/// A URL or attachment
	pub media: UnfurledMediaItem,
	/// Alt text for the media (max 1024 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Whether the media should be a spoilered (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spoiler: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
	/// The file attachment (does not support URLs)
	pub file: UnfurledMediaItem,
	/// Whether the media should be a spoiler (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spoiler: Option<bool>,
	/// The name of the file
	pub name: String,
	/// The size of the file in bytes
	pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Separator {
	/// Whether a visual divider should be displayed in the component (default true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub divider: Option<bool>,
	/// Size of separator padding (default SMALL )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spacing: Option<SeparatorSpacingType>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SeparatorSpacingType {
	/// 8px gap between elements, 16px with divider hidden
	SMALL = 1,
	/// 16px gap between elements, 32px with divider hidden
	LARGE = 2,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryEntryStructure {
	/// Content inventory entry data
	pub content_inventory_entry: Box<ContentInventoryEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryEntry {
	/// The ID of the entry as a snowflake when sent in guild or a string of numbers when DMed
	pub id: String,
	/// The ID of the user this entry is for
	pub author_id: UserId,
	/// The type of author that created this entry
	pub author_type: ContentInventoryAuthorType,
	/// [The type of content] ](#content-inventory-content-type)
	pub content_type: ContentInventoryContentType,
	/// Contains info such as streak, marathon, and time
	pub traits: Vec<ContentInventoryTrait>,
	/// Metadata, such as a game or song
	pub extra: ContentInventoryMetadata,
	/// The IDs of all users involved with this entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub participants: Option<Vec<UserId>>,
	/// When this entry expires
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<Timestamp>,
	/// When this entry ended
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ended_at: Option<Timestamp>,
	/// When this entry started
	#[serde(skip_serializing_if = "Option::is_none")]
	pub started_at: Option<Timestamp>,
	/// ID of the entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub original_id: Option<GenericSnowflake>,
	/// Guild ID this entry happened in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<GuildId>,
	/// Channel ID this entry happened in
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel_id: Option<ChannelId>,
	/// Session ID of this entry
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<GenericSnowflake>,
	/// Signature metadata for validation
	pub signature: ContentInventorySignature,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryAuthorType {
	/// No author type specified
	AUTHOR_TYPE_UNSPECIFIED = 0,
	/// A Discord user
	USER = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryContentType {
	/// No content type specified
	CONTENT_TYPE_UNSPECIFIED = 0,
	/// A game that was played
	PLAYED_GAME = 1,
	/// Media that was watched (e.g. Crunchyroll)
	WATCHED_MEDIA = 2,
	/// Top played game in the guild
	TOP_GAME = 3,
	/// Media that was listened to (e.g. Spotify)
	LISTENED_MEDIA = 4,
	/// Media listening session (e.g. Spotify Listen Along)
	LISTENED_SESSION = 5,
	/// Top listened artist in the guild
	TOP_ARTIST = 6,
	/// Custom status activity
	CUSTOM_STATUS = 7,
	/// Embedded activity
	LAUNCHED_ACTIVITY = 8,
	/// Leaderboard entry (e.g. League of Legends)
	LEADERBOARD = 9,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryTrait {
	/// The type of trait
	pub r#type: ContentInventoryTraitType,
	/// Shows the "New player" text (only FIRST_TIME )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_time: Option<bool>,
	/// Total time elapsed during the entry (only DURATION_SECONDS )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub duration_seconds: Option<u64>,
	/// Whether the entry is still ongoing (only IS_LIVE )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_live: Option<bool>,
	/// Time range (only AGGREGATE_RANGE )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range: Option<ContentInventoryAggregateRangeType>,
	/// When the game for the entry was last played (only RESURRECTED )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resurrected_last_played: Option<Timestamp>,
	/// Shows the "#h marathon" text (only MARATHON )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub marathon: Option<bool>,
	/// Number of days for the streak text (only STREAK_DAYS )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub streak_count_days: Option<u64>,
	/// The trending type (only TRENDING_CONTENT )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trending: Option<ContentInventoryTrendingType>,
	/// Total count (only TOP_ITEM_TOTAL_COUNT , TOP_PARENT_ITEM_TOTAL_COUNT , AGGREGATE_COUNT )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub count: Option<u64>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryTraitType {
	/// No trait type specified
	TRAIT_TYPE_UNSPECIFIED = 0,
	/// First time the content was played
	FIRST_TIME = 1,
	/// Total duration in seconds
	DURATION_SECONDS = 2,
	/// Whether the content is currently live
	IS_LIVE = 3,
	/// Time range for the aggregated data
	AGGREGATE_RANGE = 4,
	/// Whether the user is returning to content previously interacted with (e.g. game)
	RESURRECTED = 5,
	/// Whether the content is part of a marathon
	MARATHON = 6,
	/// Whether the content is a new release
	NEW_RELEASE = 7,
	/// Number of days in the streak
	STREAK_DAYS = 8,
	/// Whether the content is trending
	TRENDING_CONTENT = 9,
	/// Total count of the top item in the guild
	TOP_ITEM_TOTAL_COUNT = 10,
	/// Total count of the top parent item in the guild
	TOP_PARENT_ITEM_TOTAL_COUNT = 11,
	/// Aggregate count of the content in the guild
	AGGREGATE_COUNT = 12,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryAggregateRangeType {
	/// No aggregate range specified
	AGGREGATE_RANGE_UNSPECIFIED = 0,
	/// Last 7 days of data
	WEEK = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryTrendingType {
	/// No trending type specified
	TRENDING_TYPE_UNSPECIFIED = 0,
	/// Global trending content
	GLOBAL = 1,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryMetadata {
	/// Metadata type
	pub r#type: ContentInventoryMetadataType,
	/// The name of the game ( played_game_extra only)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game_name: Option<String>,
	/// The ID of the associated application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<ApplicationId>,
	/// The type of platform (only played_game_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<ContentInventoryPlatformType>,
	/// When the entry was last updated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_update: Option<Timestamp>,
	/// Entries (only 1 for the component)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entries: Option<Vec<Box<ContentInventoryMetadataEntry>>>,
	/// Metadata object of type listened_media_extra
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media: Option<Box<ContentInventoryMetadataEntry>>,
	/// The provider of the media (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provider: Option<ContentInventoryProviderType>,
	/// The type of media (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_type: Option<ContentInventoryMediaType>,
	/// The title of the media's parent (album title when media_type is TRACK ) (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_title: Option<String>,
	/// The title of the media (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// Image for the media (e.g. album art) (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/// Top listened artist (only top_artist_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub artist: Option<ContentInventoryArtist>,
	/// Artists for the media (empty when top_artist_extra ) (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub artists: Option<Vec<ContentInventoryArtist>>,
	/// The external platform ID for the media (e.g. Spotify track ID) (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub external_id: Option<String>,
	/// The external platform ID for the media's parent (e.g. Spotify album ID) (only listened_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub external_parent_id: Option<String>,
	/// The large image for the media (e.g. movie poster) (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_assets_large_image: Option<String>,
	/// Text displayed when hovering over the large image (e.g. season and episode of a show) (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_assets_large_text: Option<String>,
	/// Small image for the media (e.g. platform logo) (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_assets_small_image: Option<String>,
	/// Text displayed when hovering over the small image (e.g. platform name) (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_assets_small_text: Option<String>,
	/// The title of the media (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_title: Option<String>,
	/// The subtitle of the media (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media_subtitle: Option<String>,
	/// The URL of the media (only watched_media_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// The name of the activity (only launched_activity_extra )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub activity_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ContentInventoryMetadataType {
	/// Game
	played_game_extra,
	/// Listened session (e.g. Spotify)
	listened_session_extra,
	/// Listened media (e.g. Spotify track)
	listened_media_extra,
	/// Top artist (e.g. Spotify)
	top_artist_extra,
	/// Watched media (e.g. Crunchyroll)
	watched_media_extra,
	/// Embedded activity
	launched_activity_extra,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryProviderType {
	/// No provider specified
	PROVIDER_UNSPECIFIED = 0,
	/// Spotify
	SPOTIFY = 1,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryMediaType {
	/// Top artist in the guild
	TOP_ARTIST = 0,
	/// Track in a media provider (e.g. Spotify)
	TRACK = 1,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryMetadataEntry {
	/// Metadata object of type listened_media_extra
	#[serde(skip_serializing_if = "Option::is_none")]
	pub media: Option<ContentInventoryMetadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verification_state: Option<u8>,
	/// How many times this track has been played on repeat
	#[serde(skip_serializing_if = "Option::is_none")]
	pub repeat_count: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventoryArtist {
	/// The external platform ID for this artist (e.g. Spotify artist ID)
	pub external_id: String,
	/// The ame of the artist
	pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContentInventoryPlatformType {
	/// Desktop
	DESKTOP = 0,
	/// Xbox integration
	XBOX = 1,
	/// PlayStation integration
	PLAYSTATION = 2,
	/// iOS
	IOS = 3,
	/// Android
	ANDROID = 4,
	/// Nintendo integration
	NINTENDO = 5,
	/// Linux
	LINUX = 6,
	/// macOS
	MACOS = 7,
}

#[derive(Serialize, Deserialize)]
pub struct ContentInventorySignature {
	/// SHA256 hash of the entry
	pub signature: String,
	/// Key ID for the signature
	pub kid: String,
	/// Signature version (currently 1)
	pub version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
	/// Components of the type action row , text display , section , media gallery , separator , or file (1-10)
	pub components: Vec<Component>,
	/// Color for the accent on the container encoded as an integer representation of a hexadecimal color code
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accent_color: Option<Option<u32>>,
	/// Whether the container should be spoilered (default false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spoiler: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UnfurledMediaItem {
	/// The URL of the media, supports arbitrary URLs and attachment://<filename> references (max 2048 characters)
	pub url: ArrayString<2048>,
	/// The proxied URL of the media item
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proxy_url: Option<String>,
	/// The height of the media item
	#[serde(skip_serializing_if = "Option::is_none")]
	pub height: Option<Option<u16>>,
	/// The width of the media item
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<Option<u16>>,
	/// The media's attachment flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<AttachmentFlags>,
	/// The media type of the content
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_type: Option<String>,
	/// The content scan metadata for the media
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_scan_metadata: Option<ContentScanMetadata>,
	/// The attachment placeholder protocol version (currently 1)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder_version: Option<u8>,
	/// A low-resolution thumbhash of the media, to display before it is loaded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub placeholder: Option<String>,
	/// The loading state of the media item
	#[serde(skip_serializing_if = "Option::is_none")]
	pub loading_state: Option<UnfurledMediaItemLoadingState>,
	/// The ID of the uploaded attachment, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attachment_id: Option<AttachmentId>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UnfurledMediaItemLoadingState {
	/// Loading state is unknown
	UNKNOWN = 0,
	/// Media item is currently loading
	LOADING = 1,
	/// Media item has loaded successfully
	LOADED_SUCCESS = 2,
	/// Media item has loaded but was not found
	LOADED_NOT_FOUND = 3,
}
