#![allow(non_snake_case)]

use std::sync::OnceLock;

#[cfg(target_os = "macos")]
use apple_native_keyring_store::protected::Store;
use cowcord_config::{CONFIG_PATH, get_config_dir};
#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "linux"))]
use dbus_secret_service_keyring_store::Store;
use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
#[cfg(target_os = "windows")]
use windows_native_keyring_store::Store;

pub mod components;
pub mod utils;
pub mod ws;

pub mod cli;
mod views;
use channels::me::Me;
use views::*;

use crate::utils::fingerprint::{FINGERPRINT, get_fingerprint};

const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route {
    #[route("/")]
    Home {},

	#[route("/login")]
	Login {},

	#[route("/channels/@me")]
	Me {},

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub static CONFIG: OnceLock<cowcord_config::Config> = OnceLock::new();

fn main() {
	// init logging
	let (filter, filter_handle) = tracing_subscriber::reload::Layer::new(LevelFilter::WARN);
	tracing_subscriber::registry()
		.with(filter)
		.with(tracing_subscriber::fmt::layer())
		.init();

	if let Some(args) = cli::parse_args() {
		// init keyring store
		keyring_core::set_default_store(Store::new().unwrap());

		// update log level if cli args set a new one
		if let Some(log_level) = args.log_level {
			filter_handle.modify(|f| *f = log_level).unwrap();
		}

		// init dioxus config
		let config = dioxus::desktop::Config::new()
			.with_window(
				WindowBuilder::new()
					.with_maximized(true)
					.with_title("cowcord"), // .with_decorations(false), // .with_transparent(true),
			)
			.with_menu(None);

		// init rustls ring
		rustls::crypto::ring::default_provider()
			.install_default()
			.unwrap();

		// init cowcord config
		CONFIG_PATH
			.set(args.config_dir.unwrap_or(get_config_dir()))
			.unwrap();
		cowcord_config::Config::init().unwrap();
		CONFIG.set(cowcord_config::Config::get().unwrap()).unwrap();

		LaunchBuilder::desktop().with_cfg(config).launch(App);
	}
}

#[component]
fn App() -> Element {
	// init fingerprint
	use_effect(move || {
		spawn(async move {
			FINGERPRINT.set(get_fingerprint().await.unwrap()).unwrap();
		});
	});

	rsx! {
		Stylesheet { href: TAILWIND }
		Router::<Route> {}
	}
}
