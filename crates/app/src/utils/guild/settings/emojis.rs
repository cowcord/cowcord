use std::cmp::max;

use discord_types::api::guild::{GuildFeatures, PremiumTier};

pub fn get_emoji_slots(
	premium_tier: PremiumTier,
	guild_features: Vec<GuildFeatures>,
) -> (u8, u8, u8) {
	let normal_emojis: u8 = max(
		50 * (premium_tier + 1),
		if guild_features.contains(GuildFeatures::MORE_EMOJI) {
			200
		} else {
			if premium_tier == PremiumTier::TIER_3 {
				250
			} else {
				50
			}
		},
	);

	// currently the math for animated emojis is the exact same as normal emojis
	let animated_emojis = normal_emojis;

	let premium_emojis: u8 = 25;

	return (normal_emojis, animated_emojis, premium_emojis);
}
