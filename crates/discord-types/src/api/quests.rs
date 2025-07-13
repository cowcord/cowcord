use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::id::{ApplicationId, QuestId, SkuId, UserId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct Quest {
	/// The ID of the quest
	pub id: QuestId,
	/// The configuration and metadata for the quest
	pub config: QuestConfig,
	/// The user's quest progress, if it has been accepted
	pub user_status: Option<QuestUserStatus>,
	/// The content areas where the quest can be shown
	#[deprecated]
	pub targeted_content: Option<Vec<QuestContentType>>,
	/// Whether the quest is unreleased and in preview for Discord employees
	pub preview: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PartialQuest {
	/// The ID of the quest
	pub id: QuestId,
}

/// The config structure has multiple distinct versions with different field sets. Only actively used versions are kept documented. As of now, only the latest version is available.
#[derive(Serialize, Deserialize)]
pub struct QuestConfig {
	/// The ID of the quest
	pub id: QuestId,
	/// Quest configuration version
	pub config_version: u8,
	/// When the quest period starts
	pub starts_at: Timestamp,
	/// When the quest period ends
	pub expires_at: Timestamp,
	/// The quest features enabled for the quest
	pub features: Vec<QuestFeatures>,
	/// The application metadata for the quest
	pub application: QuestApplication,
	/// Object that holds the quest's assets
	pub assets: QuestAssets,
	/// The accent colors for the quest
	pub colors: QuestGradient,
	/// Human-readable metadata for the quest
	pub messages: QuestMessages,
	/// The task configuration for the quest
	pub task_config: QuestTaskConfig,
	/// Specifies rewards for the quest (e.g. collectibles)
	pub rewards_config: QuestRewardsConfig,
	/// The configuration for the video quest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video_metadata: Option<QuestVideoMetadata>,
	/// The configuration for the quest co-sponsor
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cosponsor_metadata: Option<QuestCosponsorMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestApplication {
	/// The ID of the application
	pub id: ApplicationId,
	/// The name of the application
	pub name: String,
	/// The link to the game's page
	pub link: String,
}

/// An object holding [CDN asset names](https://docs.discord.food/reference#cdn-formatting).
#[derive(Serialize, Deserialize)]
pub struct QuestAssets {
	/// The quest's hero image
	pub hero: String,
	/// A video representation of the hero image
	pub hero_video: Option<String>,
	/// The hero image used in the quest popup that appears when launching the game before accepting the quest
	pub quest_bar_hero: String,
	/// A video representation of the quest bar hero image
	pub quest_bar_hero_video: Option<String>,
	/// The game's icon
	pub game_tile: String,
	/// The game's logo
	pub logotype: String,
}

/// A 2-point gradient with a primary and secondary color.
#[derive(Serialize, Deserialize)]
pub struct QuestGradient {
	/// The hex-encoded primary color of the gradient
	pub primary: String,
	/// The hex-encoded secondary color of the gradient
	pub secondary: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestMessages {
	/// The name of the quest
	pub quest_name: String,
	/// The title of the game the quest is for
	pub game_title: String,
	/// The publisher of the game the quest is for
	pub game_publisher: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestTaskConfig {
	/// The type of task configuration
	pub r#type: QuestTaskConfigType,
	/// The eligibility operator used to join multiple tasks ( and or or )
	pub join_operator: String,
	/// Tasks required to complete the quest, keyed by their event name
	pub tasks: HashMap<QuestTaskEventName, QuestTask>,
	/// A link to the third-party quest tasks enrollment page
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enrollment_url: Option<String>,
	/// The ID of the embedded activity for the third-party task
	#[serde(skip_serializing_if = "Option::is_none")]
	pub developer_application_id: Option<ApplicationId>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestTask {
	/// The type of task event
	pub event_name: QuestTaskEventName,
	/// The required value
	pub target: u32,
	/// IDs of the target game on console platforms
	#[serde(skip_serializing_if = "Option::is_none")]
	pub external_ids: Option<Vec<String>>,
	/// The third-party task title
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// The third-party task description
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestTaskConfigType {
	/// The tasks are first-party
	FIRST_PARTY = 1,
	/// The tasks are third-party
	THIRD_PARTY = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QuestTaskEventName {
	/// The user must play and stream the game on desktop to at least one other user for a certain duration (see Update Activity Session )
	STREAM_ON_DESKTOP,
	/// The user must play the game on desktop for a certain duration (see Update Activity Session )
	PLAY_ON_DESKTOP,
	/// The user must play the game on desktop for a certain duration (see Update Activity Session )
	PLAY_ON_DESKTOP_V2,
	/// The user must play the game on Xbox for a certain duration
	PLAY_ON_XBOX,
	/// The user must play the game on PlayStation for a certain duration
	PLAY_ON_PLAYSTATION,
	/// The user must watch a video for a certain duration
	WATCH_VIDEO,
	/// The user must watch a video on mobile for a certain duration
	WATCH_VIDEO_ON_MOBILE,
	/// The user must play the embedded activity for a certain duration
	PLAY_ACTIVITY,
	/// The user must complete an achievement in the game
	ACHIEVEMENT_IN_GAME,
	/// The user must complete an achievement in the embedded activity
	ACHIEVEMENT_IN_ACTIVITY,
	/// The user must complete a certain number of tasks in an embedded activity
	progress,
}

#[derive(Serialize, Deserialize)]
pub struct QuestRewardsConfig {
	/// How the rewards are assigned
	pub assignment_method: QuestRewardAssignmentMethod,
	/// The possible rewards for the quest, ordered by tier (if applicable)
	pub rewards: Vec<QuestReward>,
	/// When the reward claiming period ends
	pub rewards_expire_at: Option<Timestamp>,
	/// The platforms the rewards can be redeemed on
	pub platforms: Vec<QuestPlatformType>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestReward {
	/// The reward's type
	pub r#type: QuestRewardType,
	/// The ID of the SKU awarded
	pub sku_id: SkuId,
	/// The reward's media asset
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset: Option<Option<String>>,
	/// The reward's video asset
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_video: Option<Option<String>>,
	/// Human-readable metadata for the reward
	pub messages: QuestRewardMessages,
	/// An approximate count of how many users can claim the reward
	#[serde(skip_serializing_if = "Option::is_none")]
	pub approximate_count: Option<Option<u32>>,
	/// The link to redeem the reward
	#[serde(skip_serializing_if = "Option::is_none")]
	pub redemption_link: Option<Option<String>>,
	/// When the reward expires
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<Option<Timestamp>>,
	/// When the reward expires for premium users
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at_premium: Option<Option<Timestamp>>,
	/// The expiration mode
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_mode: Option<QuestRewardExpirationMode>,
	/// The amount of Discord Orbs awarded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orb_quantity: Option<u32>,
	/// The days of fractional premium awarded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quantity: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestRewardMessages {
	/// The reward's name
	pub name: String,
	/// The article variant of the name (e.g. a Cybernetic Headgear Decoration)
	pub name_with_article: String,
	/// The instrutions on redeeming the reward per-platform
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reward_redemption_instructions_by_platform: Option<HashMap<QuestPlatformType, String>>,
}

/// The method used to assign the reward to a user.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestRewardAssignmentMethod {
	/// All rewards are assigned to the user upon completion
	ALL = 1,
	/// The rewards are assigned in tiers
	TIERED = 2,
}

/// The type of reward that the user will receive.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestRewardType {
	/// The reward is a redeemable code
	REWARD_CODE = 1,
	/// The reward is automatically given to the user in the promoted game
	IN_GAME = 2,
	/// The reward is a Discord collectible (e.g. an avatar decoration)
	COLLECTIBLE = 3,
	/// The reward is a virtual currency (Discord Orbs)
	VIRTUAL_CURRENCY = 4,
	/// The reward is a limited free premium (Nitro) trial for a fraction of a billing period
	FRACTIONAL_PREMIUM = 5,
}

/// Controls the expiration behavior of COLLECTIBLE rewards.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestRewardExpirationMode {
	/// The reward expires after a set period of time
	NORMAL = 1,
	/// The reward lasts longer for premium (Nitro) users
	PREMIUM_EXTENSION = 2,
	/// The reward is permanent for premium (Nitro) users, even after their subscription has ended
	PREMIUM_PERMANENT = 3,
}

#[derive(Serialize, Deserialize)]
pub struct QuestVideoMetadata {
	/// Human-readable metadata for the video quest
	pub messages: QuestVideoMessages,
	/// Object that holds the quest's video assets
	pub assets: QuestVideoAssets,
}

#[derive(Serialize, Deserialize)]
pub struct QuestVideoMessages {
	/// The title of the video
	pub video_title: String,
	/// The title of the call-to-action at the end of the video
	pub video_end_cta_title: String,
	/// The subtitle of the call-to-action at the end of the video
	pub video_end_cta_subtitle: String,
	/// The label of the call-to-action button at the end of the video
	pub video_end_cta_button_label: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestVideoAssets {
	/// The HLS video asset for the video player
	pub video_player_video_hls: Option<String>,
	/// The video asset for the video player
	pub video_player_video: String,
	/// The thumbnail asset for the video player
	pub video_player_thumbnail: Option<String>,
	/// The low-resolution video asset for the video player
	pub video_player_video_low_res: String,
	/// The caption asset for the video player
	pub video_player_caption: String,
	/// The transcript asset for the video player
	pub video_player_transcript: String,
	/// The video asset for the quest bar preview
	pub quest_bar_preview_video: Option<String>,
	/// The thumbnail asset for the quest bar preview
	pub quest_bar_preview_thumbnail: Option<String>,
	/// The video asset for the quest home page
	pub quest_home_video: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestCosponsorMetadata {
	/// The name of the co-sponsor
	pub name: String,
	/// The co-sponsor's logo asset
	pub logotype: String,
	/// The co-sponsor's redemption instructions
	pub redemption_instructions: String,
}

/// Areas where the quest can be shown in the Discord client.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestContentType {
	/// This quest is shown as a badge in User Settings
	GIFT_INVENTORY_SETTINGS_BADGE = 0,
	/// This quest is shown as a bar above the user popout
	QUEST_BAR = 1,
	/// This quest is shown as a card in the user's gift inventory
	QUEST_INVENTORY_CARD = 2,
	/// This quest is shown as an embed in chat
	QUESTS_EMBED = 3,
	/// This quest is shown in the Active Now page
	ACTIVITY_PANEL = 4,
	/// This quest is shown while watching a stream
	QUEST_LIVE_STREAM = 5,
	/// This quest is shown in the member list
	MEMBERS_LIST = 6,
	/// This quest is shown on the quest profile badge upsell
	QUEST_BADGE = 7,
	/// This quest is featured in the user's gift inventory for you section
	GIFT_INVENTORY_FOR_YOU = 8,
	/// This quest is featured in the user's gift inventory
	GIFT_INVENTORY_OTHER = 9,
	/// This quest is shown in the new quest bar design
	QUEST_BAR_V2 = 10,
	/// This quest is shown on the desktop Quest discovery page
	QUEST_HOME_DESKTOP = 11,
	/// This quest is shown on the mobile Quest discovery page
	QUEST_HOME_MOBILE = 12,
	/// This quest is shown in the mobile Quest bar design
	QUEST_BAR_MOBILE = 13,
	/// This quest is shown in a third-party app
	THIRD_PARTY_APP = 14,
	/// This quest is shown in the bottom sheet
	QUEST_BOTTOM_SHEET = 15,
	/// This quest is shown in the mobile Quest embed
	QUEST_EMBED_MOBILE = 16,
	/// This quest is shown in the move callout on the Quest discovery page
	QUEST_HOME_MOVE_CALLOUT = 17,
	/// This quest is shown in the discovery sidebar
	DISCOVERY_SIDEBAR = 18,
	/// This quest is eligible to be shared as a link
	QUEST_SHARE_LINK = 19,
	/// This quest is shown in the user connections modal
	CONNECTIONS_MODAL = 20,
	/// This quest is shown on the discovery button
	DISCOVERY_COMPASS = 21,
	/// This quest is shown as a card in the user's trophy case
	TROPHY_CASE_CARD = 22,
	/// This quest has a video modal
	VIDEO_MODAL = 23,
	/// This quest has an end card in the video modal
	VIDEO_MODAL_END_CARD = 24,
	/// This quest is shown in the reward modal
	REWARD_MODAL = 25,
	/// This quest is excluded from the Quest embed
	EXCLUDED_QUEST_EMBED = 26,
	/// This quest is shown in the mobile video modal
	VIDEO_MODAL_MOBILE = 27,
}

/// Specifies the platforms that the quest reward can be redeemed on.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QuestPlatformType {
	/// This reward can be redeemed on all platforms
	CROSS_PLATFORM = 0,
	/// This reward can be redeemed on Xbox
	XBOX = 1,
	/// This reward can be redeemed on PlayStation
	PLAYSTATION = 2,
	/// This reward can be redeemed on Nintendo Switch
	SWITCH = 3,
	/// This reward can be redeemed on PC
	PC = 4,
}

/// A behavioral variant for a quest.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QuestFeatures {
	/// The quest has a post-enrollment call-to-action
	POST_ENROLLMENT_CTA = 1,
	/// The quest has a playtime criteria
	PLAYTIME_CRITERIA = 2,
	/// The quest uses the new quest bar design
	QUEST_BAR_V2 = 3,
	/// The quest is not shown in Russia
	EXCLUDE_RUSSIA = 5,
	/// The console quest is first-party
	IN_HOUSE_CONSOLE_QUEST = 6,
	/// The console quest is available on mobile
	MOBILE_CONSOLE_QUEST = 7,
	/// The quest has a start call-to-action
	START_QUEST_CTA = 8,
	/// The quest has reward highlighting
	REWARD_HIGHLIGHTING = 9,
	/// The quest offers fractional rewards
	FRACTIONS_QUEST = 10,
	/// The quest has additional redemption instructions
	ADDITIONAL_REDEMPTION_INSTRUCTIONS = 11,
	/// The quest uses the new pacing system
	PACING_V2 = 12,
	/// The quest presents a survey upon dismissal
	DISMISSAL_SURVEY = 13,
	/// The quest is shown in the mobile quest dock
	MOBILE_QUEST_DOCK = 14,
	/// The quest uses the CDN for assets
	QUESTS_CDN = 15,
	/// The quest uses the pacing controller
	PACING_CONTROLLER = 16,
	/// The quest displays a static image on the Quest Home
	QUEST_HOME_FORCE_STATIC_IMAGE = 17,
	/// The video quest forces HLS video playback
	VIDEO_QUEST_FORCE_HLS_VIDEO = 18,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimedQuest {
	/// The ID of the quest
	pub id: QuestId,
	/// The configuration and metadata for the quest
	pub config: ClaimedQuestConfig,
	/// The user's quest progress
	pub user_status: QuestUserStatus,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimedQuestConfig {
	/// The ID of the quest
	pub id: QuestId,
	/// When the quest period starts
	pub starts_at: Timestamp,
	/// When the quest period ends
	pub expires_at: Timestamp,
	/// The quest features enabled for the quest
	pub features: Vec<QuestFeatures>,
	/// The accent colors for the quest
	pub colors: QuestGradient,
	/// Object that holds the quest's assets
	pub assets: QuestAssets,
	/// Human-readable metadata for the quest
	pub messages: QuestMessages,
	/// The claimed rewards for the quest
	pub rewards: Vec<ClaimedQuestReward>,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimedQuestReward {
	/// The reward's type
	pub r#type: QuestRewardType,
	/// The ID of the SKU awarded
	pub sku_id: SkuId,
	/// The reward's name
	pub name: String,
	/// The article variant of the name (e.g. a Cybernetic Headgear Decoration)
	pub name_with_article: String,
	/// The reward's media asset
	pub asset: String,
	/// The reward's video asset
	pub asset_video: Option<String>,
	/// The amount of Discord Orbs awarded
	pub orb_quantity: Option<u32>,
	/// The collectible product awarded
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collectible_product: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestUserStatus {
	/// The ID of the user
	pub user_id: UserId,
	/// The ID of the quest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quest_id: Option<QuestId>,
	/// When the user accepted the quest
	pub enrolled_at: Option<Timestamp>,
	/// When the user completed the quest
	pub completed_at: Option<Timestamp>,
	/// When the user claimed the quest's reward
	pub claimed_at: Option<Timestamp>,
	/// Which reward tier the user has claimed, if the quest's assignment_method is TIERED
	#[serde(skip_serializing_if = "Option::is_none")]
	pub claimed_tier: Option<Option<u8>>,
	/// When the last heartbeat was received
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_stream_heartbeat_at: Option<Option<Timestamp>>,
	/// Duration (in seconds) the user has streamed the game for since the quest was accepted
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_progress_seconds: Option<Timestamp>,
	/// The content areas the user has dismissed for the quest
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dismissed_quest_content: Option<DismissibleQuestContentFlags>,
	/// The user's progress for each task in the quest, keyed by their event name
	pub progress: HashMap<String, QuestTaskProgress>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestTaskProgress {
	/// The type of task event
	pub event_name: String,
	/// The current task value
	pub value: u32,
	/// When the task was last updated
	pub updated_at: Timestamp,
	/// When the task was completed
	pub completed_at: Option<Timestamp>,
	/// The task's heartbeat data
	#[serde(skip_serializing_if = "Option::is_none")]
	pub heartbeat: Option<Option<QuestTaskHeartbeat>>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestTaskHeartbeat {
	/// When the last heartbeat was received
	pub last_beat_at: Timestamp,
	/// When the task progress expires
	pub expires_at: Option<Timestamp>,
}

bitflags! {
	/// Dismissed [quest content areas](https://docs.discord.food/resources/quests#quest-content-type).
	pub struct DismissibleQuestContentFlags: u64 {
		/// User has dismissed the quest from User Settings
		const GIFT_INVENTORY_SETTINGS_BADGE = 1 << 0;
		/// User has dismissed the quest from the Quest Bar
		const QUEST_BAR = 1 << 1;
		/// User has dismissed the quest from the Active Now page
		const ACTIVITY_PANEL = 1 << 2;
		/// User has dismissed the quest from the stream overlay
		const QUEST_LIVE_STREAM = 1 << 3;
	}
}

#[derive(Serialize, Deserialize)]
pub struct QuestRewardCode {
	/// The ID of the quest
	pub quest_id: QuestId,
	/// The redeem code
	pub code: String,
	/// The platform this redeem code applies to
	pub platform: QuestPlatformType,
	/// The ID of the user who this code belongs to
	pub user_id: UserId,
	/// When the user claimed the quest's reward
	pub claimed_at: Timestamp,
	/// Which reward tier the code belongs to, if the quest's assignment_method is set to TIERED
	pub tier: Option<u8>,
}
