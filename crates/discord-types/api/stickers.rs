use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::api_types::users::PartialUser;

#[derive(Serialize, Deserialize)]
pub struct StickerPack {
	/// The ID of the sticker pack
	pub id: Snowflake,
	/// The stickers in the pack
	pub stickers: Vec<Sticker>,
	/// The name of the sticker pack
	pub name: String,
	/// The ID of the pack's SKU
	pub sku_id: Snowflake,
	/// The ID of a sticker in the pack which is shown as the pack's icon
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cover_sticker_id: Option<Snowflake>,
	/// The description for the sticker pack
	pub description: String,
	/// The ID of the sticker pack's banner image
	#[serde(skip_serializing_if = "Option::is_none")]
	pub banner_asset_id: Option<Snowflake>,
}

#[derive(Serialize, Deserialize)]
pub struct Sticker {
	/// The ID of the sticker
	pub id: Snowflake,
	/// For standard stickers, ID of the pack the sticker is from
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pack_id: Option<Snowflake>,
	/// The name of the sticker (2-30 characters)
	pub name: String,
	/// The description for the sticker (max 100 characters)
	pub description: Option<String>,
	/// Autocomplete/suggestion tags for the sticker (1-200 characters)
    /// 
    /// comma seperated list
	pub tags: String,
	/// The type of sticker
	pub r#type: StickerType,
	/// The type of format for the sticker
	pub format_type: StickerFormatTypes,
	/// Whether this guild sticker can be used; may be false due to loss of premium subscriptions (boosts)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available: Option<bool>,
	/// The ID of the guild the sticker is attached to
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id: Option<Snowflake>,
	/// The user that uploaded the guild sticker
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// The standard sticker's sort order within its pack
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_value: Option<i64>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StickerType {
	/// An official sticker in a current or legacy purchasable pack
	STANDARD = 1,
	/// A sticker uploaded to a guild for the guild's members
	GUILD = 2,
}

/// GIF stickers are not available through the [CDN](https://docs.discord.food/reference#cdn-formatting), and must be accessed at https://media.discordapp.net/stickers/{sticker_id}.gif.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StickerFormatTypes {
	/// A PNG image
	PNG = 1,
	/// An animated PNG image, using the APNG format
	APNG = 2,
	/// A lottie animation; requires the VERIFIED and/or PARTNERED  guild feature
	LOTTIE = 3,
	/// An animated GIF image
    /// 
    /// not available through the CDN, and must be accessed at https://media.discordapp.net/stickers/{sticker_id}.gif
	GIF = 4,
}

#[derive(Serialize, Deserialize)]
pub struct StickerItem {
	/// The ID of the sticker
	pub id: Snowflake,
	/// The name of the sticker
	pub name: String,
	/// The type of format for the sticker
	pub format_type: StickerFormatTypes,
}

