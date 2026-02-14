#![allow(non_snake_case)]

#[cfg(target_os = "macos")]
use apple_native_keyring_store::protected::Store;
#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "linux"))]
use dbus_secret_service_keyring_store::Store;
use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
#[cfg(target_os = "windows")]
use windows_native_keyring_store::Store;

pub mod components;
pub mod utils;
pub mod ws;

mod views;
use channels::me::Me;
use views::*;

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

fn main() {
	// init keyring store
	keyring_core::set_default_store(Store::new().unwrap());

	// init logging
	// todo: use cli arg for determining log level, but still default to info
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();
	tracing::subscriber::set_global_default(subscriber).unwrap();

	let config = dioxus::desktop::Config::new()
		.with_window(
			WindowBuilder::new()
				.with_maximized(true)
				.with_title("cowcord"), // .with_decorations(false), // .with_transparent(true),
		)
		.with_menu(None);

	// #[cfg(debug_assertions)]
	// config = config.with_disable_context_menu(true);

	LaunchBuilder::desktop().with_cfg(config).launch(|| {
		rsx! {
			Stylesheet { href: TAILWIND }
			Router::<Route> {}
		}
	});
}
