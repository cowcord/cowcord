#![allow(non_camel_case_types)]

// TODO: jsut make some build script that autogenerates this file

pub mod application;
pub mod audit_log;
pub mod auto_moderation;
pub mod channel;
pub mod discovery;
pub mod emoji;
pub mod family_center;
pub mod guild;
pub mod guild_scheduled_event;
pub mod components;
pub mod connected_accounts;
pub mod directory_entries;
pub mod entitlements;
pub mod integrations;
pub mod invites;
pub mod lobbies;
pub mod messages;
pub mod premium_referral;
pub mod presences;
pub mod quests;
pub mod relationships;
pub mod soundboard;
pub mod stage_instances;
pub mod stickers;
pub mod teams;
pub mod users;
pub mod voice;
pub mod webhooks;
pub mod websocket;

pub mod endpoints;

pub enum API_VERSION {
	v10,
	v9,
	#[deprecated]
	v8,
	#[deprecated]
	v7,
	#[deprecated]
	v6,
}

pub const DISCORD_URL: &str = "https://discord.com/";
