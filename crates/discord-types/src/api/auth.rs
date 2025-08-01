use serde::{Deserialize, Serialize};

use crate::api::users::Theme;
use crate::common::locale::Locale;

/// Where a login is initiated from outside of the normal login flow.
#[derive(Serialize, Deserialize)]
pub enum LoginSource {
	/// Login request initiated from a gift code
	gift,
	/// Login request initiated from a guild template
	guild_template,
	/// Login request initiated from a guild invite
	guild_invite,
	/// Login request initiated from a group DM invite
	dm_invite,
	/// Login request initiated from a friend invite
	friend_invite,
	/// Login request initiated from a role subscription redirect
	role_subscription,
	/// Login request initiated from a role subscription settings redirect
	role_subscription_setting,
}

/// A partial settings object to bootstrap the client with.
#[derive(Serialize, Deserialize)]
pub struct LoginSettings {
	/// The language option chosen by the user
	pub locale: Locale,
	/// The client theme selected by the user (only dark or light are returned)
	pub theme: Theme,
}

/// Actions the user must complete after a successful login.
#[derive(Serialize, Deserialize)]
pub enum LoginRequiredActionType {
	/// The user must change their password to meet Discord's new password requirements
	update_password,
}
