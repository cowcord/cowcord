use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{api::{integrations::IntegrationGuild, users::PartialUser}, common::id::{ApplicationId, ChannelId, GuildId, WebhookId}};

#[derive(Serialize, Deserialize)]
pub struct Webhook {
	/// The ID of the webhook
	pub id: WebhookId,
	/// The type of webhook
	pub r#type: WebhookType,
	/// The guild ID this webhook is for, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Option<GuildId>>,
	/// The channel ID this webhook is for, if any
	pub channel_id: Option<ChannelId>,
	/// The user this webhook was created by
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<Option<PartialUser>>,
	/// The default name of the webhook (1-80 characters)
	pub name: Option<String>,
	/// The default avatar hash of the webhook
	pub avatar: Option<String>,
	/// The secure token of the webhook (returned for INCOMING webhooks)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/// The application that created this webhook
	pub application_id: Option<ApplicationId>,
	/// The guild of the channel that this webhook is following (returned for CHANNEL_FOLLOWER webhooks)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source_guild: Option<IntegrationGuild>,
	/// The channel that this webhook is following (returned for CHANNEL_FOLLOWER webhooks)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source_channel: Option<WebhookChannel>,
	/// The URL used for executing the webhook (returned for INCOMING webhooks)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum WebhookType {
	/// Incoming webhooks can post messages to channels with a generated token
	INCOMING = 1,
	/// Channel Follower webhooks are internal webhooks used to post new messages into channels
	CHANNEL_FOLLOWER = 2,
	/// Application webhooks are webhooks used with interactions
	APPLICATION = 3,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookChannel {
	/// The ID of the channel
	pub id: ChannelId,
	/// The name of the channel (1-100 characters)
	pub name: String,
}

