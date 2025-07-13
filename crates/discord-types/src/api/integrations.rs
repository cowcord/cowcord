use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::application::{ApplicationRoleConnectionMetadata, ApplicationSku, ApplicationType};
use crate::api::users::PartialUser;
use crate::common::id::{ApplicationId, GuildId, IntegrationId, RoleId, SkuId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct Integration {
	/// The ID of the integration
	pub id: IntegrationId,
	/// The name of the integration
	pub name: String,
	/// The type of integration
	pub r#type: IntegrationType,
	/// Whether this integration is enabled
	pub enabled: bool,
	/// Integration account information
	pub account: IntegrationAccount,
	/// Whether this integration is syncing
	#[serde(skip_serializing_if = "Option::is_none")]
	pub syncing: Option<bool>,
	/// Role ID that this integration uses for subscribers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_id: Option<RoleId>,
	/// Whether emoticons should be synced for this integration (Twitch only)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enable_emoticons: Option<bool>,
	/// The behavior of expiring subscribers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expire_behavior: Option<IntegrationExpireBehavior>,
	/// The grace period before expiring subscribers (one of 1, 3, 7, 14, 30, in days)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expire_grace_period: Option<NonZeroU8>,
	/// When this integration was last synced
	#[serde(skip_serializing_if = "Option::is_none")]
	pub synced_at: Option<Timestamp>,
	/// How many subscribers this integration has
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscriber_count: Option<u32>,
	/// Whether this integration has been revoked
	#[serde(skip_serializing_if = "Option::is_none")]
	pub revoked: Option<bool>,
	/// The integrated OAuth2 application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub application: Option<IntegrationApplication>,
	/// The scopes the application has been authorized with
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scopes: Option<Vec<String>>,
	/// The metadata that the application has set for role connections
	pub role_connections_metadata: Vec<ApplicationRoleConnectionMetadata>,
	/// The user that added this integration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IntegrationType {
	#[serde(rename = "twitch")]
	Twitch,
	#[serde(rename = "youtube")]
	YouTube,
	#[serde(rename = "discord")]
	Discord,
	#[serde(rename = "guild_subscription")]
	Internal,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
	/// Remove the subscriber role from the user on expiration
	REMOVE_ROLE = 0,
	/// Remove the user from the guild on expiration
	KICK = 1,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrationAccount {
	/// The ID of the account
	pub id: String,
	/// The name of the account
	pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrationApplication {
	/// The ID of the application
	pub id: ApplicationId,
	/// The name of the application
	pub name: String,
	/// The description of the application
	pub description: String,
	/// The application's icon hash
	pub icon: Option<String>,
	/// The application's default rich presence invite cover image hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_image: Option<String>,
	/// The application's splash hash
	#[serde(skip_serializing_if = "Option::is_none")]
	pub splash: Option<String>,
	/// The type of the application , if any
	pub r#type: Option<ApplicationType>,
	/// The ID of the application's primary SKU (game, application subscription, etc.)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_sku_id: Option<SkuId>,
	/// The bot attached to this application
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<PartialUser>,
	/// The URL used for deep linking during OAuth2 authorization on mobile devices
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deeplink_uri: Option<Option<String>>,
	/// The third party SKUs of the application's game
	#[serde(skip_serializing_if = "Option::is_none")]
	pub third_party_skus: Option<Vec<ApplicationSku>>,
	/// The role connection verification entry point of the integration; when configured, this will render the application as a verification method in guild role verification configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role_connections_verification_url: Option<Option<String>>,
	/// Whether the application is verified
	pub is_verified: bool,
	/// Whether the application is discoverable in the application directory
	pub is_discoverable: bool,
	/// Whether the application has monetization enabled
	pub is_monetized: bool,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrationGuild {
	/// The ID of the guild
	pub id: GuildId,
	/// The name of the guild (2-100 characters)
	pub name: String,
	/// The guild's icon hash
	pub icon: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Gif {
	/// The ID of the GIF
	pub id: String,
	/// The title of the GIF
	#[deprecated]
	pub title: String,
	/// The provider source URL of the GIF
	pub url: String,
	/// The media URL of the GIF in the requested format
	pub src: String,
	/// The media URL of the GIF in GIF format
	pub gif_src: String,
	/// A preview image of the GIF
	pub preview: String,
	/// Width of image
	pub width: u16,
	/// Height of image
	pub height: u16,
}

#[derive(Serialize, Deserialize)]
pub enum GifMediaFormat {
	/// MP4 video
	mp4,
	/// MP4 video in a smaller size
	tinymp4,
	/// MP4 video in a very small size
	nanomp4,
	/// MP4 video that loops (same as mp4 )
	loopedmp4,
	/// WebM video
	webm,
	/// WebM video in a smaller size
	tinywebm,
	/// WebM video in a very small size
	nanowebm,
	/// GIF image
	gif,
	/// GIF image in a medium size
	mediumgif,
	/// GIF image in a smaller size
	tinygif,
	/// GIF image in a very small size
	nanogif,
}
