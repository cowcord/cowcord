use discord_api::types::token::Token;
use keyring_core::{Entry, Result};

pub fn save_token(token: &str) -> Result<()> {
	Entry::new("cowcord", "token")?.set_password(token)
}

pub fn load_token() -> Result<Option<Token>> {
	let keyring = Entry::new("cowcord", "token")?;
	match keyring.get_password() {
		| Ok(token) => Ok(Some(token.into())),
		| Err(_) => Ok(None),
	}
}

pub fn delete_token() -> Result<()> {
	Entry::new("cowcord", "token")?.delete_credential()
}
