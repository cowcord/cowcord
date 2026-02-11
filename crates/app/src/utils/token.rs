use discord_api::types::token::Token;
use keyring::{Entry, Result};

pub fn save_token(token: &str) -> Result<()> {
	let keyring = Entry::new("cowcord", "auth_token")?;
	keyring.set_password(token)
}

pub fn load_token() -> Result<Option<Token>> {
	let keyring = Entry::new("cowcord", "auth_token")?;
	match keyring.get_password() {
		| Ok(token) => Ok(Some(token.into())),
		| Err(_) => Ok(None),
	}
}
