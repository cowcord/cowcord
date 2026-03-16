use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env, fs};

use discord_api::types::locale::Locale;
use serde::{Deserialize, Serialize};
use tracing::warn;

pub static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
	pub locale: Locale,
}

impl Config {
	pub fn get() -> Result<Self, Box<dyn std::error::Error>> {
		let config_raw = fs::read_to_string(get_config_path())?;

		Ok(serde_json::from_str::<Self>(&config_raw)?)
	}

	pub fn set(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		fs::write(get_config_path(), serde_json::to_string(&self)?)?;
		Ok(())
	}

	/// creates the default config if no config already exists
	pub fn init() -> Result<(), Box<dyn std::error::Error>> {
		let config_path = get_config_path();

		fs::create_dir_all(CONFIG_PATH.get().unwrap())?;
		let mut file = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.open(&config_path)?;

		if file.metadata()?.len() == 0 {
			file.write_all(serde_json::to_string(&Config::default())?.as_bytes())?;
		}

		Ok(())
	}
}

pub fn get_config_dir() -> PathBuf {
	#[cfg(any(
		target_os = "linux",
		target_os = "freebsd",
		target_os = "openbsd",
		target_os = "netbsd"
	))]
	{
		let xdg_conf_dir = env::var("XDG_CONFIG_DIR").ok().map(PathBuf::from);
		let conf_dir = xdg_conf_dir.unwrap_or_else(|| {
			warn!("$XDG_CONFIG_DIR not set! defaulting to $HOME/.config");
			env::home_dir()
				.expect(
					"Could not determine linux/bsd config dir! try setting $XDG_CONFIG_DIR or $HOME env vars",
				)
				.join(".config")
		});

		return conf_dir.join("cowcord");
	}

	#[cfg(target_os = "macos")]
	{
		use std::env::home_dir;

		return home_dir()
			.expect("Could not find macos home dir!")
			.join("Library/Application Support/cowcord");
	}

	#[cfg(target_os = "windows")]
	{
		let appdata_dir = env::var("APPDATA")
			.expect("Somehow got an error while reading %APPDATA%, i blame microslop");

		return PathBuf::from(appdata_dir).join("cowcord");
	}
}

/// # Panics
/// If `CONFIG_PATH` has not been initialized
#[inline(always)]
pub fn get_config_path() -> PathBuf {
	CONFIG_PATH.get().unwrap().join("settings.json")
}
