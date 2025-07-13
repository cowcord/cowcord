use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
pub struct Session {
	/// The ID of the session
	pub session_id: String,
	/// Information about the client that spawned the session
	pub client_info: ClientInfo,
	/// The status of the session
	pub status: StatusType,
	/// The current activities the session is partaking in
	pub activities: Vec<Activity>,
	/// Activities that are hidden from the public
	pub hidden_activities: Vec<Activity>,
	/// Unknown
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
	/// The type of client
	pub client: ClientType,
	/// The operating system of the client
	pub os: OperatingSystemType,
	/// The version of the client type (e.g. 5 for the PS5)
	pub version: i64,
}

#[derive(Serialize, Deserialize)]
pub enum ClientType {
	/// Desktop client
	desktop,
	/// Web-based client
	web,
	/// Mobile client
	mobile,
	/// Unknown
	unknown,
}

#[derive(Serialize, Deserialize)]
pub enum OperatingSystemType {
	/// Windows
	windows,
	/// macOS
	osx,
	/// Linux
	linux,
	/// Android
	android,
	/// iOS
	ios,
	/// PlayStation
	playstation,
	/// Xbox
	xbox,
	/// Unknown
	unknown,
}

/// Active sessions are indicated with a status per platform. If a user is offline or invisible, the corresponding field is not present.
#[derive(Serialize, Deserialize)]
pub struct ClientStatus {
	/// The user's status on an active desktop (Windows, Linux, Mac) application session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub desktop: Option<String>,
	/// The user's status on an active mobile (iOS, Android) application session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<String>,
	/// The user's status on an active web (browser) application session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web: Option<String>,
	/// The user's status on an active embedded (Xbox, PlayStation) session
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embedded: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum StatusType {
	/// Online
	online,
	/// Do Not Disturb
	dnd,
	/// AFK
	idle,
	/// Shown as offline
	invisible,
	/// Offline
	offline,
	/// Unknown
	unknown,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
	/// The ID of the activity; only unique across a single user's activities
	pub id: String,
	/// The name of the activity (2-128 characters)
	pub name: String,
	/// The activity type
	pub r#type: ActivityType,
	/// The stream URL (max 512 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<Option<String>>,
	/// Unix timestamp (in milliseconds) of when the activity was added to the user's session
	pub created_at: i64,
	/// The ID of the session associated with the activity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<Option<String>>,
	/// The platform the activity is being played on
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<ActivityPlatformType>,
	/// The platforms the activity is supported on
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_platforms: Option<Vec<ActivityPlatformType>>,
	/// Unix timestamps (in milliseconds) for start and/or end of the game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamps: Option<ActivityTimestamps>,
	/// The ID of the application representing the game the user is playing
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application_id: Option<Snowflake>,
	/// What the user is currently doing (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub details: Option<Option<String>>,
	/// The user's current party status (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub state: Option<Option<String>>,
	/// The ID of the synced activity (e.g. Spotify song ID)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sync_id: Option<String>,
	/// The activity's flags
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<ActivityFlags>,
	/// Custom buttons shown in rich presence (max 2)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub buttons: Option<Vec<String>>,
	/// The emoji used for a custom status
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emoji: Option<Option<ActivityEmoji>>,
	/// Information for the current party of the user
	#[serde(skip_serializing_if = "Option::is_none")]
	pub party: Option<ActivityParty>,
	/// Images for the presence and their hover texts
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assets: Option<ActivityAssets>,
	/// Secrets for rich presence joining and spectating
	#[serde(skip_serializing_if = "Option::is_none")]
	pub secrets: Option<ActivitySecrets>,
	/// Additional metadata for the activity
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<ActivityMetadata>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityType {
	PLAYING = 0,
	STREAMING = 1,
	LISTENING = 2,
	WATCHING = 3,
	CUSTOM = 4,
	COMPETING = 5,
}

#[derive(Serialize, Deserialize)]
pub enum ActivityPlatformType {
	/// Desktop (headless)
	desktop,
	/// Xbox integration
	xbox,
	/// Samsung integration
	samsung,
	/// iOS
	ios,
	/// Android
	android,
	/// Embedded session
	embedded,
	/// PlayStation 4 integration
	ps4,
	/// PlayStation 5 integration
	ps5,
}

bitflags! {
	pub struct ActivityFlags: u64 {
		/// Activity is an instanced game session (a match that will end)
		const INSTANCE = 1 << 0;
		/// Activity can be joined by other users
		const JOIN = 1 << 1;
		/// Activity can be spectated by other users
		#[deprecated]
		const SPECTATE = 1 << 2;
		/// Activity can be synced
		const SYNC = 1 << 4;
		/// Activity can be played
		const PLAY = 1 << 5;
		/// Activity's party can be joined by friends
		const PARTY_PRIVACY_FRIENDS = 1 << 6;
		/// Activity's party can be joined by users in the same voice channel
		const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
		/// Activity is embedded within the Discord client
		const EMBEDDED = 1 << 8;
	}
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityActionType {
	/// Allows others to join a game with the user
	JOIN = 1,
	/// Allows others to spectate a game the user is playing
	#[deprecated]
	SPECTATE = 2,
	/// Allows others to listen to a song with the user
	LISTEN = 3,
	/// Asks others to invite the user to a game
	JOIN_REQUEST = 5,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityTimestamps {
	/// Unix time (in milliseconds) of when the activity starts
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<String>,
	/// Unix time (in milliseconds) of when the activity ends
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityEmoji {
	/// The name of the emoji
	pub name: String,
	/// The ID of the emoji
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<Snowflake>,
	/// Whether this emoji is animated
	#[serde(skip_serializing_if = "Option::is_none")]
	pub animated: Option<bool>,
}


#[derive(Serialize, Deserialize)]
pub struct ActivityParty {
	/// The ID of the party (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The party's current and maximum size (current_size, max_size)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<(i64, i64)>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityAssets {
	/// The large [activity asset image](https://docs.discord.food/resources/presence#activity-asset-image) (max 313 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub large_image: Option<String>,
	/// Text displayed when hovering over the large image of the activity (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub large_text: Option<String>,
	/// The small [activity asset image](https://docs.discord.food/resources/presence#activity-asset-image) (max 313 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub small_image: Option<String>,
	/// Text displayed when hovering over the small image of the activity (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub small_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivitySecrets {
	/// The secret for joining a party (max 128 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub join: Option<String>,
	/// The secret for spectating a game (max 128 characters)
	#[deprecated]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spectate: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum OperatingSystemDesktopType {
	/// Windows
	win32,
	/// macOS
	darwin,
	/// Linux
	linux,
}

#[derive(Serialize, Deserialize)]
pub enum CustomStatusLabelType {
	question,
	think,
	love,
	excited,
	recommend,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityMetadata {
	/// The URLs corresponding to the custom buttons shown in rich presence (max 2)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub button_urls: Option<Vec<String>>,
	/// The Spotify IDs of the artists of the song being played
	#[serde(skip_serializing_if = "Option::is_none")]
	pub artist_ids: Option<Vec<String>>,
	/// The Spotify ID of the album of the song being played
	#[serde(skip_serializing_if = "Option::is_none")]
	pub album_id: Option<String>,
	/// The Spotify URI of the current player context
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context_uri: Option<String>,
	/// The type of Spotify track being played ( track or episode )
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ActivityMetadataType {
    track,
    episode,
}
